use serde::Deserialize;

/// A struct containing the colors used in the game
#[derive(Deserialize, Debug, Clone)]
pub struct GameColors {
    pub square: (u8, u8, u8),
    pub selected_square: (u8, u8, u8),
    pub mine_square: (u8, u8, u8),
}

///The main game config struct loaded fomr config.ron in resources
#[derive(Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub game_size: (usize, usize),
    pub number_of_mines: usize,
    pub square_size: f32,
    pub colors: GameColors,
}
