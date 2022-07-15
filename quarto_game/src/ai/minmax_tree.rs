use std::cmp::Ordering;
use crate::ai::Score;
use crate::ai::get_moves;
use crate::ai::play;
use std::cmp::max;
use std::cmp::min;
use crate::ai::ErrorGame;
use crate::r#move::Move;
use crate::ai::Piece;
use crate::ai::Board;
use crate::ai::Strategy;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MinMaxTree {
    piece: Option<Piece>,
    c_move: Option<Move>,
    score: Score,
    depth: usize,
    maximise: bool,
    children: Vec<MinMaxTree>,
}

impl PartialOrd for MinMaxTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MinMaxTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl MinMaxTree {
    fn new(c_move: Option<Move>, depth: usize, maximise: bool) -> MinMaxTree {
        MinMaxTree {
            piece: None,
            c_move: c_move,
            score: if maximise {
                Score::Point(usize::MIN)
            } else {
                Score::Win
            },
            depth: depth,
            maximise: maximise,
            children: vec![],
        }
    }

    fn new_with_score(
        c_move: Option<Move>,
        depth: usize,
        maximise: bool,
        score: Score,
    ) -> MinMaxTree {
        MinMaxTree {
            piece: None,
            c_move: c_move,
            score: score,
            depth: depth,
            maximise: maximise,
            children: vec![],
        }
    }

    fn update_move_from_first_child(&mut self) {
        if self.c_move.is_none() {
            self.c_move = (*self.children).into_iter().find(|f| f.score == self.score).unwrap().c_move.clone();
        }
    }

    fn children_moves(&self, board: &Board) -> Vec<Move> {
        get_moves(board, self.piece)
    }

    fn with_piece(mut self, piece: Piece) -> MinMaxTree {
        self.piece = Some(piece);
        self
    }

    fn minmax(&mut self, board: &Board) -> MinMaxTree {
        // End the recursivity if we can't go deeper
        if self.depth == 0 || !board.can_play_another_turn() {
            // We calc the final board score
            let final_tree = MinMaxTree::new_with_score(
                self.c_move,
                self.depth,
                self.maximise,
                Score::calc_score(board),
            );

            trace!(
                "MinMaxTree {:?}, score = {:?}, board.can_play_another_turn() = {}",
                final_tree,
                final_tree.score,
                board.can_play_another_turn()
            );
            return final_tree;
        }

        // Loop over each child node
        self.children_moves(&board)
            .into_iter()
            .for_each(|m| {
                let mut child = MinMaxTree::new(Some(m), self.depth - 1, !self.maximise);
                if self.maximise {
                        let mut board = board.clone();
                        play(&mut board, &child.c_move.unwrap());

                        trace!("First child tree = {:?}", child);
                        self.score = max(&mut *self, &mut child.minmax(&board)).score;

                        // debug!("From move ({:?}) : maximising / depth = {} / minmax score = {:?}", child_tree.c_move.unwrap(), self.depth, self.score);
                } else {
                        let mut board = board.clone();
                        play(&mut board, &child.c_move.unwrap());

                        // root_score = min(root_score, child_tree.minmax(&board));
                        self.score = min(&mut *self, &mut child.minmax(&board)).score;
                        // debug!("From move ({:?}) : minimising / depth = {} / minmax score = {:?}", child_tree.c_move.unwrap(), self.depth, self.score);
                }
                self.children.push(child);
            });

        trace!(
            "MinMax depth = {} / maximising = {} / children_tree = {:?}",
            self.depth,
            self.maximise,
            self.children,
        );

        // Need to clone to get new ownership instance
        let mut final_minmax = self.clone();
        final_minmax.update_move_from_first_child();

        final_minmax
    }

    fn minmax_static(board: &Board, minmax_tree: MinMaxTree) -> MinMaxTree {
        if minmax_tree.depth == 0 || !board.can_play_another_turn() {
            let final_tree = MinMaxTree::new_with_score(
                Some(minmax_tree.c_move.unwrap()),
                minmax_tree.depth,
                minmax_tree.maximise,
                Score::calc_score(board),
            );

            trace!(
                "MinMaxTree {:?}, score = {:?}, board.can_play_another_turn() = {}",
                final_tree,
                final_tree.score,
                board.can_play_another_turn()
            );
            return final_tree;
        }

        let mut score =
            MinMaxTree::new(minmax_tree.c_move, minmax_tree.depth, minmax_tree.maximise);


        let children_tree: Vec<MinMaxTree> = get_moves(&board, minmax_tree.piece)
            .into_iter()
            .map(|m| MinMaxTree::new(Some(m), minmax_tree.depth - 1, !minmax_tree.maximise))
            .collect();
        trace!(
            "MinMax depth = {} / maximising = {} / children_tree = {:?}",
            minmax_tree.depth,
            minmax_tree.maximise,
            children_tree
        );

        if minmax_tree.maximise {
            // score = Score::Point(usize::MIN);
            for child_tree in children_tree {
                // minmax_tree.children.push(child_tree);

                let mut board = board.clone();
                play(&mut board, &child_tree.c_move.unwrap());
                // let save_old_score = score;
                trace!("First child tree = {:?}", child_tree);
                // let child_minmax =
                //     MinMaxTree::new(child.c_move, score, self.depth - 1, !self.maximise);
                // self.children.push(child_minmax);

                score = max(score, MinMaxTree::minmax_static(&board, child_tree));
                //debug!("From move ({:?}) : maximising / depth = {} / minmax score = {:?}", child_tree.c_move.unwrap(), minmax_tree.depth, score);
                // if score != save_old_score {
                //     debug!("From move ({:?}) : maximising / depth = {} / old score = {:?} / minmax score = {:?} ", m, depth, save_old_score, score);
                // }
            }
        } else {
            // score = Score::Win;
            for child_tree in children_tree {
                let mut board = board.clone();
                play(&mut board, &child_tree.c_move.unwrap());
                // slet save_old_score = score;

                score = min(score, MinMaxTree::minmax_static(&board, child_tree));
                //debug!("From move ({:?}) : minimising / depth = {} / minmax score = {:?}", child_tree.c_move.unwrap(), minmax_tree.depth, score);
                // if score != save_old_score {
                //     debug!("From move ({:?}) : minimising / depth = {} / old score = {:?} / minmax score = {:?}", m, depth, save_old_score, score);
                // }
            }
        }
        score
    }
}

impl Strategy for MinMaxTree {
    fn name(&self) -> String {
        String::from("MinMaxTree")
    }

    fn calc_move(&mut self, board: &Board, piece: Option<Piece>) -> Result<Move, ErrorGame> {
        if piece.is_some() {
            self.piece = Some(piece.unwrap());
        }

        let minmax_result = self.minmax(board);
        Ok(minmax_result.c_move.unwrap())
    }

    fn choose_piece_for_opponent(&mut self, board: &Board) -> Piece {
        let minmax_result = self.minmax(board);
        let b_move = minmax_result.c_move.unwrap();
        warn!("XXXX = {:?}", minmax_result.children);
        warn!("choose_piece_for_opponent b_move = {}", b_move);

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{piece::Piece, board::{Board, Cell, BoardIndex}, ai::{MinMax, minmax_tree::MinMaxTree, Strategy, get_moves, Score}};



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
    fn test_minmax_tree_compare() {
        let depth: usize = 0;
        let nb_piece_left = 10;
        let board = fill_board(nb_piece_left);
        warn!("{}", board);

        let piece_to_play = Piece::from_index(&board, 1).unwrap();
        info!("Should play : {}", &piece_to_play);
        // Tree minmax
        let mut minmax_tree = MinMaxTree::new(None, depth, true).with_piece(piece_to_play);

        let minmax_tree_result = minmax_tree.minmax(&board);

        info!("minmax_treeresult = {:?}", minmax_tree_result);
        // info!("If I play {} in cell num {}, score = {:?}",  &minmax_tree_result.c_move.unwrap().piece(), &minmax_tree_result.c_move.unwrap().cell(), &minmax_tree_result.score);
        //info!("If I play {} in cell num, score = {:?}",  &minmax_tree_result.c_move.unwrap().piece(), &minmax_tree_result.score);
        // Now play and check score
        //best_minmax_score = Win / best_first_move = (piece WEXS / cell num 16) / Current board score after best move = Point(45)
    }

    #[test]
    fn test_minmax_tree_2() {
        let depth: usize = 2;
        let nb_piece_left = 10;
        let board = fill_board(nb_piece_left);
        warn!("{}", board);

        let piece_to_play = Piece::from_index(&board, 1).unwrap();

        // Classic minmax
        let mut minmax = MinMax::new(depth, true);
        let minmax_score = MinMax::minmax(
            &board,
            minmax.depth,
            minmax.maximise,
            &get_moves(&board, None),
        );

        // Tree minmax
        let mut minmax_tree = MinMaxTree::new(None, depth, true).with_piece(piece_to_play);
        let mut minmax_tree_result = MinMaxTree::minmax_static(&board, minmax_tree.clone());
        //let res = MinMaxTree::minmax(&board, minmax_tree);

        info!("Classic minmax score = {:?}", minmax_score);
        info!("Tree minmax result = {:?}", minmax_tree_result);
        let minmax_calc_move = minmax.calc_move(&board, None).unwrap();
        let minmax_tree_calc_move = minmax_tree.calc_move(&board, Some(piece_to_play)).unwrap();

        info!("minmax_calc_move = {:?} / minmax_tree_calc_move = {}", minmax_calc_move, minmax_tree_calc_move);
        // info!(
        //     "TreeMinMax score = {:?} / children = {:?}",
        //     res, res.children
        // );
    }

    #[test]
    fn test_minmax_tree_choose_opponent_piece() {
        let depth: usize = 2;
        let nb_piece_left = 10;
        let board = fill_board(nb_piece_left);
        warn!("{}", board);

        //let piece_to_play = Piece::from_index(&board, 1).unwrap();

        // Classic minmax
        // let mut minmax = MinMax::new(depth, true);
        // let minmax_score = MinMax::minmax(
        //     &board,
        //     minmax.depth,
        //     minmax.maximise,
        //     &get_moves(&board, None),
        // );

        // Tree minmax
        let mut minmax_tree_result = MinMaxTree::new(None, depth, true);
        // let mut minmax_tree_result = MinMaxTree::minmax(&board, minmax_tree.clone());
        //let res = MinMaxTree::minmax(&board, minmax_tree);
        let piece = minmax_tree_result.choose_piece_for_opponent(&board);

        info!("Piece choose = {}", piece);
    }

    #[test]
    fn test_depth_0_eq_current_board_score() {
        let board = fill_board(5);
        let mut minmax = MinMaxTree::new(None, 0, true);
        assert_eq!(
            Score::calc_score(&board),
            minmax.minmax(&board).score
        );
    }
}