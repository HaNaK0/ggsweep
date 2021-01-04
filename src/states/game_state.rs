use ggez::{graphics, Context, GameResult};
//use ggez::nalgebra as na;

use crate::state::*;

const GRID_SIZE: f32 = 32.0;

//type Point2 = na::Point2<f32>;

pub struct GameState {
	square: graphics::Mesh,
}

impl GameState {
	pub fn new(ctx: &mut Context) -> GameResult<Self> {
		let color = (0.0, 0.4, 1.0, 1.0).into();
		let rect = graphics::Rect::new(200.0, 200.0, GRID_SIZE, GRID_SIZE);
		let square = graphics::Mesh::new_rectangle(
			ctx,
			graphics::DrawMode::fill(),
			rect,
			color
		)?;
		

		Ok(GameState {square})
	}
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<UpdateResult> {
        Ok(UpdateResult::Block)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);

		graphics::draw(ctx, &self.square, graphics::DrawParam::default())?;

		graphics::present(ctx)?;

		Ok(())
    }
}
