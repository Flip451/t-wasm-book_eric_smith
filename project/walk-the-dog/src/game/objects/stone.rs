use anyhow::Result;
use async_trait::async_trait;

use crate::{
    engine::renderer::{
        image::{self, Image},
        Point, Rect, Renderer,
    },
    game::bounding_box::BoundingBox,
};

use super::{GameObject, Obstacle};

pub struct Stone {
    image: Image,
}

#[async_trait(?Send)]
impl GameObject for Stone {
    async fn new(position: Point) -> Result<Self> {
        let image = image::load_image("Stone.png").await?;
        let image = Image::new(image, position);
        Ok(Self { image })
    }

    fn bounding_box(&self) -> BoundingBox {
        let bounding_box = Rect::new(
            self.image.position().clone(),
            self.image.width(),
            self.image.height(),
        );
        let mut bounding_boxes = BoundingBox::new();
        bounding_boxes.add(bounding_box);
        bounding_boxes
    }

    fn draw(&self, renderer: &Renderer) -> Result<()> {
        self.image.draw(renderer);

        #[cfg(feature = "collision_debug")]
        self.bounding_box().draw(renderer);

        Ok(())
    }
}

impl Obstacle for Stone {
    fn update(&mut self, velocity: i16) {
        self.image.move_horizontally(velocity);
    }
}
