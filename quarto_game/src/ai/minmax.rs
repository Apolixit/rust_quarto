use crate::ai::play;
use crate::board::Board;
use crate::board::BoardIndex;
use crate::error::ErrorGame;
use crate::piece::Piece;
use crate::r#move::Move;
use std::cmp::max;
use core::cmp::Ordering;
use std::cmp::min;
use std::collections::HashMap;

use super::get_moves;
use super::Score;
use super::Strategy;

/// This minmax algorithm is no longer in use
/// This is the minmax_tree currently use as the ai because I store every move as a tree, it's easier to
/// store value and to debug


/// The MinMax struct
pub struct MinMax {
    pub depth: usize,
    pub maximise: bool,
}

impl MinMax {
    pub fn new(depth: usize, maximise: bool) -> MinMax {
        MinMax { depth, maximise }
    }

    pub fn name() -> String {
        String::from("minmax")
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    /// Basic MinMax algorithm with just tracking score of each node
    pub fn minmax(board: &Board, depth: usize, maximise: bool, available_moves: &Vec<Move>) -> Score {
        if depth == 0 || !board.can_play_another_turn() {
            let score = Score::calc_score(board);
            debug!("MinMax depth = 0, score = {:?}", score);

            return score;
        }

        trace!("Current MinMax depth = {}", depth);

        let mut score;
        if maximise {
            score = Score::Point(usize::MIN);
            for m in available_moves {
                let mut board = board.clone();
                play(&mut board, &m);
                let save_old_score = score;
                score = max(
                    score,
                    MinMax::minmax(&board, depth - 1, !maximise, &board.get_available_moves()),
                );
                if score != save_old_score {
                    debug!("From move ({:?}) : maximising / depth = {} / old score = {:?} / minmax score = {:?} ", m, depth, save_old_score, score);
                }
            }
        } else {
            score = Score::Win;
            for m in available_moves {
                let mut board = board.clone();
                play(&mut board, &m);
                let save_old_score = score;
                score = min(
                    score,
                    MinMax::minmax(&board, depth - 1, !maximise, &board.get_available_moves()),
                );

                if score != save_old_score {
                    debug!("From move ({:?}) : minimising / depth = {} / old score = {:?} / minmax score = {:?}", m, depth, save_old_score, score);
                }
            }
        }
        score
    }

    /// Calc the score for each move (wrapper of minmax function)
    fn calc_next_moves_score(
        board: &Board,
        depth: usize,
        maximise: bool,
        piece: Option<Piece>,
    ) -> Vec<(Score, Move)> {
        let moves = get_moves(board, piece);
        let mut move_result: Vec<(Score, Move)> = vec![];

        for m in moves {
            let score = MinMax::minmax(&board.clone(), depth, maximise, &vec![m.clone()]);
            trace!("Play move = {} / Score {:?}", &m, &score);

            move_result.push((score, m));
        }

        move_result
    }

    /// The combinaison (nb_piece_already_played, depth to search)
    /// For example here:
    ///     - between 0 and 5 pieces, we search with depth = 2
    ///     - between 5 and 8 pieces, we search with depth = 3
    ///     - etc
    /// I try this to have a better algo in end game
    pub fn calc_adequat_depth(nb_piece_left: usize) -> usize {
        // const THRESHOLD_PIECE_PLAYED_DEPTH: Vec<((usize, usize), usize)> = vec![((0, 5), 2), ((5, 8), 3), ((8, 11), 4), ((11, 16), 5)];
        // let max_cells = WIDTH_BOARD * HEIGHT_BOARD - 1;
        match nb_piece_left {
            0..=4 => 2,
            5..=7 => 3,
            8..=10 => 4,
            11..=15 => 5,
            _ => 0
        }
    }
}

impl Strategy for MinMax {
    fn name(&self) -> String {
        MinMax::name()
    }

    fn calc_move(&mut self, board: &Board, piece: Option<Piece>) -> Result<Move, ErrorGame> {
        let moves_score_result =
            MinMax::calc_next_moves_score(board, self.depth, self.maximise, piece);
        debug!("calc_move >> moves_score_result = {:?}", moves_score_result);

        Ok(if self.maximise {
            //If maximise, we take take the max score
            let res = moves_score_result
                .into_iter()
                .max_by_key(|s| s.0)
                .ok_or(ErrorGame::NoBestMove)?;
            info!(
                "The max score selected is : {:?} for the move : {}",
                &res.0, &res.1
            );
            res.1
        } else {
            // If minimise we take the min
            let res = moves_score_result
                .into_iter()
                .min_by_key(|s| s.0)
                .ok_or(ErrorGame::NoBestMove)?;
            info!(
                "The min score selected is : {:?} for the move : {}",
                &res.0, &res.1
            );
            res.1
        })
    }

    fn choose_piece_for_opponent(&mut self, board: &Board) -> Piece {
        let moves_score_result = MinMax::calc_next_moves_score(board, self.depth, true, None);

        let mut best_move_per_piece: HashMap<usize, Score> = HashMap::new();
        for (new_score, new_move) in moves_score_result {
            best_move_per_piece
                .entry(new_move.piece().to_index(&board).unwrap())
                .and_modify(|score| *score = max(*score, new_score))
                .or_insert(new_score);
        }
        info!("best_move_per_piece = {:?}", best_move_per_piece);
        let worst_score = best_move_per_piece.into_iter().min_by_key(|k| k.1).unwrap();
        Piece::from_index(&board, worst_score.0).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::ai::get_moves;
    use crate::ai::minmax;
    use crate::ai::play;
    use crate::ai::Score;
    use crate::ai::{MinMax, Strategy};
    use crate::board::{BoardIndex, Cell};
    use crate::r#move::Move;
    use crate::{board::Board, piece::Piece};

    fn late_game(nb_piece_left: usize) -> Vec<Piece> {
        let mut pieces: Vec<Piece> = vec![
            Piece::from("WETS"),
            Piece::from("DFTC"),
            Piece::from("DFTS"),
            Piece::from("DFXS"),
            Piece::from("WFTS"),
            Piece::from("WFXS"),
            Piece::from("DETS"),
            Piece::from("DFXC"),
            Piece::from("DEXS"),
            Piece::from("WEXC"),
            Piece::from("WFXC"),
            Piece::from("WETC"),
            Piece::from("DETC"),
            Piece::from("WFTC"),
            Piece::from("WEXS"),
            Piece::from("DEXC"),
        ];

        let mut nb_piece_left = nb_piece_left;
        while (nb_piece_left > 0) {
            pieces.remove(pieces.len() - 1);
            nb_piece_left -= 1;
        }
        pieces
    }

    fn fill_board(nb_piece_left: usize) -> Board {
        let mut board = Board::create();

        let cloned_board = board.clone();

        //Play the first piece in first cell of the board
        for (cell, piece) in (0..14)
            .map(|i| Cell::from_index(&cloned_board, i).unwrap())
            .zip(late_game(nb_piece_left))
        {
            // let piece_index = Piece::from board.get_piece_index(&play.1).unwrap();
            board.play(piece, cell).unwrap();
            board.remove(piece).unwrap();
        }

        board
    }



    #[test]
    fn test_minmax_tree() {
        // Test case : nb_piece_left = 3, depth = 3
        // Test case : nb_piece_left = 6, depth = 3 -> fail
        let mut board = fill_board(6);

        // We have 3 pieces and cells which haven't been played, so we have 9 moves available
        // assert_eq!(board.get_available_moves().len(), 9);
        warn!("{}", board);
        // info!("{:?}", Score::calc_score(&board));
        // info!("{:?}", board.board_state());

        let piece_to_play = Piece::from_index(&board, 1).unwrap();

        let mut minmax = MinMax::new(3, true);
        let best_minmax_score = MinMax::minmax(
            &board,
            minmax.depth,
            minmax.maximise,
            &get_moves(&board, Some(piece_to_play)),
        );
        let best_first_move = minmax.calc_move(&board, Some(piece_to_play)).unwrap();

        // We got the best score from minmax and we got the best move, now we check that playing this move give the max score
        board
            .play(best_first_move.piece(), best_first_move.cell())
            .unwrap();

        warn!("{}", board);
        info!("best_minmax_score = {:?} / best_first_move = {} / Current board score after best move = {:?}", best_minmax_score, best_first_move, Score::calc_score(&board));
        //assert_eq!(Score::calc_score(&board), best_minmax_score);
        board.board_state();
    }

    #[test]
    fn test_depth_0_eq_current_board_score() {
        let board = fill_board(5);
        assert_eq!(
            Score::calc_score(&board),
            MinMax::minmax(&board, 0, true, &get_moves(&board, None))
        );
    }
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

        let mut minmax = MinMax::new(1, true);
        let best_first_move = minmax.calc_move(&board, None).unwrap();
        assert_eq!(best_first_move, winning_move);
    }

    #[test]
    fn test_choose_opponent_piece() {
        let mut board = Board::create();

        let moves: Vec<Move> = vec![
            (Move::new(Piece::from("DFXC"), Cell::from_index(&board, 0).unwrap())),
            (Move::new(Piece::from("DETS"), Cell::from_index(&board, 0).unwrap())),
            (Move::new(Piece::from("WFTC"), Cell::from_index(&board, 6).unwrap())),
            (Move::new(Piece::from("DFTC"), Cell::from_index(&board, 7).unwrap())),
            (Move::new(Piece::from("WFXS"), Cell::from_index(&board, 8).unwrap())),
            (Move::new(Piece::from("WETC"), Cell::from_index(&board, 9).unwrap())),
            (Move::new(Piece::from("WEXC"), Cell::from_index(&board, 10).unwrap())),
            (Move::new(Piece::from("DEXS"), Cell::from_index(&board, 15).unwrap())),
        ];
        for m in moves {
            play(&mut board, &m);
        }

        warn!("{}", board);
        let mut minmax = MinMax::new(2, true);
        let ai_piece = minmax.choose_piece_for_opponent(&board);
        warn!("Piece choose = {}", ai_piece);
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

        let mut minmax = MinMax::new(3, true);
        let best_first_move = minmax.calc_move(&board, Some(Piece::from("DEXS"))).unwrap();
        // let best_first_move = calc_move(&board, 2, true, None).unwrap();
        info!("best move = ({})", best_first_move);

        // Now we play
        board
            .play(best_first_move.piece(), best_first_move.cell())
            .unwrap();
        board.remove(best_first_move.piece()).unwrap();

        info!("{}", board);

        minmax.set_depth(2);
        let piece_to_give = minmax.choose_piece_for_opponent(&board);
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
