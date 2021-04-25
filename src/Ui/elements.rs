#[allow(unused_imports)]
use cgmath::prelude::*;

use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect, Text, WHITE},
    Context, GameResult,
};

use log::error;

use crate::sprite_sheet::SpriteSheet;

pub struct Element {
    position: cgmath::Point2<f32>,
    sprite_name: Option<String>,
    label: Option<Text>,
}

impl Element {
    /// Draw the ui element
    pub fn draw(&self, ctx: &mut Context, sprite_sheet: &SpriteSheet) -> GameResult {
        if let Some(sprite_name) = &self.sprite_name {
            if let Some((image, rect)) = sprite_sheet.get_sprite_and_rect(sprite_name) {
                let scale = cgmath::Vector2 { x: 1.0, y: 1.0 };

                let draw_params = DrawParam {
                    src: *rect,
                    dest: self.position.into(),
                    rotation: 0.0,
                    scale: scale.into(),
                    offset: cgmath::Point2::new(0.0, 0.0).into(),
                    color: WHITE,
                };

                graphics::draw(ctx, image, draw_params)?;
            } else {
                error!("SpriteSheet does not have any sprite named {}", sprite_name);
                let draw_params = DrawParam {
                    src: Rect::new(0.0, 0.0, 1.0, 1.0),
                    dest: self.position.into(),
                    rotation: 0.0,
                    scale: cgmath::Vector2::new(1.0, 1.0).into(),
                    offset: cgmath::Point2::new(0.0, 0.0).into(),
                    color: WHITE,
                };

                let rectangle = Mesh::new_rectangle(
                    ctx,
                    DrawMode::Fill(FillOptions::DEFAULT),
                    graphics::Rect::new(0.0, 0.0, 32.0, 32.0),
                    Color::from_rgb(255, 0, 255),
                )?;

                graphics::draw(ctx, &rectangle, draw_params)?;
            }
        }

        if let Some(label) = &self.label {
            // If this element has a sprite draw the label in the middle. If this does not have a sprite draw the label att the elements position
            let label_pos= if let Some(sprite_name) = &self.sprite_name {
                let sprite_size = sprite_sheet.get_sprite_pixel_size(sprite_name.as_str());
                let label_size = cgmath::vec2(label.width(ctx) as f32, label.height(ctx) as f32);
                self.position + sprite_size.unwrap() / 2.0 - label_size / 2.0
            } else {
                self.position
            };

            let draw_params = DrawParam {
                src: Rect::new(0.0, 0.0, 1.0, 1.0),
                dest: label_pos.into(),
                rotation: 0.0,
                scale: cgmath::Vector2::new(1.0, 1.0).into(),
                offset: cgmath::point2(0.0, 0.0).into(),
                color: WHITE,
            };

            graphics::draw(ctx, label, draw_params)?;
        }

        Ok(())
    }

    /// Create a new element with sprite and label
    pub fn new_element(position: cgmath::Point2<f32>, sprite_name: &str, label: &str) -> Self {
        Element {
            position,
            sprite_name: Some(sprite_name.to_string()),
            label: Some(graphics::Text::new(label)),
        }
    }

    /// Create a new element without any text on it
    pub fn new_image_element(position: cgmath::Point2<f32>, sprite_name: &str) -> Self {
        Element {
            position,
            sprite_name: Some(sprite_name.to_string()),
            label: None,
        }
    }

    /// Create a new label element that only has text
    pub fn new_label_element(position: cgmath::Point2<f32>, label: &str) -> Self {
        Element {
            position,
            sprite_name: None,
            label: Some(graphics::Text::new(label)),
        }
    }
}
