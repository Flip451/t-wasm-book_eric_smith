use anyhow::{anyhow, Result};
use web_sys::{Window, Document};

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or(anyhow!("no global `window` exists"))
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or(anyhow!("should have a document on window"))
}