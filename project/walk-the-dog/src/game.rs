use anyhow::Result;
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;
use web_sys::HtmlImageElement;

use crate::{
    browser,
    engine::{self, Game, Rect, Renderer},
};

use self::sprite::SpriteSheet;

mod rhb;
mod sprite;

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<SpriteSheet>,
    frame: u8,
    position: Point,
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl WalkTheDog {
    pub fn new() -> Self {
        Self {
            image: None,
            sheet: None,
            frame: 0,
            position: Point { x: 300., y: 300. },
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
        let sheet: SpriteSheet = json.into_serde()?;

        let image = engine::load_image("rhb.png").await?;

        Ok(Box::new(WalkTheDog {
            image: Some(image),
            sheet: Some(sheet),
            frame: self.frame,
            position: self.position,
        }))
    }

    fn update(&mut self, keystate: &engine::KeyState) {
        // rhb の動作が一巡するのには 24 フレームかかる
        self.frame = (self.frame + 1) % 24;

        // rhb の位置を更新
        let mut velocity = Point { x: 0., y: 0. };
        if keystate.is_pressed("ArrowDown") {
            velocity.y += 3.;
        }
        if keystate.is_pressed("ArrowUp") {
            velocity.y -= 3.;
        }
        if keystate.is_pressed("ArrowLeft") {
            velocity.x -= 3.;
        }
        if keystate.is_pressed("ArrowRight") {
            velocity.x += 3.;
        }

        self.position.x += velocity.x;
        self.position.y += velocity.y;
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
            &sprite.to_rect_on_sheet(),
            &sprite.to_rect_on_canvas(self.position.x, self.position.y),
        );
    }
}
