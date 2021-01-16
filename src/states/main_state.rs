use std::collections::VecDeque;
use ggez::{event, graphics, Context};

use crate::state;

/// The main state that contains all other states in a state stack
pub struct MainState {
	state_stack: VecDeque<Box<dyn state::State>>,
	clear_color: graphics::Color,
}


impl MainState {
	/// Creates and loads the main state
	pub fn new(initial_state: Box<dyn state::State>, clear_color: graphics::Color) -> ggez::GameResult<MainState> {
		let state_stack:VecDeque<Box<dyn state::State>> = vec![initial_state].into();

		let state = MainState {
			state_stack,
			clear_color,
		};

		Ok(state)
	}
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		for (i, state) in self.state_stack.iter_mut().enumerate() {
			match state.update(ctx)? {
			    state::UpdateResult::LetThrough => {}
			    state::UpdateResult::Block => break,
			    state::UpdateResult::Swap(new_state) => {
					self.state_stack[i] = new_state;
					break;
				}
			    state::UpdateResult::Push(new_state) => {
					self.state_stack.push_front(new_state);
					break;
				}
				state::UpdateResult::Pop => {
					self.state_stack.pop_front();
					break;
				}
			}
		}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		let mut index = 0;

		for i in 1..self.state_stack.len() {
			if !self.state_stack[i - 1].let_through_draw() {
				break;
			}
			index = i;
		}

		graphics::clear(ctx, self.clear_color);

		for i in (0..index+1).rev() {
			self.state_stack[i].draw(ctx)?;
		}

		graphics::present(ctx)?;

		Ok(())
	}
	
	fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
		
	}
}