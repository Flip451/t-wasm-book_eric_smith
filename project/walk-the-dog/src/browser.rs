use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, Window};

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
