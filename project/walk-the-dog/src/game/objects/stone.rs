use anyhow::Result;
use async_trait::async_trait;

use crate::{engine::renderer::{
    image::{self, Image},
    Point, Rect, Renderer,
}, game::bounding_box::BoundingBox};

use super::{GameObject, Obstacle};

pub struct Stone {
    image: Image,
    bounding_box: Rect,
}

#[async_trait(?Send)]
impl GameObject for Stone {
    async fn new(position: Point) -> Result<Self> {
        let image = image::load_image("Stone.png").await?;
        let image = Image::new(image, position);
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

    fn bounding_box(&self) -> BoundingBox {
        let mut bounding_boxes = BoundingBox::new();
        bounding_boxes.add(self.bounding_box.clone());
        bounding_boxes
    }

    fn draw(&self, renderer: &Renderer) -> Result<()> {
        #[cfg(feature = "collision_debug")]
        renderer.draw_rect(&self.bounding_box);
        self.image.draw(renderer)
    }
}

impl Obstacle for Stone {
    fn update(&mut self, velocity: i16) {
        self.image.move_horizontally(velocity);
        self.bounding_box.x = self.image.position().x;
        self.bounding_box.y = self.image.position().y;
    }
}