use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::engine::{
    key_state::KeyState,
    renderer::{Rect, Renderer},
    Game,
};

use self::{background::Background, rhb::RedHatBoy, objects::GameObject};

mod background;
mod objects;
mod rhb;
mod sprite;

use objects::stone::Stone;

pub enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

pub struct Walk {
    rhb: RedHatBoy,
    background: Background,
    stone: Stone,
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
                let rhb = RedHatBoy::new().await?;
                let stone = Stone::new().await?;
                let background = Background::new().await?;
                Ok(Box::new(WalkTheDog::Loaded(Walk { rhb, background, stone })))
            }
            Self::Loaded(_) => Err(anyhow!("Error: Game is already initialized")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        match self {
            Self::Loading => {}
            Self::Loaded(Walk { rhb, background: _, stone }) => {
                rhb.update();

                if rhb.bounding_box().intersects(&stone.bounding_box()) {
                    rhb.knock_out();
                }

                if keystate.is_pressed("ArrowRight") {
                    rhb.run_right();
                }

                if keystate.is_pressed("ArrowLeft") {
                    rhb.run_left();
                }

                if keystate.is_pressed("ArrowDown") {
                    rhb.slide();
                }

                if keystate.is_pressed("ArrowUp") {
                    rhb.jump();
                }
            }
        }
    }

    fn draw(&self, renderer: &Renderer) {
        match self {
            WalkTheDog::Loading => {}
            WalkTheDog::Loaded(Walk { rhb, background, stone }) => {
                renderer.clear(&Rect {
                    x: 0.,
                    y: 0.,
                    w: 600.,
                    h: 600.,
                });

                background.draw(renderer).expect("Error drawing background");
                rhb.draw(renderer).expect("Error drawing red hat boy");
                stone.draw(renderer).expect("Error drawing stone");
            }
        }
    }
}
