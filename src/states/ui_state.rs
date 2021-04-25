use crate::{
    err_here,
    error::LocatedError,
    sprite_sheet::{SheetInfo, SpriteSheet},
    state::{State, UpdateResult},
    ui::Element,
};
use ggez::{filesystem, graphics, Context};
use ron::de::from_reader;

pub struct UiState {
    sprite_sheet: SpriteSheet,
    elements: Vec<Element>,
    is_menu: bool,
}

impl UiState {
    pub fn create_main_menu_state(ctx: &mut Context) -> Result<Self, LocatedError> {
        let sprite_sheet = {
            let file =
                filesystem::open(ctx, "/Ui/Spritesheet/colored_sheet.ron").map_err(err_here!())?;
            let sheet_info: SheetInfo = from_reader(file).map_err(err_here!())?;
            sheet_info.into_sprite_sheet(ctx, "blue")?
        };

        let (_, sprite_rect) = sprite_sheet.get_sprite_and_rect("button01").unwrap();

        let target_rect = graphics::Rect {
            x: 100.0,
            y: 100.0,
            w: sprite_rect.w,
            h: sprite_rect.h,
        };

        let button = Element::new_image_element(target_rect, "button01");

        Ok(UiState {
            sprite_sheet,
            elements: vec![button],
            is_menu: true,
        })
    }
}

impl State for UiState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<crate::state::UpdateResult> {
        if self.is_menu {
            Ok(UpdateResult::Block)
        } else {
            Ok(UpdateResult::LetThrough)
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        for element in &self.elements {
            element.draw(ctx, &self.sprite_sheet)?;
        }

        Ok(())
    }
}
