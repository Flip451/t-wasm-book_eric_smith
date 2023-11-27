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
    obstacles: Vec<Box<dyn Obstacle>>,
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

                let background = Background::new().await?;

                let mut obstacles = Vec::<Box<dyn Obstacle>>::new();
                let stone = Stone::new(Point {
                    x: FIRST_STONE_X,
                    y: FIRST_STONE_Y,
                }).await?;
                obstacles.push(Box::new(stone));

                let platform = Platform::new(Point {
                    x: FIRST_PLATFORM,
                    y: LOW_PLATFORM,
                })
                .await?;
                obstacles.push(Box::new(platform));

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    rhb,
                    background,
                    obstacles,
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
                    obstacles,
                } = walk;
                rhb.update();

                background.update(velocity);

                // 画面外に出た障害物を削除する
                obstacles.retain(|obstacle| {obstacle.bounding_box().right() > 0});

                for obstacle in obstacles {
                    obstacle.update_position(velocity);
                    obstacle.check_intersection(rhb);
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
                obstacles,
            }) => {
                renderer.clear(&Rect::new_from_x_y(0, 0, WIDTH, HEIGHT));

                background.draw(renderer).expect("Error drawing background");
                rhb.draw(renderer).expect("Error drawing red hat boy");
                obstacles.iter().for_each(|obstacle| {
                    obstacle.draw(renderer).expect("Error drawing obstacle")
                });
            }
        }
    }
}
