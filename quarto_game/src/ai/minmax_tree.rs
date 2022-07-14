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

    fn with_piece(mut self, piece: Piece) -> MinMaxTree {
        self.piece = Some(piece);
        self
    }

    fn minmax(board: &Board, minmax_tree: MinMaxTree) -> MinMaxTree {
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
                let mut board = board.clone();
                play(&mut board, &child_tree.c_move.unwrap());
                // let save_old_score = score;
                trace!("First child tree = {:?}", child_tree);
                // let child_minmax =
                //     MinMaxTree::new(child.c_move, score, self.depth - 1, !self.maximise);
                // self.children.push(child_minmax);

                score = max(score, MinMaxTree::minmax(&board, child_tree));
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

                score = min(score, MinMaxTree::minmax(&board, child_tree));
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
        // let minmax_tree = MinMaxTree::new(None, depth, true).with_piece(piece_to_play);
        if piece.is_some() { 
            self.piece = Some(piece.unwrap());
        }
        // MinMaxTree::minmax(board, minmax_tree)
        todo!()
    }

    fn choose_piece_for_opponent(&self, board: &Board) -> Piece {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_minmax_tree_2() {
        // let depth: usize = 2;
        // let nb_piece_left = 10;
        // let board = fill_board(nb_piece_left);
        // warn!("{}", board);

        // let piece_to_play = Piece::from_index(&board, 1).unwrap();

        // // Classic minmax
        // let minmax = MinMax::new(depth, true);
        // let minmax_score = MinMax::minmax(
        //     &board,
        //     minmax.depth,
        //     minmax.maximise,
        //     &get_moves(&board, None),
        // );
        // // let best_first_move = minmax.calc_move(&board, None).unwrap();
        // // Tree minmax
        // let minmax_tree = MinMaxTree::new(None, depth, true).with_piece(piece_to_play);
        // let res = MinMaxTree::minmax(&board, minmax_tree);

        // warn!("Classic minmax score = {:?}", minmax_score);
        // minmax.calc_move(&board, None).unwrap();
        // info!(
        //     "TreeMinMax score = {:?} / children = {:?}",
        //     res, res.children
        // );
    }
}