use std::{rc::Rc, sync::Mutex};

use anyhow::{anyhow, Result};
use futures::channel::oneshot;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlImageElement;

use crate::browser;

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    // ImageHtmlElement の作成
    let image = browser::new_image()?;

    // 送受信機の作成
    let (complete_tx, complete_rx) = oneshot::channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);

    // 画像の読み込みが完了したことを通知するコールバック関数の作成
    let success_callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut tx| tx.take()) {
            success_tx.send(Ok(()));
        }
    });
    // 画像の読み込みが完了したら上記のコールバック関数を呼び出すように設定
    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));

    // 画像の読み込みが失敗したことを通知するコールバック関数の作成
    let error_callback = browser::closure_once::<_, JsValue, ()>(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut tx| tx.take()) {
            error_tx.send(Err(anyhow!("Error loading image: {:#?}", err)));
        }
    });
    // 画像の読み込みが失敗したら上記のコールバック関数を呼び出すように設定
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

    // 画像の読み込み開始
    image.set_src("rhb.png");

    // 画像の読み込み完了を待機
    complete_rx.await??;

    Ok(image)
}
