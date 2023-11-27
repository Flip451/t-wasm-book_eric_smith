use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::engine::{
    key_state::KeyState,
    renderer::{Point, Rect, Renderer},
    Game,
};

use self::{
    background::Background,
    objects::{platform::Platform, GameObject, Obstacle},
    rhb::{RedHatBoy, FLOOR, STARTING_POINT},
};

mod background;
mod bounding_box;
mod objects;
mod rhb;
mod sprite;

use objects::stone::Stone;

const WIDTH: i16 = 600;
const HEIGHT: i16 = 600;

const LOW_PLATFORM: i16 = 420;
const HIGH_PLATFORM: i16 = 375;
const FIRST_PLATFORM: i16 = 300;

const FIRST_STONE_X: i16 = 450;
const FIRST_STONE_Y: i16 = 366;

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

impl Walk {
    fn velocity(&self) -> i16 {
        -self.rhb.walking_speed()
    }
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
                let stone = Stone::new(Point {
                    x: FIRST_STONE_X,
                    y: FIRST_STONE_Y,
                })
                .await?;
                let background = Background::new().await?;
                let platform = Platform::new(Point {
                    x: FIRST_PLATFORM,
                    y: LOW_PLATFORM,
                })
                .await?;
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
            Self::Loaded(walk) => {
                let velocity = walk.velocity();

                let Walk {
                    rhb,
                    background,
                    stone,
                    platform,
                } = walk;
                rhb.update();

                platform.update(velocity);
                stone.update(velocity);
                background.update(velocity);

                // rhb のbounding box と platform の bounding box が重なっているかどうかを判定
                if let Some((rhb_rect, platform_rect)) =
                    rhb.bounding_box().intersects(&platform.bounding_box())
                {
                    // rhb が platform より上にいるかどうかを判定
                    // かつ rhb が落下しているかどうかを判定
                    if rhb_rect.y() < platform_rect.y() && rhb.is_falling() {
                        rhb.land_on(platform_rect.y());
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
                renderer.clear(&Rect::new_from_x_y(0, 0, WIDTH, HEIGHT));

                background.draw(renderer).expect("Error drawing background");
                rhb.draw(renderer).expect("Error drawing red hat boy");
                stone.draw(renderer).expect("Error drawing stone");
                platform.draw(renderer).expect("Error drawing platform");
            }
        }
    }
}
