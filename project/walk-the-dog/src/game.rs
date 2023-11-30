use std::rc::Rc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::engine::{
    key_state::KeyState,
    renderer::{sprite::Sprite, Point, Rect, Renderer},
    Game,
};

use self::{
    background::Background,
    bounding_box::BoundingBox,
    objects::{platform::Platform, GameObject, Obstacle},
    rhb::{RedHatBoy, FLOOR, STARTING_POINT},
};

mod background;
mod bounding_box;
mod objects;
mod rhb;

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
    obstacle_sheet: Rc<Sprite>,
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
                let rhb_sprite = RedHatBoy::load_sprite().await?;
                let rhb = RedHatBoy::new(
                    rhb_sprite,
                    Point {
                        x: STARTING_POINT,
                        y: FLOOR,
                    },
                );

                let background = Background::new().await?;

                let mut obstacles = Vec::<Box<dyn Obstacle>>::new();

                let stone_image = Stone::load_image().await?;
                let stone = Stone::new(
                    stone_image,
                    Point {
                        x: FIRST_STONE_X,
                        y: FIRST_STONE_Y,
                    },
                );
                obstacles.push(Box::new(stone));

                let platform_sprite = Platform::load_sprite().await?;
                let platform = Platform::new(
                    Rc::clone(&platform_sprite),
                    Point {
                        x: FIRST_PLATFORM,
                        y: LOW_PLATFORM,
                    },
                    &["13.png", "14.png", "15.png"],
                    BoundingBox::new(vec![
                        Rect::new_from_x_y(0, 0, 60, 54),
                        Rect::new_from_x_y(60, 0, 384 - (60 * 2), 93),
                        Rect::new_from_x_y(384 - 60, 0, 60, 54),
                    ]),
                );
                obstacles.push(Box::new(platform));

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    rhb,
                    background,
                    obstacles,
                    obstacle_sheet: platform_sprite,
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
                    obstacle_sheet: _,
                } = walk;
                rhb.update();

                background.update(velocity);

                // 画面外に出た障害物を削除する
                obstacles.retain(|obstacle| obstacle.bounding_box().right() > 0);

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
                obstacle_sheet: _,
            }) => {
                renderer.clear(&Rect::new_from_x_y(0, 0, WIDTH, HEIGHT));

                background.draw(renderer).expect("Error drawing background");
                rhb.draw(renderer).expect("Error drawing red hat boy");
                obstacles
                    .iter()
                    .for_each(|obstacle| obstacle.draw(renderer).expect("Error drawing obstacle"));
            }
        }
    }
}
