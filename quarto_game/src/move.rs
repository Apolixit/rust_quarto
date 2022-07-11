use crate::board::BoardIndex;
use core::fmt::Debug;
use std::fmt::Display;

use crate::board::Board;
use crate::board::Cell;
use crate::error::ErrorGame;
use crate::piece::Piece;

/// Represent a move on the board
#[derive(Clone, PartialEq, Copy)]
pub struct Move {
    piece: Piece,
    cell: Cell,
}

impl Move {
    pub fn new(piece: Piece, cell: Cell) -> Move{
        Move {
            piece: piece,
            cell: cell
        }
    }
    pub fn from_index(index_piece: usize, index_cell: usize, board: &Board) -> Result<Move, ErrorGame> {
        Ok(Move::new(Piece::from_index(&board, index_piece).unwrap(), Cell::from_index(&board, index_cell).unwrap()))
    }

    pub fn to_tuple(&self, board: &Board) -> (usize, usize) {
        (self.piece.to_index(&board).unwrap(), self.cell().to_index())
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn cell(&self) -> Cell {
        self.cell
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(piece {} / cell num {})", (self.piece()), (self.cell().to_index() + 1))
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, i = {})", self.piece(), self.cell().to_index())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::BoardIndex;
    use crate::board::Cell;
    use crate::board::HEIGHT_BOARD;
    use crate::board::WIDTH_BOARD;
    use crate::piece;
    use crate::{board::Board, piece::Piece};


    #[test]
    fn test_available_move() {
        let mut board = Board::create();
        let range_move = 0..((WIDTH_BOARD * HEIGHT_BOARD) - 2);

        for index in range_move {
            let piece = Piece::from_index(&board, index).expect(format!("Piece from index {} is not yet available", index).as_str());
            let cell = Cell::from_index(&board, index).expect(format!("Cell from index {} is out of bounds", index).as_str());

            trace!("Getting piece and cell from the same index = {}. Piece selected = {}", index, &piece);

            board.play(piece, cell).expect(format!("Something is wrong when playing {} in cell {}", &piece, &cell).as_str());
            board.remove(piece).expect(format!("Error when removing {} from the board", &piece).as_str());
        }

        // We have 2 pieces and cells which haven't been played, so we have 4 moves available
        assert_eq!(board.get_available_moves().len(), 4);
    }
}