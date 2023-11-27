use serde::Deserialize;
use std::collections::HashMap;

use crate::engine::renderer::Rect;

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    frame: SheetRect,
    sprite_source_size: SheetRect,
}

impl Cell {
    pub fn to_rect_on_canvas(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            x: x + self.sprite_source_size.x as f32,
            y: y + self.sprite_source_size.y as f32,
            w,
            h,
        }
    }

    pub fn x(&self) -> f32 {
        self.frame.x as f32
    }

    pub fn y(&self) -> f32 {
        self.frame.y as f32
    }

    pub fn width(&self) -> f32 {
        self.frame.w as f32
    }

    pub fn height(&self) -> f32 {
        self.frame.h as f32
    }
}

#[derive(Deserialize)]
pub struct SpriteSheet {
    pub frames: HashMap<String, Cell>,
}
