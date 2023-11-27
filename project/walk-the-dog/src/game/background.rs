use anyhow::Result;

use crate::engine::renderer::{
    image::{self, Image},
    Point, Renderer,
};

pub struct Background {
    image: Image,
}

impl Background {
    pub async fn new() -> Result<Self> {
        let image = image::load_image("BG.png").await?;
        let image = Image::new(image, Point { x: 0., y: 0. });
        Ok(Self { image })
    }

    pub fn draw(&self, renderer: &Renderer) -> Result<()> {
        self.image.draw(renderer)
    }

    pub fn update(&mut self, velocity: f32) {
        self.image.move_horizontally(velocity);
    }
}
