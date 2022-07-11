use rand::Rng;

use crate::{board::Board, error::ErrorGame, piece::Piece, r#move::Move};

use super::{get_moves, Strategy};

pub struct RandomAI {}

/// This strategy is use for the first AI move, because
impl Strategy for RandomAI {
    fn calc_move(
        board: &Board,
        _: usize,
        _: bool,
        piece: Option<Piece>,
    ) -> Result<Move, ErrorGame> {
        let moves = get_moves(&board, piece);
        Ok(*moves
            .get(rand::thread_rng().gen_range(0..moves.len()))
            .ok_or(ErrorGame::NoBestMove)?)
    }

    fn choose_piece_for_opponent(board: &Board, _: usize) -> Piece {
        let pieces = board.get_available_pieces();
        *pieces
            .get(&rand::thread_rng().gen_range(0..pieces.len()))
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use crate::ai::Board;
use crate::ai::Strategy;
use super::RandomAI;


    /// That's not easy to test random function, but let's check we got no errors
    #[test]
    fn test_calc_move_random() {
        let board = Board::create();

        let move_result = RandomAI::calc_move(&board, 0, true, None);

        assert!(move_result.is_ok());
    }

    #[test]
    fn test_choose_piece_for_opponent_random() {
        let board = Board::create();
        RandomAI::choose_piece_for_opponent(&board, 0);
        // Just to check nothing panic
    }
}