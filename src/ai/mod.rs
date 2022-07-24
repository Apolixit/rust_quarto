pub use minmax::MinMax;
pub use score::Score;

use crate::{
    ai::minmax_tree::MinMaxTree,
    board::{Board, HEIGHT_BOARD, WIDTH_BOARD},
    error::ErrorGame,
    piece::Piece,
    r#move::Move,
};

use self::random::RandomAI;

mod minmax;
pub mod minmax_tree;
pub mod random;
mod score;

/// The nb move with RandomUI strategy. After this, we will use MinMax algorithm
const NB_PLAY_WITH_RANDOM_STRATEGY: usize = 2;

pub trait Strategy {
    fn name(&self) -> String;
    /// Return the best play from :
    /// - the current board state
    /// - the depth of the the search moves
    /// - Do we currently maximize the score ?
    /// - The piece to be played (it's specific to Quarto)
    fn calc_move(&mut self, board: &Board, piece: Option<Piece>) -> Result<Move, ErrorGame>;

    /// Chose the worst piece for the opponent
    fn choose_piece_for_opponent(&mut self, board: &Board) -> Piece;
}

/// Play the current move (eq to Game struct)
// fn play(board: &mut Board, m: &Move) {
//     if let Err(e) = board.play(m.piece(), m.cell()) {
//         error!("{}", e.message());
//     }

//     board.remove(m.piece()).unwrap();
// }

/// Return the current available moves from the board
fn get_moves(board: &Board, piece: Option<Piece>) -> Vec<Move> {
    if let Some(piece) = piece {
        board.get_available_moves_from_piece(piece)
    } else {
        board.get_available_moves()
    }
}

/// Return the adequat AI strategy, depend on board state
pub fn adequat_strategy(board: &Board) -> Box<dyn Strategy> {
    // If we are on the first three move, we select RandomAI to play
    if board.get_empty_cells().len() > (WIDTH_BOARD * HEIGHT_BOARD) - NB_PLAY_WITH_RANDOM_STRATEGY {
        info!("Current strategy is RandomAI");
        return Box::new(RandomAI::new());
    }

    // Otherwise we call MinMax algorithm
    let nb_piece_left = WIDTH_BOARD * HEIGHT_BOARD - board.get_available_pieces().len();
    let minmax = MinMaxTree::new(MinMaxTree::calc_adequat_depth(nb_piece_left), true);

    info!("Strategy is MinMaxTree with depth = {}", minmax.depth());
    return Box::new(minmax);
}

#[cfg(test)]
mod tests {

    use std::thread;
    use std::time;
    use std::time::Duration;
    use std::time::Instant;

    use crate::ai::adequat_strategy;
    use crate::ai::minmax_tree::MinMaxTree;
    use crate::ai::Board;
    use crate::ai::Piece;
    use crate::ai::RandomAI;
    use crate::board::BoardState;
    use crate::board::Cell;
    use crate::r#move::Move;

    #[test]
    fn test_init_strategy() {
        let board = Board::create();

        let strategy = adequat_strategy(&board);

        trace!("Init with strategy = {:?}", strategy.name());

        assert_eq!(strategy.name(), RandomAI::name());
    }

    #[test]
    fn test_mid_game_strategy() {
        let mut board = Board::create();

        let moves: Vec<Move> = vec![
            (Move::new(Piece::from("DFXC"), Cell::from_index(&board, 0).unwrap())),
            (Move::new(Piece::from("DETS"), Cell::from_index(&board, 0).unwrap())),
            (Move::new(Piece::from("WFTC"), Cell::from_index(&board, 6).unwrap())),
            (Move::new(Piece::from("DFTC"), Cell::from_index(&board, 7).unwrap())),
        ];
        for m in moves {
            board.play_and_remove_piece(&m).unwrap();
        }

        let strategy = adequat_strategy(&board);
        trace!("Mid game strategy = {:?}", strategy.name());

        assert_eq!(strategy.name(), MinMaxTree::name());
    }

    #[test]
    fn test_adequat_thinking_strategy() {
        const MAX_SECOND: u64 = 15;
        // Check if the strategy doesn't take too long during the game
        // Let's fight AI vs AI and check if they don't think too much time
        let mut board = Board::create();
        let mut now: Instant;
        let mut elapsed_choose_piece_for_opponent: Duration;
        let mut elapsed_calc_move: Duration;
        let mut i = 1;
        let mut datas = vec![];
        while board.board_state() == BoardState::GameInProgress {
            let mut ai = adequat_strategy(&board);
            now = Instant::now();
            let selected_piece = ai.choose_piece_for_opponent(&board);
            elapsed_choose_piece_for_opponent = now.elapsed();
            datas.push(format!("Turn num {}, choose_piece_for_opponent = {}ms", i, elapsed_choose_piece_for_opponent.as_millis()));

            if elapsed_choose_piece_for_opponent.as_secs() > MAX_SECOND {
                error!(
                    "choose_piece_for_opponent duration exceed {}s ({}s) !",
                    MAX_SECOND,
                    elapsed_choose_piece_for_opponent.as_secs()
                );
                assert!(false);
            }
            info!("Round {}, selected_piece for opponent = {}", i, selected_piece);

            now = Instant::now();
            let selected_move = ai.calc_move(&board, Some(selected_piece)).unwrap();
            elapsed_calc_move = now.elapsed();
            datas.push(format!("Turn num {}, calc_move = {}ms", i, elapsed_calc_move.as_millis()));

            if elapsed_calc_move.as_secs() > MAX_SECOND {
                error!(
                    "calc_move duration exceed {}s ({}s) !",
                    MAX_SECOND,
                    elapsed_calc_move.as_secs()
                );
                assert!(false);
            }
            info!("Round {}, best move from piece {} = {}", i, selected_piece, selected_move);

            board.play_and_remove_piece(&selected_move).unwrap();

            info!("{}", board);

            info!("Round num {}, time spent choose_piece_for_opponent = {}ms, time spent calc_move = {}ms", i, elapsed_choose_piece_for_opponent.as_millis(), elapsed_calc_move.as_millis());
            i += 1;
            thread::sleep(time::Duration::from_secs(2));
        }

        info!("{:?}", datas);
    }
}
