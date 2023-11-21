use anyhow::Result;
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;

use crate::{
    browser,
    engine::{self, Game, Rect, Renderer},
};

use self::{rhb::RedHatBoy, sprite::SpriteSheet};

mod rhb;
mod sprite;

pub struct WalkTheDog {
    // image: Option<HtmlImageElement>,
    // sheet: Option<SpriteSheet>,
    rhb: Option<RedHatBoy>,
    // frame: u8,
    // position: Point,
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl WalkTheDog {
    pub fn new() -> Self {
        Self {
            rhb: None,
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
            rhb: Some(RedHatBoy::new(sheet, image)),
        }))
    }

    fn update(&mut self, keystate: &engine::KeyState) {
        self.rhb.as_mut().expect("RedHatBoy not found").update();

        if keystate.is_pressed("ArrowRight") {
            self.rhb.as_mut().expect("RedHatBoy not found").run_right();
        }

        if keystate.is_pressed("ArrowLeft") {
            self.rhb.as_mut().expect("RedHatBoy not found").run_left();
        }

        if keystate.is_pressed("ArrowDown") {
            self.rhb.as_mut().expect("RedHatBoy not found").slide();
        }
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0.,
            y: 0.,
            w: 600.,
            h: 600.,
        });

        self.rhb
            .as_ref()
            .expect("RedHatBoy not found")
            .draw(renderer);
    }
}
