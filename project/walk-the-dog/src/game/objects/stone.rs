use anyhow::Result;

use crate::engine::renderer::{
    image::{self, Image},
    Point, Renderer,
};

pub struct Stone {
    image: Image,
}

impl Stone {
    pub async fn new() -> Result<Self> {
        let image = image::load_image("Stone.png").await?;
        let image = Image::new(image, Point { x: 150., y: 546. });
        Ok(Self { image })
    }

    pub fn draw(&self, renderer: &Renderer) -> Result<()> {
        self.image.draw(renderer)
    }
}
