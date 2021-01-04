use std::{any::TypeId, collections::VecDeque};

use ggez::{event, graphics};
use ggez::nalgebra as na;

use crate::state;

/// The main state that contains all other states in a state stack
pub struct MainState {
	state_stack: VecDeque<Box<dyn state::State>>
}


impl MainState {
	/// Creates and loads the main state
	pub fn new(initial_state: Box<dyn state::State>) -> ggez::GameResult<MainState> {
		let state_stack:VecDeque<Box<dyn state::State>> = vec![initial_state].into();

		let state = MainState {
			state_stack,
		};

		Ok(state)
	}
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		for (i, state) in self.state_stack.iter().enumerate() {
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
		let states: Vec<&Box<dyn state::State>> = self.state_stack.iter()
			.scan(true, |cont, state| {
				if *cont {
					*cont = state.let_through_draw();
					Some(state)
				} else {
					None
				}
			})
			.collect();
		
		for state in states.iter().rev() {
			state.draw(ctx)?;
		}

		Ok(())
    }
}