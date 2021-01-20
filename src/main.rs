use std::{env, path};

use ggez::{event, graphics, ContextBuilder, GameResult};
use log::info;

mod states;
use states::{GameState, MainState};

mod state;

fn main() -> GameResult {
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

    let (ctx, events_loop) = &mut cb.build()?;

    info!("{}", graphics::renderer_info(ctx)?);

    let initial_state = Box::new(GameState::new(ctx, (10, 10), 5)?);
    let state = &mut MainState::new(initial_state, graphics::Color::from_rgb(38, 38, 38))?;

    event::run(ctx, events_loop, state)
}
