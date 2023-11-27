use anyhow::Result;
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;
use web_sys::HtmlImageElement;

use crate::{
    browser,
    engine::renderer::{image, Point, Rect, Renderer},
    game::{bounding_box::BoundingBox, sprite::SpriteSheet},
};

use super::{GameObject, Obstacle};

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

    fn bounding_box(&self) -> BoundingBox {
        const X_OFFSET: f32 = 60.;
        const END_HEIGHT: f32 = 54.;

        let sprite = self
            .sprite_sheet
            .frames
            .get("13.png")
            .expect("Error: Cell not found");

        let raw_rect = sprite.to_rect_on_canvas(
            self.position.x,
            self.position.y,
            sprite.width() * 3.,
            sprite.height(),
        );

        let mut bounding_box = BoundingBox::new();

        bounding_box.add(Rect {
            x: raw_rect.x,
            y: raw_rect.y,
            w: X_OFFSET,
            h: END_HEIGHT,
        });

        bounding_box.add(Rect {
            x: raw_rect.x + X_OFFSET,
            y: raw_rect.y,
            w: raw_rect.w - X_OFFSET * 2.,
            h: raw_rect.h,
        });

        bounding_box.add(Rect {
            x: raw_rect.x + raw_rect.w - X_OFFSET,
            y: raw_rect.y,
            w: X_OFFSET,
            h: END_HEIGHT,
        });

        bounding_box
    }

    fn draw(&self, renderer: &Renderer) -> Result<()> {
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

        // キャンバスに bounding box を描画
        #[cfg(feature = "collision_debug")]
        self.bounding_box().draw(renderer)?;

        Ok(())
    }
}

impl Obstacle for Platform {
    fn update(&mut self, velocity: f32) {
        self.position.x += velocity;
    }
}