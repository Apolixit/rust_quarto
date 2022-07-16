use std::fmt::Display;
use crate::{board::Cell, piece::Piece};

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
    CellIsNotEmpty(Cell, Piece),

    /// No best move has been found by the ai
    NoBestMove
}

impl ErrorGame {
    pub fn message(&self) -> String {
        match self {
            Self::IndexOutOfBound => "The index is out of bound".to_owned(),
            Self::PieceDoesNotExists => "This piece does not exists".to_owned(),
            Self::PieceDoesNotBelongPlayable => "This piece has already been played".to_owned(),
            Self::CellIsNotEmpty(cell, piece) => format!("The cell {} is not empty and have already the piece {}", cell, piece),
            Self::NoBestMove => "No best move has been found by the ai".to_owned(),
        }
    }
}

impl Display for ErrorGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}