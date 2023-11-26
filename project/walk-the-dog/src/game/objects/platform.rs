use anyhow::Result;
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;
use web_sys::HtmlImageElement;

use crate::{
    browser,
    engine::renderer::{image, Point, Rect, Renderer},
    game::sprite::SpriteSheet,
};

use super::GameObject;

pub struct Platform {
    sprite_sheet: SpriteSheet,
    image: HtmlImageElement,
    position: Point,
}

#[async_trait(?Send)]
impl GameObject for Platform {
    async fn new(position: Point) -> Result<Self> {
        let json = browser::fetch_json("tiles.json").await?;
        let sprite_sheet: SpriteSheet = json.into_serde()?;

        let image = image::load_image("tiles.png").await?;

        Ok(Self {
            sprite_sheet,
            image,
            position,
        })
    }

    fn bounding_box(&self) -> Rect {
        let sprite = self
            .sprite_sheet
            .frames
            .get("13.png")
            .expect("Error: Cell not found");

        sprite.to_rect_on_canvas(
            self.position.x,
            self.position.y,
            sprite.width() * 3.,
            sprite.height(),
        )
    }

    fn draw(&self, renderer: &Renderer) -> Result<()> {
        #[cfg(feature = "collision_debug")]
        renderer.draw_rect(&self.bounding_box());

        let sprite = self
            .sprite_sheet
            .frames
            .get("13.png")
            .expect("Error: 13.png not found in sprite sheet");

        renderer.draw_image(
            &self.image,
            &Rect {
                x: sprite.x(),
                y: sprite.y(),
                w: sprite.width() * 3.,
                h: sprite.height(),
            },
            &sprite.to_rect_on_canvas(
                self.position.x,
                self.position.y,
                sprite.width() * 3.,
                sprite.height(),
            ),
        )?;

        Ok(())
    }
}
