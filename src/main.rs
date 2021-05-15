use std::{env, path};

use ggez::{conf::WindowMode, event, graphics, ContextBuilder};
use ggsweep::{config::GameConfig, err_here, error::LocatedError, states::GameState};
use log::info;

use ggsweep::states::MainState;

fn main() -> Result<(), LocatedError> {
    // Start the logger
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_module_level("ggez", log::LevelFilter::Trace)
        .with_module_level("ggsweep", log::LevelFilter::Trace)
        .init()
        .unwrap();

    //setup the resource path
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        info!("Resource directory set to {:?}", path);
        path
    } else {
        info!("Failed to find cargo manifest directory will look fo resources in \"./resources\"");
        path::PathBuf::from("./resources")
    };

    // Create the context builder
    let cb = ContextBuilder::new("Mine Sweeper", "HaNaK0").add_resource_path(resource_dir);

    // Build the context
    let (ctx, events_loop) = &mut cb.build().map_err(err_here!())?;

    info!("{}", graphics::renderer_info(ctx).map_err(err_here!())?);

    //Load config
    let game_config_file = ggez::filesystem::open(ctx, "\\config.ron").map_err(err_here!())?;
    let game_config: GameConfig = ron::de::from_reader(game_config_file).map_err(err_here!())?;

    //Set window mode
    let mode = WindowMode::default().dimensions(
        game_config.game_size.0 as f32 * game_config.square_size,
        game_config.game_size.1 as f32 * game_config.square_size,
    );
    graphics::set_mode(ctx, mode).map_err(err_here!())?;

    //Set the screen coordinates
    let screen_rect = graphics::Rect::new(
        0.0,
        0.0,
        game_config.game_size.0 as f32 * game_config.square_size,
        game_config.game_size.1 as f32 * game_config.square_size,
    );
    graphics::set_screen_coordinates(ctx, screen_rect).map_err(err_here!())?;

    // Set the initial state
    let initial_state = Box::new(GameState::new(ctx, game_config).map_err(err_here!())?);
    let state = &mut MainState::new(initial_state, graphics::Color::from_rgb(38, 38, 38))
        .map_err(err_here!())?;

    // Run
    event::run(ctx, events_loop, state).map_err(err_here!())
}
