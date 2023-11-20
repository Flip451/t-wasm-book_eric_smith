use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
use web_sys::HtmlImageElement;

use crate::{
    browser,
    engine::{self, Game, Rect, Renderer},
};

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
}

impl WalkTheDog {
    pub fn new() -> Self {
        Self {
            image: None,
            sheet: None,
            frame: 0,
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let json = browser::fetch_json("rhb.json").await?;

        // json を Sheet 型に変換
        // この際、JsValue 型の json を serde を用いてデシリアライズ
        // この部分の実装は
        // <https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json>
        // を参考にした
        let sheet: Sheet = json.into_serde()?;

        let image = engine::load_image("rhb.png").await?;

        Ok(Box::new(WalkTheDog {
            image: Some(image),
            sheet: Some(sheet),
            frame: self.frame,
        }))
    }

    fn update(&mut self) {
        // rhb の動作が一巡するのには 24 フレームかかる
        self.frame = (self.frame + 1) % 24;
    }

    fn draw(&self, renderer: &Renderer) {
        let current_sprite = self.frame / 3 + 1;
        let frame_name = format!("Run ({}).png", current_sprite);
        // シートの中から指定の画像（Run (*).png）の位置を取得
        let sprite = self
            .sheet
            .as_ref()
            .expect("Sheet not found")
            .frames
            .get(&frame_name)
            .expect("Cell not found");
        renderer.clear(&Rect {
            x: 0.,
            y: 0.,
            w: 600.,
            h: 600.,
        });

        // キャンバスに指定の画像を描画
        renderer.draw_image(
            self.image.as_ref().expect("Image not found"),
            &Rect {
                x: sprite.frame.x as f32,
                y: sprite.frame.y as f32,
                w: sprite.frame.w as f32,
                h: sprite.frame.h as f32,
            },
            &Rect {
                x: 300.,
                y: 300.,
                w: sprite.frame.w as f32,
                h: sprite.frame.h as f32,
            },
        );
    }
}
