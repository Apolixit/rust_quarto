use crate::r#move::Move;
use crate::board::Board;
use core::cmp::max;
use core::cmp::min;

use super::Score;

/// MinMax algorithm
fn minmax(board: &Board, depth: usize, maximise: bool) -> Score {
    if depth == 0 || !board.can_play_another_turn() {
        let score = Score::calc_score(board);
        info!("MinMax depth = 0, score = {:?}", score);

        return score;
    }
    trace!("MinMax depth = {}", depth);

    let mut value;
    if maximise {
        value = Score::Point(usize::MIN);
        // for (index_piece, index_cell) in board.get_available_moves() {
        for m in board.get_available_moves() {
            let mut child_board = board.clone();
            play(&mut child_board, &m);
            value = max(value, minmax(&child_board, depth - 1, !maximise));
        }
    } else {
        value = Score::Point(usize::MAX);
        for m in board.get_available_moves() {
            let mut child_board = board.clone();
            play(&mut child_board, &m);
            value = min(value, minmax(&child_board, depth - 1, maximise));
        }
    }
    value
}

/// Return the best play from :
/// - the current board state
/// - the depth of the the search moves
/// - Do we currently maximize the score ?
/// - The piece to be played (it's specific to Quarto)
fn best_play(board: &Board, depth: usize, maximise: bool) -> Move {
    let best_score = minmax(board, depth, maximise);

    info!(
        "Best play with depth = {}, best score = {:?}",
        depth, best_score
    );

    for m in board.get_available_moves() {
        let mut child_board = board.clone();
        play(&mut child_board, &m);

        if Score::calc_score(&child_board) == best_score {
            return m;
        }
    }

    Move::new(0, 0).unwrap()
}

fn play(board: &mut Board, m: &Move) {
    board.play_piece(m.get_index_piece(), m.index_cell()).unwrap();
    board.remove_piece(m.get_index_piece()).unwrap();
}

#[cfg(test)]
mod tests {

    use crate::ai::minmax::best_play;
    use crate::r#move::Move;
    use crate::{board::Board, piece::Piece};

    #[test]
    fn test_best_play_should_win_in_one_depth() {
        // The first winning move the algorithm has to find in the next turn (depth = 1)
        let winning_move = Move::new(2, 3).unwrap();

        let moves = vec![
            (Piece::from("WETS"), 0),
            (Piece::from("DFTC"), 1),
            (Piece::from("DFTS"), 2),
            (Piece::from("WFTS"), 4),
            (Piece::from("DFXS"), 6),
            (Piece::from("DETC"), 10),
        ];

        let mut board = Board::create();

        for (piece_current, index_board) in moves {
            let piece_index = board.get_piece_index(&piece_current).unwrap();
            board.play_piece(piece_index, index_board).unwrap();
            board.remove_piece(piece_index).unwrap();
        }
        
        let best_first_move = best_play(&board, 1, true);
        assert_eq!(best_first_move, winning_move);
    }

    #[test]
    fn test_best_play_from_start() {
        let moves = vec![
            (Piece::from("WETS"), 0),
            (Piece::from("DFTC"), 1),
            (Piece::from("DFTS"), 2),
            (Piece::from("WFTS"), 4),
            (Piece::from("DFXS"), 6),
            (Piece::from("DETC"), 10),
        ];

        let mut board = Board::create();

        for (piece_current, index_board) in moves {
            let piece_index = board.get_piece_index(&piece_current).unwrap();
            board.play_piece(piece_index, index_board).unwrap();
            board.remove_piece(piece_index).unwrap();
        }
        
        // info!("{:?}", board.get_available_moves());

        let best_first_move = best_play(&board, 1, true);
        info!("best move = ({})", best_first_move);
        assert_eq!(best_first_move, Move::new(2, 3).unwrap());
        // for m in board.get_available_moves() {
        //     info!("{}", m);
        // }
        // info!("{:?}", board.get_available_moves());
        info!("{}", board);
        assert!(true);
    }
}
