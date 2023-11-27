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
    pub fn to_rect_on_canvas(&self, x: i16, y: i16, w: i16, h: i16) -> Rect {
        Rect::new_from_x_y(
            x + self.sprite_source_size.x,
            y + self.sprite_source_size.y,
            w,
            h,
        )
    }

    pub fn x(&self) -> i16 {
        self.frame.x
    }

    pub fn y(&self) -> i16 {
        self.frame.y
    }

    pub fn width(&self) -> i16 {
        self.frame.w
    }

    pub fn height(&self) -> i16 {
        self.frame.h
    }
}

#[derive(Deserialize)]
pub struct SpriteSheet {
    pub frames: HashMap<String, Cell>,
}
