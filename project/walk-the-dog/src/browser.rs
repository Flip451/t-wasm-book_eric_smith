use anyhow::{anyhow, Result};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub use self::animation_frame::*;
pub use self::async_wrapper::*;
pub use self::canvas::*;
pub use self::closure::*;
pub use self::elements::*;
pub use self::json::*;
pub use self::utils::*;
use self::window::*;

pub mod window {
    use super::*;
    use web_sys::{Document, Window};

    pub(super) fn window() -> Result<Window> {
        web_sys::window().ok_or(anyhow!("no global `window` exists"))
    }

    pub(super) fn document() -> Result<Document> {
        window()?
            .document()
            .ok_or(anyhow!("should have a document on window"))
    }
}

#[macro_use]
pub mod utils {
    use super::*;

    macro_rules! log {
        ($($t:tt)*) => {
            web_sys::console::log_1(&format!($($t)*).into());
        };
    }

    pub fn now() -> Result<f64> {
        Ok(window()?
            .performance()
            .ok_or(anyhow!("No performance object found on window"))?
            .now())
    }
}

pub mod canvas {
    use super::*;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

    pub fn canvas() -> Result<HtmlCanvasElement> {
        document()?
            .get_element_by_id("canvas")
            .ok_or(anyhow!("No canvas element found with ID 'canvas'"))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
    }

    pub fn context() -> Result<CanvasRenderingContext2d> {
        canvas()?
            .get_context("2d")
            .map_err(|js_value| anyhow!("Error getting 2d context {:#?} on canvas", js_value))?
            .ok_or(anyhow!("should have a 2d context on canvas"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|context| {
                anyhow!(
                    "Error converting {:#?} to CanvasRenderingContext2d",
                    context
                )
            })
    }
}

pub mod async_wrapper {
    use futures::Future;

    pub fn spawn_local<F>(future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(future);
    }
}

pub mod json {
    use super::*;
    use web_sys::Response;

    async fn fetch_with_str(resource: &str) -> Result<JsValue> {
        // js の window.fetch を呼び出す
        // js の window.fetch は Promise を返すので、
        // JsFuture::from を使って Future に変換する (Furure は Rust における非同期処理を表す)
        JsFuture::from(window()?.fetch_with_str(resource))
            .await
            .map_err(|js_value| anyhow!("Error fetching {:#?} from window", js_value))
    }

    pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
        let resp_value = fetch_with_str(json_path).await?;

        // レスポンス(JsValue) を Response オブジェクトにキャスト
        let resp: Response = resp_value
            .dyn_into()
            .map_err(|js_value| anyhow!("Error casting {:#?} to Response", js_value))?;

        // js の Response.json() を呼び出す
        // js の Response.json() は Promise を返すので、
        // JsFuture::from を使って Future に変換する
        JsFuture::from(
            resp.json()
                .map_err(|js_value| anyhow!("Error getting json from response {:#?}", js_value))?,
        )
        .await
        .map_err(|js_value| anyhow!("Error getting json from response {:#?}", js_value))
    }
}

pub mod elements {
    use super::*;
    use web_sys::HtmlImageElement;

    pub fn new_image() -> Result<HtmlImageElement> {
        HtmlImageElement::new()
            .map_err(|js_value| anyhow!("Error creating HTMLImageElement {:#?}", js_value))
    }
}

pub mod closure {
    use wasm_bindgen::closure::{Closure, IntoWasmClosure, WasmClosureFnOnce};

    pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
    where
        F: 'static + WasmClosureFnOnce<A, R>,
    {
        Closure::once(fn_once)
    }

    pub type WasmClosure<A, R> = Closure<dyn FnMut(A) -> R + 'static>;

    pub fn create_wasm_closure<F, A, R>(f: F) -> WasmClosure<A, R>
    where
        F: IntoWasmClosure<dyn (FnMut(A) -> R)> + 'static,
        A: wasm_bindgen::convert::FromWasmAbi + 'static,
        R: wasm_bindgen::convert::IntoWasmAbi + 'static,
    {
        Closure::new(f)
    }
}

pub mod animation_frame {
    use super::*;

    pub type LoopClosure = WasmClosure<f64, ()>;

    pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32> {
        window()?
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .map_err(|js_value| anyhow!("Error requesting animation frame {:#?}", js_value))
    }
}
