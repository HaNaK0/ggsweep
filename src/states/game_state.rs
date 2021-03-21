use std::collections;

//use cgmath::prelude::*;
use ggez::{graphics, Context, GameResult};

#[allow(unused_imports)]
use cgmath::prelude::*;
use log::trace;
use rand::prelude::*;

use crate::{config::GameConfig, state::*};

//Types
/// # Point2
/// Used for points
type Point2 = cgmath::Point2<f32>;
/// # Index Type
/// The type used for indices
type IndexType = usize;

/// # Square State
/// The state of a square   
/// A square can either be closed and the bool states wether the player has set a flag on the square
/// or it can be open and then the number represents the number of neighboring mines
#[derive(Clone, Debug, PartialEq)]
enum SquareState {
    Closed(bool),
    Open(u8),
}
/// # Game State
/// The main game state that runs the game
pub struct GameState {
    game_config: GameConfig,
    grid: Vec<SquareState>,
    mines: std::collections::HashSet<usize>,
    flag_image: graphics::Image,
    number_image: graphics::Image,
    mine_image: graphics::Image,
    square: graphics::Mesh,
    mouse_index: Option<IndexType>,
    mouse_press: Option<(ggez::input::mouse::MouseButton, IndexType)>,
}

impl GameState {
    /// # New
    /// create a new game state
    pub fn new(ctx: &mut Context, game_config: GameConfig) -> GameResult<Self> {
        let grid =
            vec![SquareState::Closed(false); game_config.game_size.0 * game_config.game_size.1];
        let mines = std::collections::HashSet::<usize>::new();

        let flag_image = graphics::Image::new(ctx, "\\flag.png")?;
        let number_image = graphics::Image::new(ctx, "\\spr_numbers.png")?;
        let mine_image = graphics::Image::new(ctx, "\\mine.png")?;

        let color = graphics::WHITE;
        let rect = graphics::Rect::new(0.0, 0.0, game_config.square_size, game_config.square_size);
        let square = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;

        Ok(GameState {
            game_config,
            grid,
            mines,
            flag_image,
            number_image,
            mine_image,
            square,
            mouse_index: None,
            mouse_press: None,
        })
    }

    /// # Index To Point
    /// Converts from a linear index to a 2 dimensional point.   
    ///
    /// returns:   
    /// x = index % game width   
    /// y = index / game width
    fn index_to_point(&self, i: IndexType) -> cgmath::Vector2<i32> {
        cgmath::Vector2::new(
            (i % self.game_config.game_size.0) as i32,
            (i / self.game_config.game_size.0) as i32,
        )
    }

    /// # Point To Index
    /// convert from a point to a index
    ///   
    /// returns:   
    /// x + y * game width
    fn point_to_index(&self, point: cgmath::Vector2<i32>) -> usize {
        point.x as usize + point.y as usize * self.game_config.game_size.0
    }

    /// # Get Neighbor
    /// Gets the indices for all of the neighbors to a square
    fn get_neighbors(&self, index: usize) -> [Option<usize>; 8] {
        let point = self.index_to_point(index);
        let mut i = 0;
        let mut neighbors = [Option::<usize>::None; 8];
        //Loop through all neighbors
        for x in -1..2 {
            for y in -1..2 {
                //Skip the middle
                if x == 0 && y == 0 {
                    continue;
                }

                let current_point = point + cgmath::vec2(x, y);

                if current_point.x < 0 || current_point.y < 0 {
                    continue;
                }

                if current_point.x >= self.game_config.game_size.0 as i32
                    || current_point.y >= self.game_config.game_size.1 as i32
                {
                    continue;
                }

                neighbors[i] = Some(self.point_to_index(current_point));
                i += 1;
            }
        }

        neighbors
    }

    /// # Count neighbors
    /// Counts the amount of neighboring squares with mines
    fn count_neighbors(&self, i: usize) -> u8 {
        let mut count = 0;
        let neighbors = self.get_neighbors(i);

        for neighbor in neighbors.iter() {
            if let Some(index) = neighbor {
                if self.mines.contains(index) {
                    count += 1;
                }
            }
        }

        count
    }

    /// # Draw Squares
    /// Draw the squares
    fn draw_squares(&self, ctx: &mut ggez::Context) -> GameResult<()> {
        let colors = &self.game_config.colors;

        for i in 0..self.grid.len() {
            let point = self.index_to_point(i);
            let v = self.game_config.square_size * Point2::new(point.x as f32, point.y as f32);

            let mut params = graphics::DrawParam::new();
            params.dest = v.into();

            match self.grid[i] {
                SquareState::Closed(flag) => {
                    // if the mouse is pressed the square it was pressed on is the selected one
                    // otherwise it is the square that the mouse is over
                    params.color = if let Some((_, index)) = self.mouse_press {
                        if index == i {
                            colors.selected_square.into()
                        } else {
                            colors.square.into()
                        }
                    } else if let Some(index) = self.mouse_index {
                        if index == i {
                            colors.selected_square.into()
                        } else {
                            colors.square.into()
                        }
                    } else {
                        colors.square.into()
                    };

                    graphics::draw(ctx, &self.square, params)?;

                    if flag {
                        params.color = graphics::WHITE;
                        let scale = self.game_config.square_size / self.flag_image.dimensions().w;
                        params.scale = ggez::mint::Vector2 { x: scale, y: scale };
                        graphics::draw(ctx, &self.flag_image, params)?;
                    }
                }
                SquareState::Open(mine_count) => {
                    if self.mines.contains(&i) {
                        params.color = graphics::WHITE;
                        graphics::draw(ctx, &self.mine_image, params)?;
                    } else if mine_count > 0 {
                        let origin_point =
                            cgmath::vec2((mine_count % 3) as f32, (mine_count / 3) as f32);
                        let origin_pos: cgmath::Vector2<f32> =
                            origin_point * self.game_config.square_size / 96.0;
                        params.src =
                            graphics::Rect::new(origin_pos.x, origin_pos.y, 1.0 / 3.0, 1.0 / 3.0);

                        graphics::draw(ctx, &self.number_image, params)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// # Open
    /// Opens a square and checks the amount of neighboring mines   
    /// If mines aren't generated it will generate them first
    fn open(&mut self, index: IndexType) {
        if self.mines.is_empty() {
            self.generate_mines(self.game_config.number_of_mines, index);
        }

        let neighbor_count = self.count_neighbors(index);
        self.grid[index] = SquareState::Open(neighbor_count);

        if neighbor_count > 0 {
            return;
        }

        for &neighbor in self.get_neighbors(index).iter() {
            if let Some(index) = neighbor {
                if let SquareState::Closed(_b) = self.grid[index] {
                    self.open(index)
                }
            }
        }
    }

    /// # Generate mines
    /// Generate mines in random slots
    fn generate_mines(&mut self, number_of_mines: IndexType, graced_index: IndexType) {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new(0, self.grid.len());

        let mut result = collections::HashSet::new();

        let mut tries = 0;
        while result.len() < number_of_mines {
            let next = dist.sample(&mut rng);

            if next != graced_index {
                result.insert(next);
            }
            tries += 1;
        }

        trace!("Mines generated at {:?} after {} tries", result, tries);
        self.mines = result;
    }
}

impl State for GameState {
    /// # Update
    /// Main update
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<UpdateResult> {
        //update delta time
        let _dt = ggez::timer::delta(ctx);

        Ok(UpdateResult::Block)
    }

    /// # Draw
    /// Draw the playing grid
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.draw_squares(ctx)?;
        Ok(())
    }

    /// # Mouse Motion Event
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
        let point = cgmath::Vector2::<i32>::new(
            (x / self.game_config.square_size) as i32,
            (y / self.game_config.square_size) as i32,
        );

        // Update the mouse index
        self.mouse_index = if point.x >= 0
            && point.y >= 0
            && point.x < self.game_config.game_size.0 as i32
            && point.y < self.game_config.game_size.1 as i32
        {
            Some(self.point_to_index(point))
        } else {
            None
        };

        Ok(EventResult::Block)
    }

    /// # Mouse Button Up Event
    /// Triggered when the mouse is released and is the end of a mouse press.   
    /// If the mouse is released on the same square as it was pressed it will call ```open```
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

    /// # Mouse Button Down Event
    /// Triggered when the mouse button is pressed down.   
    /// Saves which square the mouse was over when the button was pressed.
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
