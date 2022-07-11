pub use minmax::MinMax;
pub use score::Score;

use crate::{board::{Board, WIDTH_BOARD, HEIGHT_BOARD}, error::ErrorGame, piece::Piece, r#move::Move};

use self::random::RandomAI;

mod minmax;
mod score;
pub mod random;

const NB_PLAY_WITH_RANDOM_STRATEGY: usize = 3;

pub trait Strategy {
    /// Return the best play from :
    /// - the current board state
    /// - the depth of the the search moves
    /// - Do we currently maximize the score ?
    /// - The piece to be played (it's specific to Quarto)
    fn calc_move(
        board: &Board,
        depth: usize,
        maximise: bool,
        piece: Option<Piece>,
    ) -> Result<Move, ErrorGame>;

    /// Chose the worst piece for the opponent
    fn choose_piece_for_opponent(board: &Board, depth: usize) -> Piece;
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

