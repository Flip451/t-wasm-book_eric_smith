use anyhow::{anyhow, Result};
use futures::Future;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

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

pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}
