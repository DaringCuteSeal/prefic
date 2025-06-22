use std::fmt::Debug;

pub enum AppError {
    ParseError(String),
    InputError(String),
}

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(s) => write!(f, "{}", s),
            Self::InputError(s) => write!(f, "{}", s),
        }
        
    }
}

pub type AppResult<T> = Result<T, AppError>;
