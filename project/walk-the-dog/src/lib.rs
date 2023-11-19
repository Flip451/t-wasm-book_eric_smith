use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod sierpinski;

#[macro_use]
mod browser;

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

    // canvas要素の取得
    let canvas = browser::canvas().expect("canvas not found");

    // コンテキストの取得
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    wasm_bindgen_futures::spawn_local(async move {
        let json = fetch_json("rhb.json")
            .await
            .expect("Could not fetch rhb.json");

        // json を Sheet 型に変換
        // この際、JsValue 型の json を serde を用いてデシリアライズ
        // この部分の実装は
        // <https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json>
        // を参考にした
        let sheet: Sheet = json.into_serde().expect("Could not parse rhb.json");

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
        image.set_src("rhb.png");

        // 画像の読み込み完了を待機
        success_rx.await;

        let mut frame = -1;

        // 定期実行するコールバック関数の作成
        let interval_callback = Closure::wrap(Box::new(move || {
            // フレームカウントを 0 - 7 の間でループ
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);

            // シートの中から指定の画像（Run (*).png）の位置を取得
            let sprite = sheet.frames.get(&frame_name).expect("Cell not found");

            context.clear_rect(0., 0., 600., 600.);

            // キャンバスに指定の画像を描画
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                sprite.frame.x as f64,
                sprite.frame.y as f64,
                sprite.frame.w as f64,
                sprite.frame.h as f64,
                300.,
                300.,
                sprite.frame.w as f64,
                sprite.frame.h as f64,
            );
        }) as Box<dyn FnMut()>);

        // 毎秒 20 フレームで実行するように設定
        browser::window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
            interval_callback.as_ref().unchecked_ref(),
            50,
        );

        // コールバック関数の解放
        interval_callback.forget();
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
