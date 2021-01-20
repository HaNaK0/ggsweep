use std::time::Duration;

use ggez::{graphics, Context, GameResult};
use cgmath::prelude::*;

use rand::prelude::*;

use log::{trace, info};

use crate::state::*;

const GRID_SIZE: f32 = 32.0;
const SQUARE_COLOR: (u8, u8, u8) = (0, 191, 255);
const SELECT_COLOR: (u8, u8, u8) = (100, 200, 255);

type Point2 = cgmath::Point2<f32>;
type Vector2 = cgmath::Vector2<f32>;

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
	square: graphics::Mesh,
	timer: Duration,
	mouse_index: Option<i32>,
}

impl GameState {
	pub fn new(ctx: &mut Context, game_size: (usize, usize), number_of_mines: usize) -> GameResult<Self> {

		let grid = vec![SquareState::Closed(false); game_size.0 * game_size.1];
		let mut mines = std::collections::HashSet::<usize>::new();
		let mut rng = rand::thread_rng();

		
		let flag_image = graphics::Image::new(ctx, "\\flag.png")?;
		let color = graphics::WHITE;

		while mines.len() < number_of_mines {
			mines.insert(rng.gen_range(0..grid.len()));
		}

		let rect = graphics::Rect::new(0.0, 0.0, GRID_SIZE, GRID_SIZE);
		let square = graphics::Mesh::new_rectangle(
			ctx,
			graphics::DrawMode::fill(),
			rect.clone(),
			color
		)?;

		Ok(GameState {game_size, grid, mines, flag_image, square, timer: Duration::new(0, 0), mouse_index: None})
	}

	fn index_to_point(& self, i: usize) -> cgmath::Vector2<i32> {
		cgmath::Vector2::new((i % self.game_size.0) as i32, (i / self.game_size.0) as i32)
	}

	fn point_to_index(& self, point: cgmath::Vector2<i32>) -> usize {
		point.x as usize + point.y as usize * self.game_size.0
	}

	fn count_neighbors(& self, i : usize) -> usize {
		let point = self.index_to_point(i);
		(-1..1)
			.map(|i| {
				point + cgmath::Vector2::new(i, 1 - i)
			}).filter(|v| {
				v.x >= 0 && v.y >= 0 && v.x < self.game_size.0 as i32 && v.y < self.game_size.1 as i32
			}).map(|v| self.point_to_index(v))
			.filter(|i| self.mines.contains(i))
			.count()
	}

	fn draw_squares(& self, ctx: &mut ggez::Context) -> GameResult<()> {
		for i in 0..self.grid.len() {
			let point = self.index_to_point(i);
			let v = GRID_SIZE * Point2::new(point.x as f32, point.y as f32);

			let mut params = graphics::DrawParam::new();
			params.dest = v.into();
			
			match self.grid[i] {
			    SquareState::Closed(flag) => {
					if let Some(index) = self.mouse_index {
						params.color = if index == i as i32 {
							SELECT_COLOR.into()
						} else {
							SQUARE_COLOR.into()
						}
					} else {
						params.color = SQUARE_COLOR.into()
					}

					graphics::draw(ctx, &self.square, params)?;

					if flag {
						let scale = GRID_SIZE / self.flag_image.dimensions().w;
						params.scale = ggez::mint::Vector2 {x: scale, y: scale};
						graphics::draw(ctx, &self.flag_image, params)?;
					}
				}
			    SquareState::Open(_) => {}
			}
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

		Ok(UpdateResult::Block)
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
		self.draw_squares(ctx)?;
		Ok(())
	}
	
	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> ggez::GameResult<EventResult> {
		let point = cgmath::Vector2::<i32>::new((x / GRID_SIZE) as i32, (y / GRID_SIZE) as i32);
		
		self.mouse_index = if point.x >= 0 && point.y >= 0 && point.x < self.game_size.0 as i32 && point.y < self.game_size.1 as i32{
			Some(self.point_to_index(point) as i32)
		} else {
			None
		};

		Ok(EventResult::Block)
	}
}
