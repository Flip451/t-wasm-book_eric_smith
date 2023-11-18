use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod sierpinski;
use sierpinski::*;

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

    wasm_bindgen_futures::spawn_local(async move {
        // 送受信機の作成
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<()>();

        // ImageHtmlElement の作成と画像の読み込み開始
        let image = web_sys::HtmlImageElement::new().unwrap();
        image.set_src("Idle (1).png");

        // 画像の読み込みが完了したことを通知するコールバック関数の作成
        let callback = Closure::once(|| {
            success_tx.send(()).unwrap();
        });
        // 画像の読み込みが完了したら上記のコールバック関数を呼び出すように設定
        image.set_onload(Some(callback.as_ref().unchecked_ref()));

        // 画像の読み込み完了を待機
        success_rx.await.unwrap();

        // 画像の描画
        context.draw_image_with_html_image_element(&image, 0., 0.).unwrap();

        sierpinski::draw_sierpinski(
            &context,
            &Triangle {
                p1: Point { x: 300.0, y: 0. },
                p2: Point { x: 0., y: 600.0 },
                p3: Point { x: 600.0, y: 600.0 },
            },
            6,
            &Color::random_color(),
        );
    });

    Ok(())
}
