use anyhow::Result;

use crate::engine::renderer::{
    image::{self, Image},
    Renderer, Point,
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

    pub fn draw(&self, renderer: &Renderer) {
        self.image.draw(renderer);
    }
}
