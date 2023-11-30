pub mod platform;
pub mod stone;

use anyhow::Result;

use crate::engine::renderer::Renderer;

use super::{bounding_box::BoundingBox, rhb::RedHatBoy};

pub trait GameObject {
    fn bounding_box(&self) -> BoundingBox;
    fn draw(&self, renderer: &Renderer) -> Result<()>;
}

pub trait Obstacle: GameObject {
    fn update_position(&mut self, velocity: i16);
    fn check_intersection(&self, rhb: &mut RedHatBoy);
}
