pub mod stone;

use anyhow::Result;
use async_trait::async_trait;

use crate::engine::renderer::{Renderer, Rect};

#[async_trait(?Send)]
pub trait GameObject {
    async fn new() -> Result<Self>
    where
        Self: Sized;
    fn bounding_box(&self) -> Rect;
    fn draw(&self, renderer: &Renderer) -> Result<()>;
}
