use ggez::{event, graphics, Context, GameResult};
use state::EventResult;
use std::collections::VecDeque;

use log::error;

use crate::state;

/// The main state that contains all other states in a state stack
pub struct MainState {
    state_stack: VecDeque<Box<dyn state::State>>,
    clear_color: graphics::Color,
    event_result: GameResult<()>,
}

impl MainState {
    /// Creates and loads the main state
    pub fn new(
        initial_state: Box<dyn state::State>,
        clear_color: graphics::Color,
    ) -> ggez::GameResult<MainState> {
        let state_stack: VecDeque<Box<dyn state::State>> = vec![initial_state].into();

        let state = MainState {
            state_stack,
            clear_color,
            event_result: Ok(()),
        };

        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        match &self.event_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e.clone());
            }
        }

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

        for i in (0..index + 1).rev() {
            self.state_stack[i].draw(ctx)?;
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        for state in &mut self.state_stack {
            match state.mouse_motion_event(ctx, x, y, dx, dy) {
                Ok(r) => {
                    if r == EventResult::Block {
                        break;
                    }
                }
                Err(e) => {
                    error!("Encountered error in mouse motion event: {:?}", e);
                    self.event_result = Err(e);
                    break;
                }
            }
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        for state in &mut self.state_stack {
            match state.mouse_button_down_event(ctx, button, x, y) {
                Ok(r) => {
                    if r == EventResult::Block {
                        break;
                    }
                }
                Err(e) => {
                    error!("Encountered error in mouse button down: {:?}", e);
                    self.event_result = Err(e);
                    break;
                }
            }
        }
    }

    /// Goes through the states and calls the mouse button up event until a state blocks it
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        for state in &mut self.state_stack {
            match state.mouse_button_up_event(ctx, button, x, y) {
                Ok(r) => {
                    if r == EventResult::Block {
                        break;
                    }
                }
                Err(e) => {
                    error!("Encountered error in mouse button down: {:?}", e);
                    self.event_result = Err(e);
                    break;
                }
            }
        }
    }
}
