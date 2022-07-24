use crate::{
    board::{Board, Cell},
    error::ErrorGame,
    piece::Piece,
    player::{Human, Player},
    r#move::Move,
};

pub struct Game {
    /// The Quarto board
    board: Board,

    /// The 2 players of the game
    players: [Box<dyn Player>; 2],

    /// Current index player (I used index to avoid to borrow player and have to introduce lifetime)
    current_index_player: usize,
}

impl Game {
    /// Start a new game
    pub fn start<P1: Player + 'static, P2: Player + 'static>(p1: P1, p2: P2) -> Game {
        Game {
            board: Board::create(),
            players: [Box::new(p1), Box::new(p2)],
            current_index_player: 0,
        }
    }

    pub fn start_dyn(p1: Box<dyn Player>, p2: Box<dyn Player>) -> Game {
        Game {
            board: Board::create(),
            players: [p1, p2],
            current_index_player: 0,
        }
    }

    /// Borrow the board
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Borrow the board mutable
    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Get a player by his index
    pub fn get_player(&self, index: usize) -> &Box<dyn Player> {
        &self.players[index]
    }

    /// Get the current player
    pub fn current_player(&self) -> &Box<dyn Player> {
        &self.players[self.current_index_player]
    }

    /// Get the player which is not currently playing
    pub fn opponent_player(&self) -> &Box<dyn Player> {
        &self.players[(self.current_index_player as isize - 1).abs() as usize]
    }

    /// Switch the current player to the other
    pub fn switch_current_player(&mut self) {
        self.current_index_player = (self.current_index_player as isize - 1).abs() as usize;
    }

    /// Play a turn with cell selected
    pub fn play(&mut self, piece: Piece, cell: Cell) -> Result<Piece, ErrorGame> {
        Ok(self.board.play_and_remove_piece(&Move::new(piece, cell))?)
    }
}

/// Create game from two people names
impl From<(&str, &str)> for Game {
    fn from(p: (&str, &str)) -> Self {
        Game::start(Human::new(p.0), Human::new(p.1))
    }
}

#[cfg(test)]
mod tests {
    use crate::{player::{Human, AI}, board::BoardIndex};

    use super::*;

    /// Start a new game and check players name
    #[test]
    fn start_new_game() {
        let new_game = Game::start(Human::new("p1"), Human::new("p2"));
        assert_eq!(new_game.get_player(0).name(), "p1".to_string());
        assert_eq!(new_game.get_player(1).name(), "p2".to_string());

        assert_eq!(new_game.current_player().name(), "p1".to_string());
        assert_eq!(new_game.opponent_player().name(), "p2".to_string());

        let new_game = Game::start(Human::new("I'm human bro"), AI::new());
        assert_eq!(new_game.get_player(0).name(), "I'm human bro".to_string());
        assert_eq!(new_game.get_player(1).name(), AI::default_name());

        assert_eq!(
            new_game.current_player().name(),
            "I'm human bro".to_string()
        );
        assert_eq!(new_game.opponent_player().name(), AI::default_name());
    }

    #[test]
    fn start_new_game_and_play_one_turn_with_struct() -> Result<(), ErrorGame> {
        const INDEX_PIECE: usize = 0;
        const INDEX_CELL: usize = 0;
        let mut game = Game::from(("p1", "p2"));

        let available_pieces = game.board.get_available_pieces();
        let nb_initial_piece = available_pieces.len();

        let selected_piece = Piece::from_index(&game.board, INDEX_PIECE).unwrap();
        println!("selected_piece = {}", selected_piece);

        game.play(selected_piece, Cell::from_index(&game.board, INDEX_CELL)?)?;

        assert_eq!(
            game.board.get_available_pieces().len(),
            nb_initial_piece - 1
        );

        match game.board[0].piece() {
            Some(p) => {
                assert_eq!(p, selected_piece)
            }
            None => panic!("No piece found"),
        }
        Ok(())
    }

    #[test]
    fn start_new_game_and_play_one_turn_with_index() -> Result<(), ErrorGame> {
        let mut game = Game::from(("p1", "p2"));

        let available_pieces = game.board.get_available_pieces();
        let nb_initial_piece = available_pieces.len();

        let selected_piece = Piece::from_index(&game.board, 0).unwrap();

        game.play(selected_piece, Cell::from_index(&game.board, 0)?)?;

        assert_eq!(
            game.board.get_available_pieces().len(),
            nb_initial_piece - 1
        );
        match game.board[0].piece() {
            Some(p) => {
                assert_eq!(p, selected_piece)
            }
            None => panic!("No piece found"),
        }

        Ok(())
    }

    #[test]
    fn start_new_game_and_play_multiple_turn() -> Result<(), ErrorGame> {
        let mut game = Game::from(("p1", "p2"));

        // let get_piece = |i: usize| Piece::from_index(&game.board, i).unwrap();
        // let get_cell = |i: usize| Cell::from_index(&game.board, i).unwrap();

        let piece_0 = game.play(
            Piece::from_index(&game.board, 0).unwrap(),
            Cell::from_index(&game.board, 0).unwrap(),
        )?;

        let piece_1 = game.play(
            Piece::from_index(&game.board, 1).unwrap(),
            Cell::from_index(&game.board, 5).unwrap(),
        )?;

        let piece_10 = game.play(
            Piece::from_index(&game.board, 10).unwrap(),
            Cell::from_index(&game.board, 12).unwrap(),
        )?;

        match game.board[0].piece() {
            Some(p) => {
                assert_eq!(p, piece_0)
            }
            None => panic!("No piece found"),
        }

        match game.board[5].piece() {
            Some(p) => {
                assert_eq!(p, piece_1)
            }
            None => panic!("No piece found"),
        }

        match game.board[12].piece() {
            Some(p) => {
                assert_eq!(p, piece_10)
            }
            None => panic!("No piece found"),
        }

        Ok(())
    }

    /// Play the same piece two time, should fail
    #[test]
    fn start_new_game_and_try_to_play_multiple_time_same_piece_should_fail() -> Result<(), ErrorGame>
    {
        let mut game = Game::from(("p1", "p2"));
        let piece = Piece::from_index(&game.board, 0).unwrap();

        game.play(piece, Cell::from_index(&game.board, 0).unwrap())?;
        assert_eq!(
            game.play(piece, Cell::from_index(&game.board, 1).unwrap()),
            Err(ErrorGame::PieceDoesNotBelongPlayable)
        );

        Ok(())
    }

    /// Play two pieces in the same cell should fail
    #[test]
    fn start_new_game_and_try_to_play_multiple_piece_same_cell_should_fail() -> Result<(), ErrorGame>
    {
        let mut game = Game::from(("p1", "p2"));

        let piece_error = Piece::from_index(&game.board, 0);

        game.play(
            Piece::from_index(&game.board, 0).unwrap(),
            Cell::from_index(&game.board, 0).unwrap(),
        )?;
        let play_result = game.play(
            Piece::from_index(&game.board, 1).unwrap(),
            Cell::from_index(&game.board, 0).unwrap(),
        );

        let cell_error = Cell::from_coordinate(&game.board, 0, 0);

        assert_eq!(
            play_result,
            Err(ErrorGame::CellIsNotEmpty(
                cell_error.unwrap(),
                piece_error.unwrap()
            ))
        );

        Ok(())
    }

    /// Remove piece multiple time
    #[test]
    fn start_new_game_and_try_to_remove_multiple_piece_should_fail() -> Result<(), ErrorGame> {
        let mut game = Game::from(("p1", "p2"));

        let piece_to_remove = Piece::from_index(&game.board, 0)?;
        game.get_board_mut().remove(piece_to_remove)?;

        assert_eq!(
            game.get_board_mut().remove(piece_to_remove),
            Err(ErrorGame::PieceDoesNotBelongPlayable)
        );

        Ok(())
    }
}
