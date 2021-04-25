use crate::{
    err_here,
    error::LocatedError,
    sprite_sheet::{SheetInfo, SpriteSheet},
    state::{State, UpdateResult},
    ui::Element,
};
use ggez::{filesystem, Context};
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

        let position = cgmath::point2(100.0, 100.0);

        let button = Element::new_element(position, "button01", "hello world");

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
