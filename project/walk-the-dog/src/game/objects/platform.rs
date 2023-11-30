use anyhow::Result;
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;

use crate::{
    browser,
    engine::renderer::{
        image,
        sprite::{Sprite, SpriteSheet},
        Point, Rect, Renderer,
    },
    game::bounding_box::BoundingBox,
};

use super::{GameObject, Obstacle};

pub struct Platform {
    sprite: Sprite,
    position: Point,
}

#[async_trait(?Send)]
impl GameObject for Platform {
    async fn new(position: Point) -> Result<Self> {
        let json = browser::fetch_json("tiles.json").await?;
        let sprite_sheet: SpriteSheet = json.into_serde()?;
        let image = image::load_image("tiles.png").await?;
        let sprite = Sprite::new(sprite_sheet, image);

        Ok(Self { sprite, position })
    }

    fn bounding_box(&self) -> BoundingBox {
        const X_OFFSET: i16 = 60;
        const END_HEIGHT: i16 = 54;

        let sprite = self.sprite.cell("13.png").expect("Error: Cell not found");

        let raw_rect = sprite.to_rect_on_canvas(
            self.position.x,
            self.position.y,
            sprite.width() * 3,
            sprite.height(),
        );

        let mut bounding_box = BoundingBox::new();

        bounding_box.add(Rect::new_from_x_y(
            raw_rect.x(),
            raw_rect.y(),
            X_OFFSET,
            END_HEIGHT,
        ));

        bounding_box.add(Rect::new_from_x_y(
            raw_rect.x() + X_OFFSET,
            raw_rect.y(),
            raw_rect.w - X_OFFSET * 2,
            raw_rect.h,
        ));

        bounding_box.add(Rect::new_from_x_y(
            raw_rect.x() + raw_rect.w - X_OFFSET,
            raw_rect.y(),
            X_OFFSET,
            END_HEIGHT,
        ));

        bounding_box
    }

    fn draw(&self, renderer: &Renderer) -> Result<()> {
        let sprite = self
            .sprite
            .cell("13.png")
            .expect("Error: 13.png not found in sprite sheet");

        self.sprite.draw(
            &renderer,
            &Rect::new_from_x_y(sprite.x(), sprite.y(), sprite.width() * 3, sprite.height()),
            &sprite.to_rect_on_canvas(
                self.position.x,
                self.position.y,
                sprite.width() * 3,
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
    fn update_position(&mut self, velocity: i16) {
        self.position.x += velocity;
    }

    fn check_intersection(&self, rhb: &mut crate::game::rhb::RedHatBoy) {
        if let Some((rhb_rect, platform_rect)) = rhb.bounding_box().intersects(&self.bounding_box())
        {
            // rhb が platform より上にいるかどうかを判定
            // かつ rhb が落下しているかどうかを判定
            if rhb_rect.y() < platform_rect.y() && rhb.is_falling() {
                rhb.land_on(platform_rect.y());
            } else {
                rhb.knock_out();
            }
        }
    }
}
