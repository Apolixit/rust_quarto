use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::format;
use std::fmt::Debug;
use std::fmt::Display;
use termtree::Tree;

use crate::ai::get_moves;
use crate::ai::play;
use crate::ai::Board;
use crate::ai::ErrorGame;
use crate::ai::Piece;
use crate::ai::Score;
use crate::ai::Strategy;
use crate::board::BoardIndex;
use crate::r#move::Move;
use std::cmp::max;
use std::cmp::min;

/// Structure which represent the MinMax algorigthm with score affected to each moves
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MinMaxTree {
    /// Piece which had to be played
    piece: Option<Piece>,
    /// Current move
    selected_move: Option<Move>,
    /// The MinMax score.
    /// Careful, this is not the score of the current board (except for leaves)
    score: Score,
    /// Current depth
    depth: usize,
    /// Do we maximize the score ?
    maximise: bool,
    /// The moves available from this board
    children: Vec<MinMaxTree>,
}

/// Implementation of PartialOrd and Ord to allow performing min / max comparison (based on the score)
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

/// Display the current tree state. also use to display the full tree (ref: to_tree() function)
impl Display for MinMaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prelude = if self.depth > 0 {
            Some(format!(
                "[{} - {}]",
                self.depth,
                if self.maximise {
                    "Maximising"
                } else {
                    "Minimising"
                }
            ))
        } else {
            None
        };
        write!(
            f,
            "{} {:?} = {}",
            prelude.unwrap_or("".to_string()),
            self.selected_move,
            self.score
        )
    }
}

impl MinMaxTree {
    /// Create a new MinMaxTree
    fn new(c_move: Option<Move>, depth: usize, maximise: bool) -> MinMaxTree {
        MinMaxTree {
            piece: None,
            selected_move: c_move,
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

    #[cfg(test)]
    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    /// Get the adequat move from the immediate children
    fn update_move_from_first_child(&mut self) {
        if self.selected_move.is_none() {
            self.selected_move = (*self.children)
                .into_iter()
                .find(|f| f.score == self.score)
                .unwrap()
                .selected_move
                .clone();
        }
    }

    /// Return moves available
    fn children_moves(&self, board: &Board) -> Vec<Move> {
        get_moves(board, self.piece)
    }

    #[cfg(test)]
    fn with_piece(mut self, piece: Piece) -> MinMaxTree {
        self.piece = Some(piece);
        self
    }

    /// MinMax algorithm with the build of the MinMaxTree tree.
    /// Enable log "debug" if you need informations.
    fn minmax(&mut self, board: &Board) {
        // End the recursivity if we can't go deeper
        if self.depth == 0 || !board.can_play_another_turn() {
            // We calc the final board score for the leaf
            self.score = Score::calc_score(board);

            debug!(
                "MinMaxTree {:?}, Can play other turn = {}",
                &self,
                board.can_play_another_turn()
            );
        } else {
            // Loop over each child node
            self.children_moves(&board).into_iter().for_each(|m| {
                let mut child = MinMaxTree::new(Some(m), self.depth - 1, !self.maximise);
                let previous_score = self.score; // Just for further logging

                // We play the current move
                let mut board = board.clone();
                play(&mut board, &child.selected_move.unwrap());

                // Call minmax recursivity on each children
                child.minmax(&board);

                // Get the max or min score, depend on the depth of the tree
                if self.maximise {
                    self.score = max(&mut *self, &mut child).score;
                } else {
                    self.score = min(&mut *self, &mut child).score;
                }

                debug!(
                    "{} / Move ({:?}) / depth = {} / Previous score = {:?} / Now score = {:?}",
                    if self.maximise {
                        "Maximising"
                    } else {
                        "Minimising"
                    },
                    child.selected_move.unwrap(),
                    self.depth,
                    previous_score,
                    self.score
                );
                self.children.push(child);
            });

            debug!(
                "MinMax depth = {} / maximising = {} / best score = {} / children_tree = {:?}",
                self.depth, self.maximise, self.score, self.children,
            );

            // We are here when the minmax has finished, we now need to update the move from children which is equal on the best score
            self.update_move_from_first_child();
        }
    }

    /// Display the MinMaxTree as a tree
    #[cfg(test)]
    fn to_tree(&self) -> Tree<&MinMaxTree> {
        let x = self.children.as_slice();
        let tree: Tree<&MinMaxTree> = x.into_iter().fold(Tree::new(self), |mut root, entry| {
            if entry.is_leaf() {
                root.push(Tree::new(entry));
            } else {
                root.push(entry.to_tree());
            }
            root
        });
        tree
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

        self.minmax(board);
        Ok(self.selected_move.unwrap())
    }

    fn choose_piece_for_opponent(&mut self, board: &Board) -> Piece {
        self.minmax(board);

        // Visit of the children to select the worst move
        let mut best_move_per_piece: HashMap<usize, Score> = HashMap::new();
        for minmax in &self.children {
            best_move_per_piece
                .entry(minmax.selected_move.unwrap().piece().to_index(board).unwrap())
                .and_modify(|score| *score = max(*score, minmax.score))
                .or_insert(minmax.score);
        }

        info!("best_move_per_piece = {:?}", best_move_per_piece);
        let worst_score = best_move_per_piece.into_iter().min_by_key(|k| k.1).unwrap();
        Piece::from_index(&board, worst_score.0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ai::{get_moves, minmax_tree::MinMaxTree, MinMax, Score, Strategy},
        board::{Board, BoardIndex, Cell},
        piece::Piece,
    };

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
        let depth: usize = 2;
        let nb_piece_left = 6;
        let board = fill_board(nb_piece_left);
        warn!("{}", board);

        let piece_to_play = Piece::from_index(&board, 1).unwrap();
        info!("Should play : {}", &piece_to_play);
        // Tree minmax
        let mut minmax_tree = MinMaxTree::new(None, depth, true).with_piece(piece_to_play);
        minmax_tree.minmax(&board);

        info!(
            "minmax best move = {} give score {}",
            minmax_tree.selected_move.unwrap(),
            minmax_tree.score
        );
        info!("MinMax tree result =  \n{}", minmax_tree.to_tree());
    }



    #[test]
    fn test_minmax_tree_choose_opponent_piece() {
        let depth: usize = 2;
        let nb_piece_left = 10;
        let board = fill_board(nb_piece_left);
        warn!("{}", board);

        // Tree minmax
        let mut minmax_tree = MinMaxTree::new(None, depth, true);
        let piece = minmax_tree.choose_piece_for_opponent(&board);

        info!("Piece choose = {}", piece);
    }

    #[test]
    fn test_depth_0_eq_current_board_score() {
        let board = fill_board(5);
        let mut minmax = MinMaxTree::new(None, 0, true);
        minmax.minmax(&board);

        assert_eq!(Score::calc_score(&board), minmax.score);
    }
}
