use std::fmt::Display;

/// Represent the different errors which could happen during the game
#[derive(Debug, PartialEq)]
pub enum ErrorGame {
    /// Try to play outside the board
    IndexOutOfBound,

    /// The piece doesn't exist (should never happen)
    PieceDoesNotExists,

    /// The piece has already been played
    PieceDoesNotBelongPlayable,

    /// A piece has already been played on this cell
    CellIsNotEmpty,
}

impl ErrorGame {
    pub fn message(&self) -> &str {
        match self {
            Self::IndexOutOfBound => "The index is out of bound",
            Self::PieceDoesNotExists => "This piece does not exists",
            Self::PieceDoesNotBelongPlayable => "This piece has already been played",
            Self::CellIsNotEmpty => "The cell is not empty",
        }
    }
}

impl Display for ErrorGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}