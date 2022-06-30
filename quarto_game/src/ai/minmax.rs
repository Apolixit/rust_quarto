use crate::board::Board;

use super::Score;

/// MinMax algorithm
fn minmax(board: &Board, depth: usize,  maximise: bool) -> Score {
    // if depth == 0 || !board.can_play_another_turn() {
    //     return Score::calc_score(board);
    // }

    // if maximise {
    //     let value: usize = usize::MIN;
    //     for (index_piece, index_cell) in board.get_available_moves() {

    //     }
    // } else {
    //     let value: usize = usize::MAX;
    // }
    todo!();
}