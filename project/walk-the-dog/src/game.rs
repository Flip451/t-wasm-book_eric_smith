use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::engine::{
    key_state::KeyState,
    renderer::{Rect, Renderer},
    Game,
};

use self::rhb::RedHatBoy;

mod rhb;
mod sprite;

pub enum WalkTheDog {
    Loading,
    Loaded(RedHatBoy),
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
                Ok(Box::new(WalkTheDog::Loaded(rhb)))
            }
            Self::Loaded(_) => Err(anyhow!("Error: Game is already initialized")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
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

                if keystate.is_pressed("ArrowUp") {
                    rhb.jump();
                }
            }
        }
    }

    fn draw(&self, renderer: &Renderer) {
        match self {
            WalkTheDog::Loading => {}
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
