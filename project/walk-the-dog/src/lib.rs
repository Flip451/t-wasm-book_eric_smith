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

mod engine;

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

    // コンテキストの取得
    let canvas_context = browser::context().expect("Getting canvas context failed");

    browser::spawn_local(async move {
        let json = browser::fetch_json("rhb.json")
            .await
            .expect("Could not fetch rhb.json");

        // json を Sheet 型に変換
        // この際、JsValue 型の json を serde を用いてデシリアライズ
        // この部分の実装は
        // <https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json>
        // を参考にした
        let sheet: Sheet = json.into_serde().expect("Could not parse rhb.json");

        let image = engine::load_image("rhb.png")
            .await
            .expect("Could not load rhb.png");

        let mut frame = -1;

        // 定期実行するコールバック関数の作成
        let interval_callback = Closure::wrap(Box::new(move || {
            // フレームカウントを 0 - 7 の間でループ
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);

            // シートの中から指定の画像（Run (*).png）の位置を取得
            let sprite = sheet.frames.get(&frame_name).expect("Cell not found");

            canvas_context.clear_rect(0., 0., 600., 600.);

            // キャンバスに指定の画像を描画
            canvas_context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
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
