use crate::board::Board;
use crate::board::BoardIndex;
use crate::error::ErrorGame;
use crate::piece::Piece;
use crate::r#move::Move;
use core::cmp::max;
use core::cmp::min;
use std::collections::HashMap;

use super::Score;

/// MinMax algorithm
fn minmax(board: &Board, depth: usize, maximise: bool, available_moves: &Vec<Move>) -> Score {
    if depth == 0 || !board.can_play_another_turn() {
        let score = Score::calc_score(board);
        trace!("MinMax depth = 0, score = {:?}", score);

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

fn calc_next_moves_score(
    board: &Board,
    depth: usize,
    maximise: bool,
    piece: Option<Piece>,
) -> Vec<(Score, Move)> {
    let moves = get_moves(board, piece);
    let mut move_result: Vec<(Score, Move)> = vec![];
    // info!("{} moves should be evaluated", moves.len());

    // let best_score = minmax(board, depth, maximise, &moves);

    // info!(
    //     "Best play with depth = {}, best score = {:?}",
    //     depth, best_score
    // );

    for m in moves {
        // let mut child_board = board.clone();
        // info!("Score before move = {:?}", Score::calc_score(&child_board));
        // play(&mut child_board, &m);
        // info!("{}", &child_board);
        // info!("Score after move = {:?}", Score::calc_score(&child_board));
        // let score = minmax(&child_board, depth, maximise, &child_board.get_available_moves());

        let score = minmax(&board.clone(), depth, maximise, &vec![m.clone()]);
        trace!("Play move = {} / Score {:?}", &m, &score);

        // info!("Max score from this board with depth = {} ==== {:?}", depth, score);
        move_result.push((score, m));

        // let child_score = Score::calc_score(&child_board);
        // info!("Just played move = {}, got the score = {:?}", m, child_score);
        // if child_score == best_score {
        //     return m;
        // }
    }

    move_result
}

/// Return the best play from :
/// - the current board state
/// - the depth of the the search moves
/// - Do we currently maximize the score ?
/// - The piece to be played (it's specific to Quarto)
pub fn calc_move(
    board: &Board,
    depth: usize,
    maximise: bool,
    piece: Option<Piece>,
) -> Result<Move, ErrorGame> {
    let moves_score_result = calc_next_moves_score(board, depth, maximise, piece);
    // info!("Moves result = {:?}", move_result);

    Ok(if maximise {
        //If maximise, we take take the max score
        let res = moves_score_result
            .into_iter()
            .max_by_key(|s| s.0)
            .ok_or(ErrorGame::NoBestMove)?
            .1;
        info!("The max score selected is : {}", &res);
        res
    } else {
        // If minimise we take the min
        let res = moves_score_result
            .into_iter()
            .min_by_key(|s| s.0)
            .ok_or(ErrorGame::NoBestMove)?
            .1;
        info!("The min score selected is : {}", &res);
        res
    })

    // Ok(move_result
    //     .into_iter()
    //     .max_by_key(|s| s.0)
    //     .ok_or(ErrorGame::NoBestMove)?
    //     .1)

    // Move::new(0, 0).unwrap()
}

pub fn calc_piece(board: &Board, depth: usize, maximise: bool) -> Piece {
    // let mut move_score: Vec<(Move, Score)> = vec![];
    let moves_score_result = calc_next_moves_score(board, depth, maximise, None);
    // for current_move in board.get_available_moves() {
    // let mut child_board = board.clone();
    // play(&mut child_board, &current_move);
    // let mut value = Score::Point(usize::MIN);
    // value = min(
    //     value,
    //     minmax(
    //         &child_board,
    //         depth - 1,
    //         maximise,
    //         &child_board.get_available_moves(),
    //     ),
    // );

    //     move_score.push((current_move, value));
    // }

    // move_score
    //     .into_iter()
    //     .min_by_key(|x| x.1)
    //     .map(|x| board.get_piece_from_available(x.0.get_index_piece()))
    //     .unwrap()
    //     .unwrap()

    // let x = moves_score_result.into_iter().max_by_key(|k| k.0);
    let mut best_move_per_piece: HashMap<usize, Score> = HashMap::new();
    for (new_score, new_move) in moves_score_result {
        best_move_per_piece
            .entry(new_move.piece().to_index(&board).unwrap())
            .and_modify(|score| *score = max(*score, new_score))
            .or_insert(new_score);
    }
    info!("best_move_per_piece = {:?}", best_move_per_piece);
    let worst_score = best_move_per_piece.into_iter().min_by_key(|k| k.1).unwrap();
    // board.get_piece_from_available(worst_score.0).unwrap()
    Piece::from_index(&board, worst_score.0).unwrap()
}

fn get_moves(board: &Board, piece: Option<Piece>) -> Vec<Move> {
    if let Some(piece) = piece {
        board.get_available_moves_from_piece(piece)
    } else {
        board.get_available_moves()
    }
}

fn play(board: &mut Board, m: &Move) {
    if let Err(e) = board.play(m.piece(), m.cell()) {
        error!("{}", e.message());
    }

    board.remove(m.piece()).unwrap();
}

#[cfg(test)]
mod tests {

    use crate::ai::minmax::{calc_move, calc_piece};
    use crate::ai::Score;
    use crate::board::{BoardIndex, Cell};
    use crate::r#move::Move;
    use crate::{board::Board, piece::Piece};

    #[test]
    fn test_best_play_should_win_in_one_depth() {
        // The first winning move the algorithm has to find in the next turn (depth = 1)
        // let winning_move = Move::new(2, 3).unwrap();
        let mut board = Board::create();
        let winning_move = Move::from_index(12, 14, &board).unwrap();

        let moves = vec![
            (Piece::from("WETS"), 0),
            (Piece::from("DFTC"), 1),
            (Piece::from("DFTS"), 2),
            (Piece::from("WFTS"), 4),
            (Piece::from("DFXS"), 6),
            (Piece::from("DETC"), 10),
        ];

        for (piece_current, index_board) in moves {
            board
                .play(
                    piece_current,
                    Cell::from_index(&board, index_board).unwrap(),
                )
                .unwrap();
            board.remove(piece_current).unwrap();
        }

        let best_first_move = calc_move(&board, 1, true, None).unwrap();
        assert_eq!(best_first_move, winning_move);
    }

    #[test]
    fn test_choose_piece_when_start() {
        let depth: usize = 1;

        info!("Start a new game");
        let board = Board::create();
        info!("Start calc_piece with depth = {}", depth);
        let piece_choose = calc_piece(&board, depth, true);

        info!("The piece {} has been choose", piece_choose);
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
            board
                .play(
                    piece_current,
                    Cell::from_index(&board, index_board).unwrap(),
                )
                .unwrap();
            board.remove(piece_current).unwrap();
        }

        info!(
            "{:?}",
            board.get_available_moves_from_piece(Piece::from("DEXS"))
        );

        info!("{}", board);

        let best_first_move = calc_move(&board, 3, true, Some(Piece::from("DEXS"))).unwrap();
        // let best_first_move = calc_move(&board, 2, true, None).unwrap();
        info!("best move = ({})", best_first_move);

        // Now we play
        board
            .play(
                best_first_move.piece(),
                best_first_move.cell(),
            )
            .unwrap();
        board
            .remove(best_first_move.piece())
            .unwrap();

        info!("{}", board);

        let piece_to_give = calc_piece(&board, 2, true);
        info!("I give the player this piece = {}", piece_to_give);
        // info!("{}", board);
        //
        // info!(
        //     "best_first_move.get_index_piece() = {}, this piece is {}, best_first_move.index_cell() = {}",
        //     best_first_move.get_index_piece(),
        //     board.get_piece_from_available(best_first_move.get_index_piece()).unwrap(),
        //     best_first_move.index_cell()
        // );

        // -----

        // board
        //     .play_piece(
        //         best_first_move.get_index_piece(),
        //         best_first_move.index_cell(),
        //     )
        //     .unwrap();
        // board.remove_piece(best_first_move.get_index_piece()).unwrap();
        // assert_eq!(Score::calc_score(&board), Score::Point(18));
        // info!("{}", board);
        // let worst_move = calc_move(&board, 1, false, None);
        // info!("worst move = ({})", worst_move);

        // let worst_piece = calc_piece(&board, 1, true);
        // info!("worst piece = ({})", worst_piece);

        // -----

        // assert_eq!(best_first_move, Move::new(2, 3).unwrap());
        // for m in board.get_available_moves() {
        //     info!("{}", m);
        // }
        // info!("{:?}", board.get_available_moves());
        // info!("{}", board);
        assert!(true);
    }
}
