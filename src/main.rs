use ggez::{event, ContextBuilder, GameResult, graphics};
use log::{info};


mod states;
use states::MainState;

fn main() -> GameResult {
	simple_logger::SimpleLogger::new()
		.with_level(log::LevelFilter::Warn)
		.with_module_level("ggsweep", log::LevelFilter::Trace)
		.init()
		.unwrap();

	let cb = ContextBuilder::new("Mine Sweeper", "HaNaK0");

	let (ctx, events_loop) = &mut cb.build()?;

	info!("{}", graphics::renderer_info(ctx)?);
	let state = &mut MainState::new()?;

	event::run(ctx, events_loop, state)
}

