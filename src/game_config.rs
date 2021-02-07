use ron::{de::from_reader, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GameColors {
    pub square: (u8, u8, u8),
    pub selected_square: (u8, u8, u8),
    pub mine_square: (u8, u8, u8),
}

#[derive(Deserialize, Debug)]
pub struct GameConfig {
    pub game_size: (usize, usize),
    pub number_of_mines: usize,
    pub square_size: f32,
    pub colors: GameColors,
}

impl GameConfig {
    pub fn load_from_file(file: ggez::filesystem::File) -> Result<GameConfig, Error> {
        from_reader(file)
    }
}
