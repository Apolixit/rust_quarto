use rand::Rng;

use crate::{board::Board, error::ErrorGame, piece::Piece, r#move::Move};

use super::{get_moves, Strategy};

pub struct RandomAI {}

impl RandomAI {
    pub fn new() -> RandomAI {
        RandomAI::default()
    }

    pub fn name() -> String {
        String::from("random")
    }
}
impl Default for RandomAI {
    fn default() -> Self {
        Self {  }
    }
}

/// This strategy is use for the first AI move, because
impl Strategy for RandomAI {
    fn name(&self) -> String {
        RandomAI::name()
    }

    fn calc_move(
        &mut self,
        board: &Board,
        piece: Option<Piece>,
    ) -> Result<Move, ErrorGame> {
        let moves = get_moves(&board, piece);
        Ok(*moves
            .get(rand::thread_rng().gen_range(0..moves.len()))
            .ok_or(ErrorGame::NoBestMove)?)
    }

    fn choose_piece_for_opponent(&self, board: &Board) -> Piece {
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

        let move_result = RandomAI::new().calc_move(&board, None);

        assert!(move_result.is_ok());
    }

    #[test]
    fn test_choose_piece_for_opponent_random() {
        let board = Board::create();
        RandomAI::new().choose_piece_for_opponent(&board);
        // Just to check nothing panic
    }
}