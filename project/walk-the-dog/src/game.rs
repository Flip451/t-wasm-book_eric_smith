use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::engine::{
    key_state::KeyState,
    renderer::{Point, Rect, Renderer},
    Game,
};

use self::{
    background::Background,
    objects::{platform::Platform, GameObject},
    rhb::{RedHatBoy, FLOOR, STARTING_POINT},
};

mod background;
mod bounding_box;
mod objects;
mod rhb;
mod sprite;

use objects::stone::Stone;

const WIDTH: f32 = 600.;
const HEIGHT: f32 = 600.;

pub enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

pub struct Walk {
    rhb: RedHatBoy,
    background: Background,
    stone: Stone,
    platform: Platform,
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
                let rhb = RedHatBoy::new(Point {
                    x: STARTING_POINT,
                    y: FLOOR,
                })
                .await?;
                let stone = Stone::new(Point { x: 150., y: 546. }).await?;
                let background = Background::new().await?;
                let platform = Platform::new(Point { x: 200., y: 400. }).await?;
                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    rhb,
                    background,
                    stone,
                    platform,
                })))
            }
            Self::Loaded(_) => Err(anyhow!("Error: Game is already initialized")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        match self {
            Self::Loading => {}
            Self::Loaded(Walk {
                rhb,
                background: _,
                stone,
                platform,
            }) => {
                rhb.update();

                // rhb のbounding box と platform の bounding box が重なっているかどうかを判定
                if let Some((rhb_rect, platform_rect)) =
                    rhb.bounding_box().intersects(&platform.bounding_box())
                {
                    // rhb が platform より上にいるかどうかを判定
                    // かつ rhb が落下しているかどうかを判定
                    if rhb_rect.y < platform_rect.y && rhb.is_falling() {
                        rhb.land_on(platform_rect.y);
                    } else {
                        rhb.knock_out();
                    }
                }

                // rhb のbounding box と stone の bounding box が重なっているかどうかを判定
                if let Some((_, _)) = rhb.bounding_box().intersects(&stone.bounding_box()) {
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
            WalkTheDog::Loaded(Walk {
                rhb,
                background,
                stone,
                platform,
            }) => {
                renderer.clear(&Rect {
                    x: 0.,
                    y: 0.,
                    w: WIDTH,
                    h: HEIGHT,
                });

                background.draw(renderer).expect("Error drawing background");
                rhb.draw(renderer).expect("Error drawing red hat boy");
                stone.draw(renderer).expect("Error drawing stone");
                platform.draw(renderer).expect("Error drawing platform");
            }
        }
    }
}
