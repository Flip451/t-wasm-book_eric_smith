use anyhow::Result;
use web_sys::HtmlImageElement;

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

impl Stone {
    pub fn new(image: HtmlImageElement, position: Point) -> Self {
        Self {
            image: Image::new(image, position),
        }
    }

    pub async fn load_image() -> Result<HtmlImageElement> {
        image::load_image("Stone.png").await
    }
}

impl GameObject for Stone {
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
        self.image.draw(renderer)?;

        #[cfg(feature = "collision_debug")]
        self.bounding_box().draw(renderer)?;

        Ok(())
    }
}

impl Obstacle for Stone {
    fn update_position(&mut self, velocity: i16) {
        self.image.move_horizontally(velocity);
    }

    fn check_intersection(&self, rhb: &mut crate::game::rhb::RedHatBoy) {
        if let Some((_, _)) = rhb.bounding_box().intersects(&self.bounding_box()) {
            rhb.knock_out();
        }
    }
}
