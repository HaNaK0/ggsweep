use ggez::graphics;

use crate::{
    err_here,
    error::{LocatedError, WrappedError},
    sprite_sheet::SpriteSheet,
};

#[allow(dead_code)]
enum PanelDirection {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

/// A panel for a window or a part of the UI.
/// Sized dynamically but will look weird with sizes close above a multiple of the size of the sub sprite
pub struct Panel {
    rect: graphics::Rect,
    sprites: [String; 9],
}

impl Panel {
    pub fn new(rect: graphics::Rect, sprites: [String; 9]) -> Self {
        Self { rect, sprites }
    }

    pub fn draw(
        &self,
        ctx: &mut ggez::Context,
        sprite_sheet: &SpriteSheet,
    ) -> Result<(), LocatedError> {
        let sprite_size = sprite_sheet
            .get_sprite_pixel_size(&self.sprites[0])
            .ok_or_else(|| {
                WrappedError::SheetError(format!(
                    "Sprite Sheet {:?} does not have {:?}",
                    sprite_sheet, &self.sprites[0]
                ))
            })
            .map_err(err_here!())?;

        let sprites = self
            .sprites
            .iter()
            .map(|s| (s, sprite_sheet.get_sprite_and_rect(&s)))
            .map(|o| {
                o.1.ok_or_else(|| {
                    WrappedError::SheetError(format!("Sprite sheet does not have{}", o.0))
                })
                .map_err(err_here!())
            })
            .collect::<Result<Vec<(&graphics::Image, &graphics::Rect)>, LocatedError>>()?;

        // Draw the corners
        // Top left
        let (corner_sprite, src_rect) = sprites[PanelDirection::TopLeft as usize];
        let params = graphics::DrawParam::default()
            .dest(self.rect.point())
            .src(*src_rect);
        graphics::draw(ctx, corner_sprite, params).map_err(err_here!())?;

        // Top Right
        let (corner_sprite, src_rect) = sprites[PanelDirection::TopRight as usize];
        let dest = cgmath::point2(self.rect.right() - sprite_size.x, self.rect.top());
        let params = graphics::DrawParam::default().dest(dest).src(*src_rect);
        graphics::draw(ctx, corner_sprite, params).map_err(err_here!())?;

        // Bottom Left
        let (corner_sprite, src_rect) = sprites[PanelDirection::BottomLeft as usize];
        let dest = cgmath::point2(self.rect.left(), self.rect.bottom() - sprite_size.y);
        let params = graphics::DrawParam::default().dest(dest).src(*src_rect);
        graphics::draw(ctx, corner_sprite, params).map_err(err_here!())?;

        // Bottom Right
        let (corner_sprite, src_rect) = sprites[PanelDirection::BottomRight as usize];
        let dest = cgmath::point2(self.rect.right(), self.rect.bottom()) - sprite_size;
        let params = graphics::DrawParam::default().dest(dest).src(*src_rect);
        graphics::draw(ctx, corner_sprite, params).map_err(err_here!())?;

        let number_of_vert = (self.rect.h / sprite_size.y - 1.0).max(0.0) as usize;
        let number_of_hor = (self.rect.w / sprite_size.x - 1.0).max(0.0) as usize;

        // Draw the edges
        // Horizontal
        let (top_sprite, top_src) = sprites[PanelDirection::Top as usize];
        let (bottom_sprite, bottom_src) = sprites[PanelDirection::Bottom as usize];

        let top_origin = cgmath::point2(self.rect.left() + sprite_size.x, self.rect.top());
        let bottom_origin = cgmath::point2(
            self.rect.left() + sprite_size.x,
            self.rect.bottom() - sprite_size.y,
        );

        for i in 0..number_of_hor {
            let offset = cgmath::vec2(sprite_size.x * i as f32, 0.0);
            let params = graphics::DrawParam::default()
                .dest(top_origin + offset)
                .src(*top_src);

            graphics::draw(ctx, top_sprite, params).map_err(err_here!())?;

            let params = graphics::DrawParam::default()
                .dest(bottom_origin + offset)
                .src(*bottom_src);

            graphics::draw(ctx, bottom_sprite, params).map_err(err_here!())?;
        }

        // Vertical
        let (left_sprite, left_src) = sprites[PanelDirection::Left as usize];
        let (right_sprite, right_src) = sprites[PanelDirection::Right as usize];

        let left_origin = cgmath::point2(self.rect.left(), self.rect.top() + sprite_size.y);
        let right_origin = cgmath::point2(
            self.rect.right() - sprite_size.x,
            self.rect.top() + sprite_size.y,
        );

        for i in 0..number_of_vert {
            let offset = cgmath::vec2(0.0, sprite_size.y * i as f32);
            let params = graphics::DrawParam::default()
                .dest(left_origin + offset)
                .src(*left_src);

            graphics::draw(ctx, left_sprite, params).map_err(err_here!())?;

            let params = graphics::DrawParam::default()
                .dest(right_origin + offset)
                .src(*right_src);

            graphics::draw(ctx, right_sprite, params).map_err(err_here!())?;
        }

        //Fill the panel
        let (center_sprite, center_src) = sprites[PanelDirection::Center as usize];
        let origin = cgmath::point2(self.rect.left(), self.rect.top()) + sprite_size;

        for i in 0..number_of_vert {
            for j in 0..number_of_hor {
                let params = graphics::DrawParam::default()
                    .dest(origin + cgmath::vec2(sprite_size.x * j as f32, sprite_size.y * i as f32))
                    .src(*center_src);

                graphics::draw(ctx, center_sprite, params).map_err(err_here!())?;
            }
        }

        Ok(())
    }
}
