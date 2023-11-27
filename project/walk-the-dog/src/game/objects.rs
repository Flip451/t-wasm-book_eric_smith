pub mod stone;
pub mod platform;

use anyhow::Result;
use async_trait::async_trait;

use crate::engine::renderer::{Renderer, Point};

use super::{bounding_box::BoundingBox, rhb::RedHatBoy};

#[async_trait(?Send)]
pub trait GameObject {
    async fn new(position: Point) -> Result<Self>
    where
        Self: Sized;
    fn bounding_box(&self) -> BoundingBox;
    fn draw(&self, renderer: &Renderer) -> Result<()>;
}

pub trait Obstacle: GameObject {
    fn update_position(&mut self, velocity: i16);
    fn check_intersection(&self, rhb: &mut RedHatBoy);
}