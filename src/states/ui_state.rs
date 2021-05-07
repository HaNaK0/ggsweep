use crate::{err_here, error::LocatedError, sprite_sheet::{SheetInfo, SpriteSheet}, state::{EventResult, State, UpdateResult}, ui::{Element, Panel}};
use ggez::{filesystem, graphics, Context, conf};
use ron::de::from_reader;

pub struct UiState {
    sprite_sheet: SpriteSheet,
    panel: Panel,
    elements: Vec<Element>,
    is_menu: bool,
    quit: bool,
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
            quit: false,
        })
    }

    pub fn create_game_over_state(ctx: &mut Context, has_won: bool) -> Result<Self, LocatedError>{
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

        let panel_rect = graphics::Rect::new_i32(32,320 / 2 - 32, 320-64, 64);

        let panel = Panel::new(panel_rect, panel_sprites);

        let font = graphics::Font::new(ctx, "/Kenney_Future.ttf").map_err(err_here!())?;
        let scale = graphics::Scale::uniform(32.0);

        let game_over_text = if has_won {
            Element::new_element(cgmath::point2(320.0 / 2.0, 320.0 / 2.0))
                .set_label("You Won", &font, &scale)
        } else {
            Element::new_element(cgmath::point2( 64.0, 320.0 / 2.0 - 16.0))
                .set_label("You Lost", &font, &scale)
        };

        Ok(UiState {
            sprite_sheet,
            elements: vec![game_over_text],
            is_menu: true,
            panel,
            quit: false,
        })
    }
}

impl State for UiState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<UpdateResult, LocatedError> {
        if self.quit {
            return Ok(UpdateResult::Pop);
        }

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

    fn let_through_draw(&mut self) -> bool {
        true
    }
    
    fn mouse_button_down_event(&mut self, _ctx: &mut ggez::Context, _button: ggez::input::mouse::MouseButton, _x: f32, _y: f32) -> ggez::GameResult<EventResult> {
        Ok(EventResult::Block)
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, _button: ggez::input::mouse::MouseButton, _x: f32, _y: f32) -> ggez::GameResult<EventResult> {
        self.quit = true;
        Ok(EventResult::Block)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut ggez::Context, _x: f32, _y: f32, _dx: f32, _dy: f32) -> ggez::GameResult<EventResult> {
        Ok(EventResult::Block)
    } 
}
