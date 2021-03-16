use std::{env, path};

use ggez::{event, graphics, ContextBuilder};
use log::info;

use ggsweep::{err_here, error::LocatedError, states};

/// Main function of the pipeline generating assets for the game
fn main() -> Result<(), LocatedError> {
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

    // Create the context builder
    let cb = ContextBuilder::new("ggsweep Pipeline", "HaNaK0").add_resource_path(resuource_dir);

    let (ctx, events_loop) = &mut cb.build().map_err(err_here!())?;

    info!("{}", graphics::renderer_info(ctx).map_err(err_here!())?);

    let initial_state = Box::new(states::PipelineState::new("/GenConfig.ron", ctx)?);
    let state = &mut states::MainState::new(initial_state, graphics::Color::from_rgb(38, 38, 38))
        .map_err(err_here!())?;

    event::run(ctx, events_loop, state).map_err(err_here!())?;
    Ok(())
}
