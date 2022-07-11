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
