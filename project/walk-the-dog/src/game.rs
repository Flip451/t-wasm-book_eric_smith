use std::rc::Rc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rand::{thread_rng, Rng};
use web_sys::HtmlImageElement;

use crate::{
    engine::{
        key_state::KeyState,
        renderer::{sprite::Sprite, Point, Rect, Renderer},
        Game,
    },
    segments::{rightmost, stone_and_high_platform, two_stone_and_low_platform},
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

const TIMELINE_MINIMUM: i16 = 1000;
const OBSTACLE_BUFFER: i16 = 20;

pub enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

pub struct Walk {
    rhb: RedHatBoy,
    background: Background,
    obstacles: Vec<Box<dyn Obstacle>>,
    obstacle_sheet: Rc<Sprite>,
    stone: HtmlImageElement,
    timeline: i16,
}

impl Walk {
    fn velocity(&self) -> i16 {
        -self.rhb.walking_speed()
    }

    fn generate_next_segment(&mut self) {
        let mut rng = thread_rng();
        let next_segment = rng.gen_range(0..2);

        let mut next_obstacles = match next_segment {
            0 => stone_and_high_platform(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),
            1 => two_stone_and_low_platform(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),
            _ => vec![],
        };

        self.timeline = rightmost(&next_obstacles);
        self.obstacles.append(&mut next_obstacles);
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

                let obstacles =
                    two_stone_and_low_platform(stone_image.clone(), platform_sprite.clone(), 0);
                let timeline = rightmost(&obstacles);

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    rhb,
                    background,
                    obstacles,
                    obstacle_sheet: platform_sprite,
                    stone: stone_image,
                    timeline,
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

                walk.rhb.update();

                walk.background.update(velocity);

                // 画面外に出た障害物を削除する
                walk.obstacles
                    .retain(|obstacle| obstacle.bounding_box().right() > 0);

                walk.obstacles.iter_mut().for_each(|obstacle| {
                    obstacle.update_position(velocity);
                    obstacle.check_intersection(&mut walk.rhb);
                });

                if walk.timeline < TIMELINE_MINIMUM {
                    walk.generate_next_segment();
                } else {
                    walk.timeline += velocity;
                }

                if keystate.is_pressed("ArrowRight") {
                    walk.rhb.run_right();
                }

                if keystate.is_pressed("ArrowLeft") {
                    walk.rhb.run_left();
                }

                if keystate.is_pressed("ArrowDown") {
                    walk.rhb.slide();
                }

                if keystate.is_pressed("ArrowUp") {
                    walk.rhb.jump();
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
                stone: _,
                timeline: _,
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
