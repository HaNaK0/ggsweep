use crate::{
    err_here,
    error::LocatedError,
    sprite_sheet::{SheetInfo, SpriteSheet},
    state::{State, UpdateResult},
    ui::{Element, Panel},
};
use ggez::{filesystem, graphics, Context};
use ron::de::from_reader;

pub struct UiState {
    sprite_sheet: SpriteSheet,
    panel: Panel,
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

        let panel_sprites = [
            "panel_top_left".to_string(),
            "panel_top".to_string(),
            "panel_top_right".to_string(),
            "panel_left".to_string(),
            "panel_center".to_string(),
            "panel_right".to_string(),
            "panel_bottom_left".to_string(),
            "panel_bottom".to_string(),
            "panel_bottom_right".to_string(),
        ];

        let panel_rect = graphics::Rect::new_i32(32,32, 244, 244);

        let panel = Panel::new(panel_rect, panel_sprites);

        Ok(UiState {
            sprite_sheet,
            elements: vec![],
            is_menu: true,
            panel,
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
        self.panel.draw(ctx, &self.sprite_sheet).unwrap();

        for element in &self.elements {
            element.draw(ctx, &self.sprite_sheet)?;
        }

        Ok(())
    }
}
