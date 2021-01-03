use ggez::{event, graphics};
use ggez::nalgebra as na;


const GRID_SIZE: f32 = 32.0;

pub struct MainState {
	square_color: graphics::Color,
	square_origin: na::Point2<f32>,
}

impl MainState {
	pub fn new() -> ggez::GameResult<MainState> {
		let state = MainState {
			square_color: graphics::Color::new(0.0, 0.5, 1.0, 1.0),
			square_origin: na::Point2::new(400.0, 400.0),
		};
		Ok(state)
	}
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		graphics::clear(ctx, graphics::BLACK);

		let rect = graphics::Rect::new(self.square_origin.x, self.square_origin.y, GRID_SIZE, GRID_SIZE);
		let square = graphics::Mesh::new_rectangle(
			ctx, 
			graphics::DrawMode::fill(), 
			rect, 
			self.square_color,
		)?;

		graphics::draw(ctx, &square, graphics::DrawParam::default())?;

		graphics::present(ctx)
    }
}