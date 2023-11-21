use anyhow::{anyhow, Result};
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;

use crate::{
    browser,
    engine::{self, Game, Rect, Renderer},
};

use self::{rhb::RedHatBoy, sprite::SpriteSheet};

mod rhb;
mod sprite;

pub enum WalkTheDog {
    Loading,
    Loaded(RedHatBoy),
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl WalkTheDog {
    pub fn new() -> Self {
        Self::Loading
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            Self::Loading => {
                let json = browser::fetch_json("rhb.json").await?;

                // json を Sheet 型に変換
                // この際、JsValue 型の json を serde を用いてデシリアライズ
                // この部分の実装は
                // <https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html#an-alternative-approach---using-json>
                // を参考にした
                let sheet: SpriteSheet = json.into_serde()?;

                let image = engine::load_image("rhb.png").await?;
                let rhb = RedHatBoy::new(sheet, image);

                Ok(Box::new(WalkTheDog::Loaded(rhb)))
            }
            Self::Loaded(_) => Err(anyhow!("Error: Game is already initialized")),
        }
    }

    fn update(&mut self, keystate: &engine::KeyState) {
        match self {
            Self::Loading => {}
            Self::Loaded(rhb) => {
                rhb.update();

                if keystate.is_pressed("ArrowRight") {
                    rhb.run_right();
                }

                if keystate.is_pressed("ArrowLeft") {
                    rhb.run_left();
                }

                if keystate.is_pressed("ArrowDown") {
                    rhb.slide();
                }
            }
        }
    }

    fn draw(&self, renderer: &Renderer) {
        match self {
            WalkTheDog::Loading => {},
            WalkTheDog::Loaded(rhb) => {
                renderer.clear(&Rect {
                    x: 0.,
                    y: 0.,
                    w: 600.,
                    h: 600.,
                });

                rhb.draw(renderer);
            }
        }
    }
}
