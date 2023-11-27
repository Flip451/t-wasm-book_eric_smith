use anyhow::Result;

use crate::engine::renderer::{
    image::{self, Image},
    Point, Renderer,
};

pub struct Background {
    images: [Image; 2],
}

impl Background {
    pub async fn new() -> Result<Self> {
        let image = image::load_image("BG.png").await?;
        let image1 = Image::new(image.clone(), Point { x: 0, y: 0 });
        let image2 = Image::new(
            image,
            Point {
                x: image1.width(),
                y: 0,
            },
        );
        Ok(Self {
            images: [image1, image2],
        })
    }

    pub fn draw(&self, renderer: &Renderer) -> Result<()> {
        self.images
            .iter()
            .map(|image| image.draw(renderer))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    pub fn update(&mut self, velocity: i16) {
        self.images.iter_mut().for_each(|image| {
            image.move_horizontally(velocity);
        });

        let len = self.images.len();
        let right_position_list = self.images.iter().map(|image| image.right()).collect::<Vec<_>>();

        let image_out_of_game = self.images.iter_mut().enumerate().find(|(_, image)| {
            image.right() < 0
        });

        if let Some((index, image)) = image_out_of_game {
            let new_x = right_position_list[(index + len - 1) % len];
            image.set_x(new_x);
        }
    }
}
