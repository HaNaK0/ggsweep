use std::time::Duration;

use ggez::{graphics, Context, GameResult};

#[allow(unused_imports)]
use cgmath::prelude::*;
use log::trace;
use rand::prelude::*;

use crate::state::*;

//Constants
///The size of every grid square in pixels
const GRID_SIZE: f32 = 32.0;

/// The color of a square
const SQUARE_COLOR: (u8, u8, u8) = (0, 191, 255);
/// The color of a square when the mouse hovers over it
const SELECT_COLOR: (u8, u8, u8) = (100, 200, 255);

//Types
type Point2 = cgmath::Point2<f32>;
//type Vector2 = cgmath::Vector2<f32>;

/// The state of a square   
/// A square can either be closed and the bool states wetehr the player has set a flag on the square
/// or it can be open and then the number represents the number of neighboring mines
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum SquareState {
    Closed(bool),
    Open(u8),
}

#[allow(dead_code)]
pub struct GameState {
    game_size: (usize, usize),
    grid: Vec<SquareState>,
    mines: std::collections::HashSet<usize>,
    flag_image: graphics::Image,
    square: graphics::Mesh,
    timer: Duration,
    mouse_index: Option<i32>,
    mouse_press: Option<(ggez::input::mouse::MouseButton, i32)>,
}

impl GameState {
    pub fn new(
        ctx: &mut Context,
        game_size: (usize, usize),
        number_of_mines: usize,
    ) -> GameResult<Self> {
        let grid = vec![SquareState::Closed(false); game_size.0 * game_size.1];
        let mut mines = std::collections::HashSet::<usize>::new();
        let mut rng = rand::thread_rng();

        let flag_image = graphics::Image::new(ctx, "\\flag.png")?;
        let color = graphics::WHITE;

        while mines.len() < number_of_mines {
            mines.insert(rng.gen_range(0..grid.len()));
        }

        let rect = graphics::Rect::new(0.0, 0.0, GRID_SIZE, GRID_SIZE);
        let square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;

        Ok(GameState {
            game_size,
            grid,
            mines,
            flag_image,
            square,
            timer: Duration::new(0, 0),
            mouse_index: None,
            mouse_press: None,
        })
    }

    fn index_to_point(&self, i: usize) -> cgmath::Vector2<i32> {
        cgmath::Vector2::new((i % self.game_size.0) as i32, (i / self.game_size.0) as i32)
    }

    fn point_to_index(&self, point: cgmath::Vector2<i32>) -> usize {
        point.x as usize + point.y as usize * self.game_size.0
    }

    #[allow(dead_code)]
    fn count_neighbors(&self, i: usize) -> usize {
        let point = self.index_to_point(i);
        (-1..1)
            .map(|i| point + cgmath::Vector2::new(i, 1 - i))
            .filter(|v| {
                v.x >= 0
                    && v.y >= 0
                    && v.x < self.game_size.0 as i32
                    && v.y < self.game_size.1 as i32
            })
            .map(|v| self.point_to_index(v))
            .filter(|i| self.mines.contains(i))
            .count()
    }

    fn draw_squares(&self, ctx: &mut ggez::Context) -> GameResult<()> {
        for i in 0..self.grid.len() {
            let point = self.index_to_point(i);
            let v = GRID_SIZE * Point2::new(point.x as f32, point.y as f32);

            let mut params = graphics::DrawParam::new();
            params.dest = v.into();

            match self.grid[i] {
                SquareState::Closed(flag) => {
                    // if the mouse is pressed the square it was pressed on is the selected one
                    // otherwise it is the square that the mouse is over
                    params.color = if let Some((_, index)) = self.mouse_press {
                        if index == i as i32 {
                            SELECT_COLOR.into()
                        } else {
                            SQUARE_COLOR.into()
                        }
                    } else if let Some(index) = self.mouse_index {
                        if index == i as i32 {
                            SELECT_COLOR.into()
                        } else {
                            SQUARE_COLOR.into()
                        }
                    } else {
                        SQUARE_COLOR.into()
                    };

                    graphics::draw(ctx, &self.square, params)?;

                    if flag {
                        params.color = graphics::WHITE;
                        let scale = GRID_SIZE / self.flag_image.dimensions().w;
                        params.scale = ggez::mint::Vector2 { x: scale, y: scale };
                        graphics::draw(ctx, &self.flag_image, params)?;
                    }
                }
                SquareState::Open(_) => {}
            }
        }
        Ok(())
    }

    fn open(&mut self, index: i32) {
        self.grid[index as usize] = SquareState::Open(0)
    }
}

impl State for GameState {
    /// Main update
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<UpdateResult> {
        //update delta time
        let _dt = ggez::timer::delta(ctx);

        Ok(UpdateResult::Block)
    }

    ///Draw the playing grid
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.draw_squares(ctx)?;
        Ok(())
    }

    /// When the mouse is moved we update to the mouse index to the index of the square
    /// which the mouse is currently over
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> ggez::GameResult<EventResult> {
        // Convert the mouse position to a position in the playing grid
        let point = cgmath::Vector2::<i32>::new((x / GRID_SIZE) as i32, (y / GRID_SIZE) as i32);

        // Update the mouse index
        self.mouse_index = if point.x >= 0
            && point.y >= 0
            && point.x < self.game_size.0 as i32
            && point.y < self.game_size.1 as i32
        {
            Some(self.point_to_index(point) as i32)
        } else {
            None
        };

        Ok(EventResult::Block)
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) -> ggez::GameResult<EventResult> {
        // Get and unwrap mouse press and mouse index
        if let (Some((press_button, press_index)), Some(mouse_index)) =
            (self.mouse_press, self.mouse_index)
        {
            // If the mouse is released on the same square it was pressed it will active click
            if button == press_button && mouse_index == press_index {
                trace!("Mouse pressed on index {:?}", mouse_index);
                match button {
                    ggez::event::MouseButton::Left => self.open(press_index),
                    ggez::event::MouseButton::Right => {
                        // If right button is pressed we toggle the flag
                        if let SquareState::Closed(flagged) = self.grid[press_index as usize] {
                            self.grid[press_index as usize] = SquareState::Closed(!flagged)
                        }
                    }
                    ggez::event::MouseButton::Middle => {}
                    ggez::event::MouseButton::Other(_) => {}
                }
            }
        }

        self.mouse_press = None;

        Ok(EventResult::Block)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) -> ggez::GameResult<EventResult> {
        // When a mouse button is pressed set the mouse press to the button and index of the square the mouse is over
        if let Some(index) = self.mouse_index {
            self.mouse_press = Some((button, index))
        }
        Ok(EventResult::Block)
    }
}
