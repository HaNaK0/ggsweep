use crate::err_here;
use crate::error::{LocatedError, WrappedError};
use ggez::{filesystem, graphics, Context};
use log::error;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashMap;

/// The info for the sprite sheet loaded from the ron file
#[derive(Deserialize, Debug, Clone)]
pub struct SheetInfo {
    name: String,
    files: HashMap<String, String>,
    sprites: Vec<SpriteInfo>,
}

impl SheetInfo {
    /// Create a [SpriteSheet] from this sheet info that will take ownership of this sheet info.
    pub fn into_sprite_sheet(
        self,
        ctx: &mut Context,
        file_name: &str,
    ) -> Result<SpriteSheet, LocatedError> {
        SpriteSheet::new(ctx, self, file_name)
    }
}

/// Info for a sprite in a sprite sheet
#[derive(Deserialize, Debug, Clone)]
struct SpriteInfo {
    name: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

pub fn test_load(ctx: &mut Context) {
    let file = filesystem::open(ctx, "/Ui/Spritesheet/colored_sheet.ron");
    if let Result::Err(e) = file {
        error!("Failed to load file: {:?}", e);
        panic!()
    }
    let file = file.unwrap();

    let sheet_info: Result<SheetInfo, ron::Error> = from_reader(file);

    if let Result::Err(e) = sheet_info {
        error!("Failed to parse file: {:?}", e);
        panic!()
    }
}

/// A sprite sheet keeping track of different sprites in the same image file
#[derive(Debug)]
pub struct SpriteSheet {
    sheet_info: SheetInfo,
    image: graphics::Image,
    sprite_rects: HashMap<String, graphics::Rect>,
}

impl SpriteSheet {
    /// Create an new `SpriteSheet`    
    fn new(
        ctx: &mut Context,
        sheet_info: SheetInfo,
        file_name: &str,
    ) -> Result<Self, LocatedError> {
        let image_path = if let Some(path) = sheet_info.files.get(file_name) {
            Ok(path)
        } else {
            let e = WrappedError::SheetError(format!(
                "Sheet{} does not have a link to a file called {}",
                sheet_info.name, file_name
            ));
            Err(LocatedError::new(
                e,
                format!("at line {} in {}", line!(), file!()),
            ))
        }?;
        let image = graphics::Image::new(ctx, image_path).map_err(err_here!())?;

        let sprite_iter = sheet_info.sprites.iter().map(|s| {
            let rect = graphics::Rect::new(
                s.x as f32 / image.dimensions().w,
                s.y as f32 / image.dimensions().h,
                s.width as f32 / image.dimensions().w,
                s.height as f32 / image.dimensions().h,
            );
            (s.name.clone(), rect)
        });

        let sprite_rects = sprite_iter.collect();

        Ok(Self {
            image,
            sprite_rects,
            sheet_info,
        })
    }

    pub fn get_sprite_and_rect(
        &self,
        sprite_name: &str,
    ) -> Option<(&graphics::Image, &graphics::Rect)> {
        if let Some(r) = self.sprite_rects.get(sprite_name) {
            Some((&self.image, r))
        } else {
            None
        }
    }

    pub fn get_sprite_pixel_size(&self, sprite_name: &str) -> Option<cgmath::Vector2<f32>> {
        self.sprite_rects.get(sprite_name).map(|r| {
            cgmath::vec2(
                r.w * self.image.dimensions().w,
                r.h * self.image.dimensions().h,
            )
        })
    }
}
