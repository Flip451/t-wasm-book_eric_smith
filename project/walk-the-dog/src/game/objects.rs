pub mod stone;

use anyhow::Result;
use async_trait::async_trait;

use crate::engine::renderer::Renderer;

#[async_trait(?Send)]
pub trait GameObject {
    async fn new() -> Result<Self>
    where
        Self: Sized;
    fn draw(&self, renderer: &Renderer) -> Result<()>;
}
