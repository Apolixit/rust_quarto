use std::fmt::Display;

use crate::{piece::Piece, board::Board, r#move::Move, error::ErrorGame, ai::adequat_strategy};

pub enum PlayerType {
    /// A human player
    HUMAN,
    /// An AI player
    AI,
}

pub trait Player {
    /// The current player name
    fn name(&self) -> String;

    /// Explicit enum to declare player as Human or AI
    fn player_type(&self) -> PlayerType;

    /// Selected his own move
    fn choose_move(&self, piece: Piece, board: &Board) -> Result<Move, ErrorGame>;

    /// Choose a piece for the opponent
    fn choose_piece_for_opponent(&self, board: &Board) -> Piece;
}

/// Represent a player (humain or AI)
pub struct Human {
    name: String,
}

impl Human {
    pub fn new(name: &str) -> Human {
        Human {
            name: name.to_string(),
        }
    }
}

impl Player for Human {
    fn name(&self) -> String {
        String::from(&self.name)
    }

    fn player_type(&self) -> PlayerType {
        PlayerType::HUMAN
    }

    fn choose_move(&self, _piece: Piece, _board: &Board) -> Result<Move, ErrorGame> {
        unimplemented!()
    }

    fn choose_piece_for_opponent(&self, _board: &Board) -> Piece {
        unimplemented!()
    }
}

impl Display for dyn Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone)]
pub struct AI {
    name: String,
}

impl AI {
    /// Create a new AI player
    pub fn new() -> AI {
        AI {
            name: AI::default_name(),
        }
    }

    pub fn default_name() -> String {
        String::from("AI")
    }
}

impl Player for AI {
    fn name(&self) -> String {
        String::from(&self.name)
    }

    fn player_type(&self) -> PlayerType {
        PlayerType::AI
    }

    /// Calc the algorithm to choose the best move
    fn choose_move(&self, piece: Piece, board: &Board) -> Result<Move, ErrorGame> {
        adequat_strategy(&board).calc_move(board, Some(piece))
    }

    /// Calc the algorithm to choose the worst piece for the opponent
    fn choose_piece_for_opponent(&self, board: &Board) -> Piece {
        adequat_strategy(&board).choose_piece_for_opponent(board)
    }
}