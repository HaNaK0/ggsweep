use ron::de;
use serde::Deserialize;

/// A struct containing the colors used in the game
#[derive(Deserialize, Debug)]
pub struct GameColors {
    pub square: (u8, u8, u8),
    pub selected_square: (u8, u8, u8),
    pub mine_square: (u8, u8, u8),
}

///The main game config struct loaded fomr config.ron in resources
#[derive(Deserialize, Debug)]
pub struct GameConfig {
    pub game_size: (usize, usize),
    pub number_of_mines: usize,
    pub square_size: f32,
    pub colors: GameColors,
}

impl GameConfig {
    /// Load the game config from a reader
    pub fn load_from_file<R>(file: R) -> Result<GameConfig, ron::Error>
    where
        R: std::io::Read,
    {
        de::from_reader(file)
    }
}
