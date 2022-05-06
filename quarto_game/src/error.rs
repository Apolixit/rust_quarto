use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ErrorGame {
    IndexOutOfBound,
    PieceDoesNotExists,
    PieceDoesNotBelongPlayable
}

impl ErrorGame {
    pub fn message(&self) -> &str {
        match self {
            Self::IndexOutOfBound => "Index out of bound",
            Self::PieceDoesNotExists => "Piece does not exists",
            Self::PieceDoesNotBelongPlayable => "Piece has already been played"
        }
    }
}

impl Display for ErrorGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}