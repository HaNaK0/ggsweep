use std::{env, path};

use error::Error;
use ggez::{event, graphics, ContextBuilder};
use log::info;

mod states;
use states::{GameState, MainState};

mod error;
mod game_config;
mod state;

fn main() -> Result<(), error::Error> {
    // Start the logger
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_module_level("ggsweep", log::LevelFilter::Trace)
        .init()
        .unwrap();

    //setup the resource path
    let resuource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        info!("Resource directory set to {:?}", path);
        path
    } else {
        info!("Failed to find cargo manfiest directory will look fo resources in \"./resources\"");
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("Mine Sweeper", "HaNaK0").add_resource_path(resuource_dir);

    let (ctx, events_loop) = &mut cb.build().map_err(Error::GameError)?;

    info!(
        "{}",
        graphics::renderer_info(ctx).map_err(Error::GameError)?
    );

    let game_config_file = ggez::filesystem::open(ctx, "\\config.ron").map_err(Error::GameError)?;
    let game_config =
        game_config::GameConfig::load_from_file(game_config_file).map_err(Error::RonError)?;

    let initial_state = Box::new(GameState::new(ctx, game_config).map_err(Error::GameError)?);
    let state = &mut MainState::new(initial_state, graphics::Color::from_rgb(38, 38, 38))
        .map_err(Error::GameError)?;

    event::run(ctx, events_loop, state).map_err(Error::GameError)
}
