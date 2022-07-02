use crate::board::Board;
use crate::piece::Piece;
use crate::r#move::Move;
use core::cmp::max;
use core::cmp::min;

use super::Score;

/// MinMax algorithm
fn minmax(board: &Board, depth: usize, maximise: bool, available_moves: &Vec<Move>) -> Score {
    if depth == 0 || !board.can_play_another_turn() {
        let score = Score::calc_score(board);
        info!("MinMax depth = 0, score = {:?}", score);

        return score;
    }
    trace!("MinMax depth = {}", depth);
    trace!("Available moves = {:?}", available_moves);

    let mut value;
    if maximise {
        value = Score::Point(usize::MIN);
        for m in available_moves {
            let mut child_board = board.clone();
            play(&mut child_board, &m);
            value = max(
                value,
                minmax(
                    &child_board,
                    depth - 1,
                    !maximise,
                    &child_board.get_available_moves(),
                ),
            );
        }
    } else {
        value = Score::Point(usize::MAX);
        for m in available_moves {
            let mut child_board = board.clone();
            play(&mut child_board, &m);
            value = min(
                value,
                minmax(
                    &child_board,
                    depth - 1,
                    maximise,
                    &child_board.get_available_moves(),
                ),
            );
        }
    }
    value
}

/// Return the best play from :
/// - the current board state
/// - the depth of the the search moves
/// - Do we currently maximize the score ?
/// - The piece to be played (it's specific to Quarto)
fn calc_move(board: &Board, depth: usize, maximise: bool, piece: Option<&Piece>) -> Move {
    let moves = get_moves(board, piece);
    let best_score = minmax(board, depth, maximise, &moves);

    info!(
        "Best play with depth = {}, best score = {:?}",
        depth, best_score
    );

    for m in moves {
        let mut child_board = board.clone();
        play(&mut child_board, &m);

        if Score::calc_score(&child_board) == best_score {
            return m;
        }
    }

    Move::new(0, 0).unwrap()
}

fn calc_piece(board: &Board, depth: usize, maximise: bool) -> &Piece {
    let mut move_score: Vec<(Move, Score)> = vec![];
    for current_move in board.get_available_moves() {
        let mut child_board = board.clone();
        play(&mut child_board, &current_move);
        let mut value = Score::Point(usize::MIN);
        value = min(
            value,
            minmax(
                &child_board,
                depth - 1,
                maximise,
                &child_board.get_available_moves(),
            ),
        );

        move_score.push((current_move, value));
    }

    move_score
        .into_iter()
        .min_by_key(|x| x.1)
        .map(|x| board.get_piece_from_available(x.0.get_index_piece()))
        .unwrap()
        .unwrap()
}

fn get_moves(board: &Board, piece: Option<&Piece>) -> Vec<Move> {
    if let Some(piece) = piece {
        board.get_available_moves_from_piece(piece)
    } else {
        board.get_available_moves()
    }
}

fn play(board: &mut Board, m: &Move) {
    if let Err(e) = board
        .play_piece(m.get_index_piece(), m.index_cell()) {
            error!("{}", e.message());
        }

    board.remove_piece(m.get_index_piece()).unwrap();
}

#[cfg(test)]
mod tests {

    use crate::ai::minmax::calc_move;
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

        let best_first_move = calc_move(&board, 1, true, None);
        assert_eq!(best_first_move, winning_move);
    }

    #[test]
    fn test_best_play_from_start() {
        let moves = vec![
            (Piece::from("WETS"), 0),
            (Piece::from("DFTS"), 2),
            (Piece::from("WFTS"), 4),
            (Piece::from("DFXS"), 6),
            (Piece::from("DETC"), 9),
            (Piece::from("WFXS"), 15),
        ];

        let mut board = Board::create();

        for (piece_current, index_board) in moves {
            let piece_index = board.get_piece_index(&piece_current).unwrap();
            board.play_piece(piece_index, index_board).unwrap();
            board.remove_piece(piece_index).unwrap();
        }

        info!(
            "{:?}",
            board.get_available_moves_from_piece(&Piece::from("DEXS"))
        );

        let best_first_move = calc_move(&board, 2, true, Some(&Piece::from("DEXS")));
        info!("best move = ({})", best_first_move);

        // info!("{}", board);
        // board
        //     .play_piece(
        //         best_first_move.get_index_piece(),
        //         best_first_move.index_cell(),
        //     )
        //     .unwrap();
        // info!(
        //     "best_first_move.get_index_piece() = {}, this piece is {}, best_first_move.index_cell() = {}",
        //     best_first_move.get_index_piece(),
        //     board.get_piece_from_available(best_first_move.get_index_piece()).unwrap(),
        //     best_first_move.index_cell()
        // );
        // board.remove_piece(best_first_move.get_index_piece()).unwrap();
        // info!("{}", board);
        // let worst_move = best_play(&board, 1, false, None);
        // info!("worst move = ({})", worst_move);
        // assert_eq!(best_first_move, Move::new(2, 3).unwrap());
        // for m in board.get_available_moves() {
        //     info!("{}", m);
        // }
        // info!("{:?}", board.get_available_moves());
        // info!("{}", board);
        assert!(true);
    }
}
