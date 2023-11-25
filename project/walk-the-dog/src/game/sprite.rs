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
pub struct Cell {
    frame: SheetRect,
}

type RectOnCanvas = Rect;
type RectOnSheet = Rect;

impl Cell {
    pub fn to_rect_on_sheet(&self) -> RectOnSheet {
        RectOnSheet {
            x: self.frame.x as f32,
            y: self.frame.y as f32,
            w: self.frame.w as f32,
            h: self.frame.h as f32,
        }
    }

    pub fn to_rect_on_canvas(&self, x: f32, y: f32) -> RectOnCanvas {
        RectOnCanvas {
            x,
            y,
            w: self.frame.w as f32,
            h: self.frame.h as f32,
        }
    }
}

#[derive(Deserialize)]
pub struct SpriteSheet {
    pub frames: HashMap<String, Cell>,
}
