use std::fmt::Display;

use crate::{board::Board, error::ErrorGame, piece::Piece};

/// Represent a player (humain or AI)
pub struct Player {
    pub name: String,
    pub score: Option<u8>,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            score: None,
        }
    }
}

/// Display the name and score of the player
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Game {
    /// The Quarto board
    board: Board,

    /// The 2 players of the game
    players: [Player; 2],

    /// Current index player (I used index to avoid to borrow player and have to introduce lifetime)
    current_index_player: usize,
}

impl Game {
    /// Add player to the game
    fn add_player(p1: Player, p2: Player) -> [Player; 2] {
        [p1, p2]
    }
}

impl Game {
    /// Start a new game
    pub fn new(p1_name: &str, p2_name: &str) -> Game {
        Game {
            board: Board::create(),
            players: Game::add_player(Player::new(p1_name), Player::new(p2_name)),
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
    pub fn get_player(&self, index: usize) -> &Player {
        &self.players[index]
    }

    /// Get the current player
    pub fn current_player(&self) -> &Player {
        &self.players[self.current_index_player]
    }

    /// Get the player which is not currently playing
    pub fn opponent_player(&self) -> &Player {
        &self.players[(self.current_index_player as isize - 1).abs() as usize]
    }

    /// Switch the current player to the other
    pub fn switch_current_player(&mut self) {
        self.current_index_player = (self.current_index_player as isize - 1).abs() as usize;
    }

    /// Play a turn with cell selected
    pub fn play(&mut self, piece: &Piece, cell_index: usize) -> Result<Piece, ErrorGame> {
        let piece_index = self
            .board
            .get_available_pieces()
            .into_iter()
            .position(|pos| pos.1 == *piece)
            .ok_or(ErrorGame::PieceDoesNotExists)?;

        self.play_index(piece_index, cell_index)
    }

    ///Play a turn with index cell
    pub fn play_index(&mut self, piece_key: usize, cell_key: usize) -> Result<Piece, ErrorGame> {
        //Play the piece
        self.board.play_piece(piece_key, cell_key)?;

        //Remove piece from available piece pool
        let piece_removed = self.board.remove_piece(piece_key)?;

        Ok(piece_removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_new_game() {
        let new_game = Game::new("p1", "p2");
        assert_eq!(new_game.get_player(0).name, "p1".to_string());
        assert_eq!(new_game.get_player(1).name, "p2".to_string());

        assert_eq!(new_game.current_player().name, "p1".to_string());
        assert_eq!(new_game.opponent_player().name, "p2".to_string());
    }

    #[test]
    fn start_new_game_and_play_one_turn_with_struct() -> Result<(), ErrorGame> {
        const INDEX_PIECE: usize = 0;
        const INDEX_CELL: usize = 0;
        let mut game = Game::new("p1", "p2");

        let available_pieces = &game.board.get_available_pieces();
        let nb_initial_piece = available_pieces.len();

        let selected_piece = available_pieces.get(&INDEX_PIECE).unwrap();
        println!("selected_piece = {}", selected_piece);

        game.play(selected_piece, INDEX_CELL)?;

        assert_eq!(
            game.board.get_available_pieces().len(),
            nb_initial_piece - 1
        );

        match game.board[0].piece {
            Some(p) => {
                assert_eq!(&p, selected_piece)
            }
            None => panic!("No piece found"),
        }
        Ok(())
    }

    #[test]
    fn start_new_game_and_play_one_turn_with_index() -> Result<(), ErrorGame> {
        let mut game = Game::new("p1", "p2");

        let available_pieces = &game.board.get_available_pieces();
        let nb_initial_piece = available_pieces.len();

        let selected_piece = available_pieces.get(&0).unwrap();

        game.play_index(0, 0)?;

        assert_eq!(
            game.board.get_available_pieces().len(),
            nb_initial_piece - 1
        );
        match game.board[0].piece {
            Some(p) => {
                assert_eq!(&p, selected_piece)
            }
            None => panic!("No piece found"),
        }

        Ok(())
    }

    #[test]
    fn start_new_game_and_play_multiple_turn() -> Result<(), ErrorGame> {
        let mut game = Game::new("p1", "p2");

        let piece_0 = game.play_index(0, 0)?;

        let piece_1 = game.play_index(1, 5)?;

        let piece_10 = game.play_index(10, 12)?;

        match game.board[0].piece {
            Some(p) => {
                assert_eq!(p, piece_0)
            }
            None => panic!("No piece found"),
        }

        match game.board[5].piece {
            Some(p) => {
                assert_eq!(p, piece_1)
            }
            None => panic!("No piece found"),
        }

        match game.board[12].piece {
            Some(p) => {
                assert_eq!(p, piece_10)
            }
            None => panic!("No piece found"),
        }

        Ok(())
    }

    /// Play the first piece in case num 25 (outside the board)
    #[test]
    fn start_new_game_and_try_to_play_piece_out_of_board_should_fail() -> Result<(), ErrorGame> {
        let mut game = Game::new("p1", "p2");

        assert_eq!(game.play_index(0, 25), Err(ErrorGame::IndexOutOfBound));
        assert_eq!(game.play_index(0, 16), Err(ErrorGame::IndexOutOfBound));
        Ok(())
    }

    /// Play the same piece two time, should fail
    #[test]
    fn start_new_game_and_try_to_play_multiple_time_same_piece_should_fail() -> Result<(), ErrorGame>
    {
        let mut game = Game::new("p1", "p2");

        game.play_index(0, 0)?;
        assert_eq!(
            game.play_index(0, 1),
            Err(ErrorGame::PieceDoesNotBelongPlayable)
        );

        Ok(())
    }

    /// Play two pieces in the same cell should fail
    #[test]
    fn start_new_game_and_try_to_play_multiple_piece_same_cell_should_fail() -> Result<(), ErrorGame>
    {
        let mut game = Game::new("p1", "p2");

        game.play_index(0, 0)?;
        assert_eq!(
            game.play_index(1, 0),
            Err(ErrorGame::CellIsNotEmpty)
        );

        Ok(())
    }

    /// Remove piece multiple time
    #[test]
    fn start_new_game_and_try_to_remove_multiple_piece_should_fail() -> Result<(), ErrorGame>
    {
        let mut game = Game::new("p1", "p2");

        game.get_board_mut().remove_piece(0)?;

        assert_eq!(
            game.get_board_mut().remove_piece(0),
            Err(ErrorGame::PieceDoesNotExists)
        );

        Ok(())
    }
}
