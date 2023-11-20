use anyhow::{anyhow, Result};
use futures::Future;
use wasm_bindgen::{
    closure::{Closure, IntoWasmClosure, WasmClosureFnOnce},
    JsCast, JsValue,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement, Response, Window,
};

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or(anyhow!("no global `window` exists"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or(anyhow!("should have a document on window"))
}

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

pub fn now() -> Result<f64> {
    Ok(window()?
        .performance()
        .ok_or(anyhow!("No performance object found on window"))?
        .now())
}

pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
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

pub fn new_image() -> Result<HtmlImageElement> {
    HtmlImageElement::new()
        .map_err(|js_value| anyhow!("Error creating HTMLImageElement {:#?}", js_value))
}

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}

pub type LoopClosure<T> = Closure<dyn FnMut(T)>;
pub fn request_animation_frame(callback: &LoopClosure<f64>) -> Result<i32> {
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|js_value| anyhow!("Error requesting animation frame {:#?}", js_value))
}

pub fn create_raf_closure<F, T>(f: F) -> LoopClosure<T>
where
    F: FnMut(T) + 'static + IntoWasmClosure<dyn (FnMut(T) -> ())>,
    T: wasm_bindgen::convert::FromWasmAbi + 'static,
{
    Closure::new(f)
}
