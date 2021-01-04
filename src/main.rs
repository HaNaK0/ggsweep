use ggez::{event, ContextBuilder, GameResult, graphics};
use log::{info};

mod states;
use states::{GameState, MainState};

mod state;

fn main() -> GameResult {
	simple_logger::SimpleLogger::new()
		.with_level(log::LevelFilter::Warn)
		.with_module_level("ggsweep", log::LevelFilter::Trace)
		.init()
		.unwrap();

	let cb = ContextBuilder::new("Mine Sweeper", "HaNaK0");

	let (ctx, events_loop) = &mut cb.build()?;

	info!("{}", graphics::renderer_info(ctx)?);

	let initial_state = Box::new(GameState::new(ctx)?);
	let state = &mut MainState::new(initial_state)?;

	event::run(ctx, events_loop, state)
}

