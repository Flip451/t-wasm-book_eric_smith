use std::rc::Rc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::{
    engine::{
        key_state::KeyState,
        renderer::{sprite::Sprite, Point, Rect, Renderer},
        Game,
    },
    segments::two_stone_and_low_platform,
};

use self::{
    background::Background,
    objects::{platform::Platform, GameObject, Obstacle},
    rhb::{RedHatBoy, FLOOR, STARTING_POINT},
};

mod background;
pub mod bounding_box;
pub mod objects;
mod rhb;

use objects::stone::Stone;

const WIDTH: i16 = 600;
const HEIGHT: i16 = 600;

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

                let stone_image = Stone::load_image().await?;

                let platform_sprite = Platform::load_sprite().await?;

                let obstacles = two_stone_and_low_platform(stone_image, platform_sprite.clone(), 0);

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
