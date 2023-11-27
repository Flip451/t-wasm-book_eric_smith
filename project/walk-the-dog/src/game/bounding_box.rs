use anyhow::Result;
use crate::engine::renderer::{Rect, Renderer};

pub struct BoundingBox {
    boxes: Vec<Rect>
}

impl BoundingBox {
    pub fn new() -> Self {
        Self {
            boxes: Vec::new()
        }
    }

    pub fn add(&mut self, rect: Rect) {
        self.boxes.push(rect);
    }

    pub fn intersects<'a>(&'a self, other: &'a BoundingBox) -> Option<(&Rect, &Rect)> {
        self.boxes.iter().find_map(|rect| {
            other.boxes.iter().find_map(|other_rect| {
                if rect.intersects(&other_rect) {
                    Some((rect, other_rect))
                } else {
                    None
                }
            })
        })
    }

    pub fn draw(&self, renderer: &Renderer) -> Result<()> {
        self.boxes.iter().for_each(|rect| {
            renderer.draw_rect(rect);
        });
        Ok(())
    }
}