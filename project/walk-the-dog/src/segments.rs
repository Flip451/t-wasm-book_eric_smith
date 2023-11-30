use std::rc::Rc;

use web_sys::HtmlImageElement;

use crate::{
    engine::renderer::{sprite::Sprite, Point, Rect},
    game::{
        bounding_box::BoundingBox,
        objects::{platform::Platform, stone::Stone, Obstacle},
    },
};

// stone の y 座標
const STONE_ON_GROUND: i16 = 545;
const STONE_ON_LOW_PLATFORM: i16 = 366;
const STONE_ON_HIGH_PLATFORM: i16 = 321;

// platform の y 座標
const LOW_PLATFORM: i16 = 420;
const HIGH_PLATFORM: i16 = 375;

const FLOATING_PLATFORM_CELLS: [&str; 3] = ["13.png", "14.png", "15.png"];
const FLOATING_PLATFORM_BOUNDING_BOX: [Rect; 3] = [
    Rect::new_from_x_y(0, 0, 60, 54),
    Rect::new_from_x_y(60, 0, 384 - (60 * 2), 93),
    Rect::new_from_x_y(384 - 60, 0, 60, 54),
];

pub fn rightmost(obstacle_list: &Vec<Box<dyn Obstacle>>) -> i16 {
    obstacle_list
        .iter()
        .map(|obstacle| obstacle.bounding_box().right())
        .max()
        .unwrap_or(0)
}

pub fn create_floating_platform(sprite: Rc<Sprite>, position: Point) -> Platform {
    Platform::new(
        sprite,
        position,
        &FLOATING_PLATFORM_CELLS,
        BoundingBox::new(FLOATING_PLATFORM_BOUNDING_BOX.to_vec()),
    )
}

pub fn two_stone_and_low_platform(
    stone_image: HtmlImageElement,
    sprite: Rc<Sprite>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const FIRST_STONE_X: i16 = 230;
    const SECOND_STONE_X: i16 = 450;
    const FIRST_PLATFORM: i16 = 300;

    vec![
        Box::new(Stone::new(
            stone_image.clone(),
            Point {
                x: offset_x + FIRST_STONE_X,
                y: STONE_ON_GROUND,
            },
        )),
        Box::new(Stone::new(
            stone_image,
            Point {
                x: offset_x + SECOND_STONE_X,
                y: STONE_ON_LOW_PLATFORM,
            },
        )),
        Box::new(create_floating_platform(
            sprite,
            Point {
                x: offset_x + FIRST_PLATFORM,
                y: LOW_PLATFORM,
            },
        )),
    ]
}

pub fn stone_and_high_platform(
    stone_image: HtmlImageElement,
    sprite: Rc<Sprite>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const STONE_X: i16 = 330;
    const PLATFORM_X: i16 = 300;

    vec![
        Box::new(Stone::new(
            stone_image,
            Point {
                x: offset_x + STONE_X,
                y: STONE_ON_HIGH_PLATFORM,
            },
        )),
        Box::new(create_floating_platform(
            sprite,
            Point {
                x: offset_x + PLATFORM_X,
                y: HIGH_PLATFORM,
            },
        )),
    ]
}
