use crate::ai::get_moves;
use crate::ai::Board;
use crate::ai::ErrorGame;
use crate::ai::Piece;
use crate::ai::Score;
use crate::ai::Strategy;
use crate::board::BoardIndex;
use crate::r#move::Move;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
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
    pub fn name() -> String {
        String::from("MinMaxTree")
    }

    /// Create a new MinMaxTree
    pub fn new(depth: usize, maximise: bool) -> MinMaxTree {
        MinMaxTree {
            piece: None,
            selected_move: None,
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

    /// Create a new MinMaxTree with a predefined move
    pub fn from_move(c_move: Move, depth: usize, maximise: bool) -> MinMaxTree {
        MinMaxTree {
            piece: None,
            selected_move: Some(c_move),
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

    /// Reset the algo
    fn reset(&mut self) {
        if self.children.len() > 0 {
            *self = MinMaxTree::new(self.depth, self.maximise);
        }
    }

    #[cfg(test)]
    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    /// Get the adequat move from immediate children
    fn update_move_from_first_child(&mut self) {
        if self.selected_move.is_none() {
            trace!("No selected move, we want to find score = {}", self.score);
            for child in &self.children {
                trace!("update_move_from_first_child >> child score = {} / child move = {:?} / child depth = {}", child.score, child.selected_move, child.depth);
                if child.score == self.score {
                    self.selected_move = child.selected_move;
                }
            }

            trace!(
                "I have selected = {:?} with score = {}",
                self.selected_move,
                self.score
            );
            if self.selected_move.is_none() {
                error!("update_move_from_first_child : no best selected move have been found !");
            }
        }
    }

    /// Return moves available
    fn children_moves(&self, board: &Board) -> Vec<Move> {
        get_moves(board, self.piece)
    }

    /// Force to calc a move from a specific piece
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
                let mut child = MinMaxTree::from_move(m, self.depth - 1, !self.maximise);
                let previous_score = self.score; // Just for further logging

                // We play the current move
                let mut board = board.clone();
                board
                    .play_and_remove_piece(&child.selected_move.unwrap())
                    .unwrap();

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

                // Add the child
                self.children.push(child);
            });

            debug!(
                "MinMax depth = {} / maximising = {} / best score = {} / children_tree = {:?}",
                self.depth, self.maximise, self.score, self.children,
            );

            if self.selected_move.is_none() {
                // We are here when the minmax has finished, we now need to update the move from children which is equal on the best score
                self.update_move_from_first_child();
            }
        }
    }

    /// Display the MinMaxTree as a tree
    #[cfg(test)]
    fn as_tree(&self, display_leaf: bool) -> termtree::Tree<&MinMaxTree> {
        use termtree::Tree;

        let x = self.children.as_slice();
        let tree: Tree<&MinMaxTree> = x.into_iter().fold(Tree::new(self), |mut root, entry| {
            if display_leaf {
                if entry.is_leaf() {
                    root.push(Tree::new(entry));
                } else {
                    root.push(entry.as_tree(display_leaf));
                }
            } else {
                if entry.children.first().unwrap().depth > 0 {
                    root.push(entry.as_tree(display_leaf));
                } else {
                    root.push(Tree::new(entry));
                }
            }

            root
        });
        tree
    }

    /// The combinaison (nb_piece_already_played, depth to search)
    /// For example here:
    ///     - between 0 and 5 pieces, we search with depth = 2
    ///     - between 5 and 8 pieces, we search with depth = 3
    ///     - etc
    /// I try this to have a better algo in end game
    pub fn calc_adequat_depth(nb_piece_left: usize) -> usize {
        match nb_piece_left {
            0..=7 => 2,
            8..=10 => 3,
            11..=15 => 4,
            _ => 0,
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

impl Strategy for MinMaxTree {
    fn name(&self) -> String {
        MinMaxTree::name()
    }

    fn calc_move(&mut self, board: &Board, piece: Option<Piece>) -> Result<Move, ErrorGame> {
        self.reset();

        if piece.is_some() {
            self.piece = Some(piece.unwrap());
        }

        self.minmax(board);

        let selected_move = self.selected_move.unwrap();

        // if we passed a piece in parameter, the move selected by minmax should play this piece
        if piece.is_some() {
            assert_eq!(piece.unwrap(), selected_move.piece());
        }
        Ok(selected_move)
    }

    fn choose_piece_for_opponent(&mut self, board: &Board) -> Piece {
        let mut worst_score: (Option<usize>, Score) = (None, Score::Win);
        let initial_depth = self.depth;

        while worst_score.1 == Score::Win && self.depth > 0 {
            self.reset();

            // self.maximise = false;
            self.minmax(board);

            // Visit of the children to select the worst move
            let mut best_move_per_piece: HashMap<usize, Score> = HashMap::new();
            for minmax in &self.children {
                best_move_per_piece
                    .entry(
                        minmax
                            .selected_move
                            .unwrap()
                            .piece()
                            .to_index(board)
                            .unwrap(),
                    )
                    .and_modify(|score| *score = max(*score, minmax.score))
                    .or_insert(minmax.score);
            }

            trace!("best_move_per_piece = {:?}", best_move_per_piece);
            worst_score = best_move_per_piece
                .into_iter()
                .min_by_key(|k| k.1)
                .map(|k| (Some(k.0), k.1))
                .unwrap();

            if worst_score.1 == Score::Win {
                info!("All best move per piece with depth = {} are winning. We decrease depth to find a not winning play", self.depth());
                self.depth = self.depth - 1;
            }
        }

        self.depth = initial_depth;
        let piece = Piece::from_index(&board, worst_score.0.unwrap()).unwrap();
        info!("worst_score = {} which is piece = {}", worst_score.1, piece);
        piece
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ai::{minmax_tree::MinMaxTree, Score, Strategy},
        board::{Board, BoardIndex, Cell},
        piece::Piece,
        r#move::Move,
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
        while nb_piece_left > 0 {
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
        debug!("{}", board);

        let piece_to_play = Piece::from_index(&board, 1).unwrap();
        debug!("Should play : {}", &piece_to_play);
        // Tree minmax
        let mut minmax_tree = MinMaxTree::new(depth, true).with_piece(piece_to_play.clone());
        minmax_tree.minmax(&board);

        let selected_move = minmax_tree.selected_move.unwrap();
        debug!(
            "minmax best move = {} give score {}",
            selected_move, minmax_tree.score
        );

        assert_eq!(
            minmax_tree.calc_move(&board, Some(piece_to_play)).unwrap(),
            selected_move
        );

        debug!("MinMax tree result =  \n{}", minmax_tree.as_tree(true));
    }

    #[test]
    fn test_minmax_tree_choose_opponent_piece() {
        let depth: usize = 2;
        let nb_piece_left = 10;
        let board = fill_board(nb_piece_left);
        debug!("{}", board);

        // Tree minmax
        let mut minmax_tree = MinMaxTree::new(depth, true);
        let piece = minmax_tree.choose_piece_for_opponent(&board);

        debug!("Piece choose = {}", piece);
    }

    #[test]
    fn test_depth_0_eq_current_board_score() {
        let board = fill_board(5);
        let mut minmax = MinMaxTree::new(0, true);
        minmax.minmax(&board);

        assert_eq!(Score::calc_score(&board), minmax.score);
    }

    #[test]
    fn test_debug() {
        let mut board = Board::create();

        board
            .play_and_remove_piece(&Move::new(
                Piece::from("WETS"),
                Cell::from_index(&board, 2).unwrap(),
            ))
            .unwrap();
        board
            .play_and_remove_piece(&Move::new(
                Piece::from("DEXC"),
                Cell::from_index(&board, 5).unwrap(),
            ))
            .unwrap();
        board
            .play_and_remove_piece(&Move::new(
                Piece::from("DFTS"),
                Cell::from_index(&board, 7).unwrap(),
            ))
            .unwrap();

        debug!("{}", board);

        let mut algo = MinMaxTree::new(2, true);
        algo.minmax(&board);
        //info!("{}", algo.as_tree());
        let piece_opponent = algo.choose_piece_for_opponent(&board);
        debug!("piece_opponent = {}", piece_opponent);
        let worst_score = Score::Point(5);
        for m in board.get_available_moves_from_piece(piece_opponent) {
            let mut board_clone = board.clone();
            board_clone.play_and_remove_piece(&m).unwrap();
            let current_score = Score::calc_score(&board_clone);
            debug!(
                "Score board = {} / Worst score (choose_piece_for_opponent) = {}",
                current_score, worst_score
            );
            assert!(current_score <= worst_score);
        }
    }

    #[test]
    fn test_debug_2() {
        let mut board = Board::create();
        board.with_scenario(vec![
            Move::new(Piece::from("DETS"), Cell::from_index(&board, 5).unwrap()),
            Move::new(Piece::from("DFTS"), Cell::from_index(&board, 6).unwrap()),
            Move::new(Piece::from("WFXC"), Cell::from_index(&board, 9).unwrap()),
            Move::new(Piece::from("WETC"), Cell::from_index(&board, 10).unwrap()),
            Move::new(Piece::from("DFTC"), Cell::from_index(&board, 12).unwrap()),
            Move::new(Piece::from("WFTC"), Cell::from_index(&board, 13).unwrap()),
            Move::new(Piece::from("WFXS"), Cell::from_index(&board, 14).unwrap()),
            Move::new(Piece::from("DEXC"), Cell::from_index(&board, 15).unwrap()),
        ]);

        info!("{}", board);

        let mut algo = MinMaxTree::new(3, true);
        // let best_move = algo.calc_move(&board, Some(Piece::from("DETS")));
        let best_move = algo.calc_move(&board, None);
        info!("best_move = {}", best_move.unwrap());
        // algo.minmax(&board);
        // info!("{}", algo.as_tree(true));
        // let piece_opponent = algo.choose_piece_for_opponent(&board);
        // info!("piece_opponent = {}", piece_opponent);

        // let selected_move = algo.calc_move(&board, Some(piece_opponent)).unwrap();
        // info!("Play {} like this {}", selected_move.piece(), selected_move);
        // board.play(selected_move.piece(), selected_move.cell()).unwrap();
        // info!("Status {:?}", board.board_state())
    }

    #[test]
    fn bench_minimax() {
        let board = Board::create();
        let mut algo = MinMaxTree::new(2, true);

        let now = std::time::Instant::now();

        algo.minmax(&board);

        let elapsed = now.elapsed();
        info!("MinMax duration: {} ns ({} ms)", elapsed.as_nanos(), elapsed.as_millis());
    }
}
