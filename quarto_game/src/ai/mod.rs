pub use minmax::MinMax;
pub use score::Score;

use crate::{
    board::{Board, HEIGHT_BOARD, WIDTH_BOARD},
    error::ErrorGame,
    piece::Piece,
    r#move::Move,
};

use self::random::RandomAI;

mod minmax;
pub mod random;
mod score;

/// The nb move with RandomUI strategy. After this, we will use MinMax algorithm
const NB_PLAY_WITH_RANDOM_STRATEGY: usize = 3;

pub trait Strategy {
    fn name(&self) -> String;
    /// Return the best play from :
    /// - the current board state
    /// - the depth of the the search moves
    /// - Do we currently maximize the score ?
    /// - The piece to be played (it's specific to Quarto)
    fn calc_move(&self, board: &Board, piece: Option<Piece>) -> Result<Move, ErrorGame>;

    /// Chose the worst piece for the opponent
    fn choose_piece_for_opponent(&self, board: &Board) -> Piece;
}

/// Play the current move (eq to Game struct)
fn play(board: &mut Board, m: &Move) {
    if let Err(e) = board.play(m.piece(), m.cell()) {
        error!("{}", e.message());
    }

    board.remove(m.piece()).unwrap();
}

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
    let minmax = MinMax::new(MinMax::calc_adequat_depth(nb_piece_left), true);

    info!("Strategy is MinMax with depth = {}", minmax.depth());
    return Box::new(minmax);
}

#[cfg(test)]
mod tests {

    use crate::board::Cell;
use crate::ai::RandomAI;
use crate::ai::MinMax;
    use crate::ai::play;
use crate::ai::Piece;
use crate::r#move::Move;
use crate::ai::adequat_strategy;
    use crate::ai::Board;

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
            play(&mut board, &m);
        }

        let strategy = adequat_strategy(&board);
        trace!("Mid game strategy = {:?}", strategy.name());

        assert_eq!(strategy.name(), MinMax::name());
    }
}
