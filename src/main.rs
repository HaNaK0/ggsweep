use std::{env, path};

use ggez::{event, graphics, ContextBuilder};
use ggsweep::{err_here, error::LocatedError};
use log::info;

use ggsweep::states::{GameState, MainState};

fn main() -> Result<(), LocatedError> {
    // Start the logger
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
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

    let cb = ContextBuilder::new("Mine Sweeper", "HaNaK0").add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build().map_err(err_here!())?;

    info!("{}", graphics::renderer_info(ctx).map_err(err_here!())?);

    let game_config_file = ggez::filesystem::open(ctx, "\\config.ron").map_err(err_here!())?;
    let game_config = ron::de::from_reader(game_config_file).map_err(err_here!())?;

    let initial_state = Box::new(GameState::new(ctx, game_config).map_err(err_here!())?);
    let state = &mut MainState::new(initial_state, graphics::Color::from_rgb(38, 38, 38))
        .map_err(err_here!())?;

    event::run(ctx, events_loop, state).map_err(err_here!())
}
