#[derive(Debug, Clone)]
pub enum Error {
    GameError(ggez::GameError),
    RonError(ron::error::Error),
}
