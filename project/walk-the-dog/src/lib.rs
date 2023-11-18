use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // windowオブジェクトの取得
    let window = web_sys::window().expect("no global `window` exists");
    // documentオブジェクトの取得
    let document = window.document().expect("should have a document on window");
    // canvas要素の取得 (Element型)
    let canvas = document.get_element_by_id("canvas").unwrap();
    // Element 型の canvas を HtmlCanvasElement にキャスト
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // コンテキストの取得
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    context.move_to(300., 0.);
    context.begin_path();
    context.line_to(0., 600.);
    context.line_to(600., 600.);
    context.line_to(300., 0.);
    context.close_path();
    context.stroke();
    context.fill();

    Ok(())
}
