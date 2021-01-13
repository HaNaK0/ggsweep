use std::time::Duration;

use ggez::{graphics, Context, GameResult};
use ggez::nalgebra as na;

use rand::prelude::*;

use log::{trace, info};

use crate::state::*;

const GRID_SIZE: f32 = 32.0;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

/// The state of a square   
/// A square can either be closed and the bool states wetehr the player has set a flag on the square
/// or it can be open and then the number represents the number of neighboring mines
#[derive(Clone, Debug, PartialEq)]
enum SquareState {
	Closed(bool),
	Open(u8),
}

pub struct GameState {
	game_size: (usize, usize),
	grid: Vec<SquareState>,
	mines: std::collections::HashSet<usize>,
	flag_image: graphics::Image,
	timer: Duration,
	testing_dest: Point2,
}

impl GameState {
	pub fn new(ctx: &mut Context, game_size: (usize, usize), number_of_mines: usize) -> GameResult<Self> {

		let grid = vec![SquareState::Closed(false); game_size.0 * game_size.1];
		let mut mines = std::collections::HashSet::<usize>::new();
		let mut rng = rand::thread_rng();

		
		let flag_image = graphics::Image::new(ctx, "\\flag.png")?;

		while mines.len() < number_of_mines {
			mines.insert(rng.gen_range(0..grid.len()));
		}

		let testing_dest = Point2::new(0.0, 0.0);
		Ok(GameState {game_size, grid, mines, flag_image, timer: Duration::new(0, 0), testing_dest})
	}

	fn index_to_point(& self, i: usize) -> na::Vector2<i32> {
		na::Vector2::new((i % self.game_size.0) as i32, (i / self.game_size.0) as i32)
	}

	fn point_to_index(& self, point: na::Vector2<i32>) -> usize {
		point.x as usize + point.y as usize * self.game_size.0
	}

	fn count_neighbors(& self, i : usize) -> usize {
		let point = self.index_to_point(i);
		(-1..1)
			.map(|i| {
				point + na::Vector2::new(i, 1 - i)
			}).filter(|v| {
				v.x >= 0 && v.y >= 0 && v.x < self.game_size.0 as i32 && v.y < self.game_size.1 as i32
			}).map(|v| self.point_to_index(v))
			.filter(|i| self.mines.contains(i))
			.count()
	}

	fn draw_squares(& self, ctx: &mut ggez::Context) -> GameResult<()> {
		let color = (0.1, 0.5, 1.0, 1.0).into();
		

		for i in 0..self.grid.len() {
			let v = Point2::new((i % self.game_size.0 ) as f32 * GRID_SIZE, (i / self.game_size.0 ) as f32 * GRID_SIZE);
			
			let rect = graphics::Rect::new(v.x, v.y, GRID_SIZE, GRID_SIZE);
			let square = graphics::Mesh::new_rectangle(
				ctx,
				graphics::DrawMode::fill(),
				rect.clone(),
				color
			)?;

			let square_border = graphics::Mesh::new_rectangle(
				ctx,
				graphics::DrawMode::stroke(3.0),
				rect.clone(),
				(7.0, 7.0, 7.0, 0.7).into()
			)?;

			let params = graphics::DrawParam::new();
			params.dest(v);
			
			match self.grid[i] {
			    SquareState::Closed(flag) => {
					graphics::draw(ctx, &square, params)?;

					if flag {
						
					}
				}
			    SquareState::Open(_) => {}
			}

			graphics::draw(ctx, &square_border, params)?;
		}
		Ok(())
	}
}

impl State for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<UpdateResult> {
		let dt = ggez::timer::delta(ctx);
		self.timer += dt;

		if self.timer.as_secs() > 1 {
			let mut rng = rand::thread_rng();
			let i = rng.gen_range(0..self.grid.len());

			if let SquareState::Closed(b) = self.grid[i] {
				self.grid[i] = SquareState::Closed(!b)
			}
			
			self.timer = Duration::new(0, 0);
		}

		self.testing_dest += Vector2::new(1.0, 1.0) * dt.as_secs_f32();

		Ok(UpdateResult::Block)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);

		self.draw_squares(ctx)?;
		
		let params = graphics::DrawParam::new();
		params.dest(self.testing_dest);
		graphics::draw(ctx, &self.flag_image, params)?;

		graphics::present(ctx)?;

		Ok(())
    }
}
