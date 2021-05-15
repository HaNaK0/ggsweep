#[derive(Debug, Clone)]
pub enum WrappedError {
    GameError(ggez::GameError),
    RonError(ron::error::Error),
    SheetError(String),
}

#[derive(Debug, Clone)]
pub struct LocatedError {
    error: WrappedError,
    location: String,
}

impl LocatedError {
    pub fn new(error: impl Into<WrappedError>, location: String) -> Self {
        Self {
            error: error.into(),
            location,
        }
    }
}

impl From<ggez::GameError> for WrappedError {
    fn from(error: ggez::GameError) -> Self {
        Self::GameError(error)
    }
}

impl From<ron::error::Error> for WrappedError {
    fn from(error: ron::error::Error) -> Self {
        Self::RonError(error)
    }
}

#[macro_export]
macro_rules! err_here {
    () => {
        |e| LocatedError::new(e, format!("at line {} in {}", line!(), file!()))
    };
}
