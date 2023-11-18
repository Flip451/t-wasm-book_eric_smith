use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
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
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = success_tx.clone();

        // ImageHtmlElement の作成
        let image = web_sys::HtmlImageElement::new().unwrap();

        // 画像の読み込みが完了したことを通知するコールバック関数の作成
        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok().and_then(|mut tx| tx.take()) {
                success_tx.send(Ok(()));
            }
        });
        // 画像の読み込みが完了したら上記のコールバック関数を呼び出すように設定
        image.set_onload(Some(callback.as_ref().unchecked_ref()));

        // 画像の読み込みが失敗したことを通知するコールバック関数の作成
        let callback_error = Closure::once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok().and_then(|mut tx| tx.take()) {
                error_tx.send(Err(err));
            }
        });
        // 画像の読み込みが失敗したら上記のコールバック関数を呼び出すように設定
        image.set_onerror(Some(callback_error.as_ref().unchecked_ref()));

        // 画像の読み込み開始
        image.set_src("Idle (1).png");

        // 画像の読み込み完了を待機
        success_rx.await;

        // 画像の描画
        context.draw_image_with_html_image_element(&image, 0., 0.);

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

        let json = fetch_json("rhb.json").await.expect("Could not fetch rhb.json");

        // json を Sheet 型に変換
        // この際、JsValue 型の json を serde を用いてデシリアライズ
        // この部分の実装は
        // <https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json>
        // を参考にした
        let sheet: Sheet = json.into_serde().expect("Could not parse rhb.json");
    });

    Ok(())
}

async fn fetch_json(json_path: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");

    // js の window.fetch を呼び出す
    // js の window.fetch は Promise を返すので、
    // JsFuture::from を使って Future に変換する (Furure は Rust における非同期処理を表す)
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(json_path)).await?;

    // レスポンス(JsValue) を Response オブジェクトにキャスト
    let resp: web_sys::Response = resp_value.dyn_into()?;

    // js の Response.json() を呼び出す
    // js の Response.json() は Promise を返すので、
    // JsFuture::from を使って Future に変換する
    wasm_bindgen_futures::JsFuture::from(resp.json()?).await
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}