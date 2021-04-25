#[allow(unused_imports)]
use cgmath::prelude::*;
use ggez::{
    graphics::{self, DrawParam, Rect, Text, WHITE},
    Context, GameResult,
};
use graphics::{Color, DrawMode, FillOptions, Mesh};
use log::error;

use crate::sprite_sheet::SpriteSheet;

pub struct Element {
    target_rect: Rect,
    sprite_name: Option<String>,
    label: Option<Text>,
}

impl Element {
    pub fn draw(&self, ctx: &mut Context, sprite_sheet: &SpriteSheet) -> GameResult {
        if let Some(sprite_name) = &self.sprite_name {
            if let Some((image, rect)) = sprite_sheet.get_sprite_and_rect(sprite_name) {
                let scale = cgmath::Vector2 {
                    x: self.target_rect.w / rect.w,
                    y: self.target_rect.h / rect.h,
                };

                let draw_params = DrawParam {
                    src: *rect,
                    dest: cgmath::point2(self.target_rect.x, self.target_rect.y).into(),
                    rotation: 0.0,
                    scale: scale.into(),
                    offset: cgmath::Point2::new(0.0, 0.0).into(),
                    color: WHITE,
                };

                graphics::draw(ctx, image, draw_params)?;
            } else {
                error!("SpriteSheet does not have any sprite named {}", sprite_name);
                let draw_params = DrawParam {
                    src: Rect::new(0.0, 0.0, self.target_rect.w, self.target_rect.h),
                    dest: cgmath::point2(self.target_rect.x, self.target_rect.y).into(),
                    rotation: 0.0,
                    scale: cgmath::Vector2::new(1.0, 1.0).into(),
                    offset: cgmath::Point2::new(0.0, 0.0).into(),
                    color: WHITE,
                };

                let rectangle = Mesh::new_rectangle(
                    ctx,
                    DrawMode::Fill(FillOptions::DEFAULT),
                    self.target_rect,
                    Color::from_rgb(255, 0, 255),
                )?;

                graphics::draw(ctx, &rectangle, draw_params)?;
            }
        }

        if let Some(label) = &self.label {
            let label_pos = cgmath::point2(
                self.target_rect.x + self.target_rect.w / 2.0,
                self.target_rect.y + self.target_rect.h / 2.0,
            );

            let draw_params = DrawParam {
                src: Rect::new(0.0, 0.0, self.target_rect.w, self.target_rect.h),
                dest: label_pos.into(),
                rotation: 0.0,
                scale: cgmath::Vector2::new(1.0, 1.0).into(),
                offset: cgmath::Point2::new(0.5, 0.5).into(),
                color: WHITE,
            };

            graphics::draw(ctx, label, draw_params)?;
        }

        Ok(())
    }

    pub fn new_element(target_rect: graphics::Rect, sprite_name: &str, label: &str) -> Self {
        Element {
            target_rect,
            sprite_name: Some(sprite_name.to_string()),
            label: Some(graphics::Text::new(label)),
        }
    }

    pub fn new_image_element(target_rect: graphics::Rect, sprite_name: &str) -> Self {
        Element {
            target_rect,
            sprite_name: Some(sprite_name.to_string()),
            label: None,
        }
    }
}
