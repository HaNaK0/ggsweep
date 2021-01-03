use ggez::{event, ContextBuilder, GameResult, graphics};


mod states;
use states::MainState;

fn main() -> GameResult {
	let cb = ContextBuilder::new("Mine Sweeper", "HaNaK0");

	let (ctx, events_loop) = &mut cb.build()?;

	println!("{}", graphics::renderer_info(ctx)?);
	let state = &mut MainState::new()?;

	event::run(ctx, events_loop, state)
}

