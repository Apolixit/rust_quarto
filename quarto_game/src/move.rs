use core::fmt::Debug;
use std::fmt::Display;

use crate::board::Board;
use crate::board::Cell;
use crate::error::ErrorGame;
use crate::piece::Piece;

/// Represent a move on the board
#[derive(Clone, PartialEq)]
pub struct Move {
    index_piece: usize,
    index_cell: usize,
}

impl Move {
    pub fn new(index_piece: usize, index_cell: usize) -> Result<Move, ErrorGame> {
        Ok(Move {
            index_piece,
            index_cell,
        })
    }

    pub fn get_index_piece(&self) -> usize {
        self.index_piece
    }

    pub fn index_cell(&self) -> usize {
        self.index_cell
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.index_piece, self.index_cell)
    }

    // pub fn get_piece(&self, board: Board) -> Result<&Piece, ErrorGame> {
    //     Ok(board.get_piece_from_available(self.index_piece)?)
    // }

    // pub fn get_cell(&self, board: Board) -> Result<&Cell, ErrorGame> {
    //     let x = board
    //         .get_cells()
    //         .get(&self.index_cell)
    //         .ok_or(ErrorGame::IndexOutOfBound)?;

    //     Ok(x)
    // }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(piece num {} / cell num {})", (self.index_piece + 1), (self.index_cell + 1))
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.index_piece, self.index_cell)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::HEIGHT_BOARD;
use crate::board::WIDTH_BOARD;
use crate::ai::Score;
use crate::{board::Board, piece::Piece};


    #[test]
    fn test_available_move() {
        let mut board = Board::create();
        let range_move = 0..((WIDTH_BOARD * HEIGHT_BOARD) - 2);

        for index in range_move {
            board.play_piece(index, index).unwrap();
            board.remove_piece(index).unwrap();
        }

        // We have 2 pieces and cells which haven't been played, so we have 4 moves available
        assert_eq!(board.get_available_moves().len(), 4);
    }
}