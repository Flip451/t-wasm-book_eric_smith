use anyhow::Result;
use async_trait::async_trait;

use crate::engine::renderer::{
    image::{self, Image},
    Point, Rect, Renderer,
};

use super::GameObject;

pub struct Stone {
    image: Image,
    bounding_box: Rect,
}

#[async_trait(?Send)]
impl GameObject for Stone {
    async fn new() -> Result<Self> {
        let image = image::load_image("Stone.png").await?;
        let image = Image::new(image, Point { x: 150., y: 546. });
        let bounding_box = Rect {
            x: image.position().x,
            y: image.position().y,
            w: image.width(),
            h: image.height(),
        };
        Ok(Self {
            image,
            bounding_box,
        })
    }

    fn draw(&self, renderer: &Renderer) -> Result<()> {
        self.image.draw(renderer)
    }
}
