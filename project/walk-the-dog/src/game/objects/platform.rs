use std::rc::Rc;

use anyhow::Result;
use gloo_utils::format::JsValueSerdeExt;

use crate::{
    browser,
    engine::renderer::{
        image,
        sprite::{Cell, Sprite, SpriteSheet},
        Point, Rect, Renderer,
    },
    game::bounding_box::BoundingBox,
};

use super::{GameObject, Obstacle};

pub struct Platform {
    sprite: Rc<Sprite>,
    position: Point,
    sprite_cells: Vec<Cell>,
    bounding_box: BoundingBox,
}

impl Platform {
    pub fn new(
        sprite: Rc<Sprite>,
        position: Point,
        sprite_names: &[&str],
        mut bounding_box: BoundingBox,
    ) -> Self {
        let sprite_cells = sprite_names
            .iter()
            .map(|name| sprite.cell(name).cloned().expect("Error: Cell not found"))
            .collect();
        bounding_box.move_by(position);

        Self {
            sprite,
            position,
            sprite_cells,
            bounding_box,
        }
    }

    pub async fn load_sprite() -> Result<Rc<Sprite>> {
        let json = browser::fetch_json("tiles.json").await?;
        let sprite_sheet: SpriteSheet = json.into_serde()?;
        let image = image::load_image("tiles.png").await?;
        let sprite = Rc::new(Sprite::new(sprite_sheet, image));

        Ok(sprite)
    }
}

impl GameObject for Platform {
    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box.clone()
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
        self.bounding_box.move_by(Point { x: velocity, y: 0 });
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
