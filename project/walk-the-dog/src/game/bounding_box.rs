use anyhow::Result;
use crate::engine::renderer::{Rect, Renderer, Point};

#[derive(Clone)]
pub struct BoundingBox {
    boxes: Vec<Rect>
}

impl BoundingBox {
    pub fn new(rects: Vec<Rect>) -> Self {
        Self {
            boxes: rects
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

    pub fn right(&self) -> i16 {
        self.boxes.iter().map(|rect| rect.right()).max().unwrap_or(0)
    }

    pub fn move_by(&mut self, position: Point) {
        self.boxes.iter_mut().for_each(|rect| {
            rect.move_by(position);
        });
    }
}