use std::fmt::Display;

use crate::{
    ai::{self, Strategy},
    board::{Board, BoardIndex, Cell},
    error::ErrorGame,
    piece::Piece,
    r#move::Move,
};

pub enum PlayerType {
    HUMAN,
    AI,
}

pub trait Player {
    /// The current player name
    fn name(&self) -> String;

    /// Explicit enum to declare player as Human or AI
    fn player_type(&self) -> PlayerType;

    fn choose_move(&self, piece: Piece, board: &Board) -> Result<Move, ErrorGame>;

    fn choose_piece_for_opponent(&self, board: &Board) -> Piece;

    // /// Func to decide which move to play
    // fn choose_move<F>(&self, piece: &Piece, board: &Board, f: F) -> Result<Move, ErrorGame>
    // where
    //     F: Fn() -> usize;

    // /// Func to decide which piece to give to his opponent
    // fn choose_piece_for_opponent<F>(&self, board: &Board, f: F) -> Piece
    // where
    //     F: Fn() -> usize;
}

/// Represent a player (humain or AI)
pub struct Human {
    name: String,
}

impl Human {
    pub fn new(name: &str) -> Human {
        Human {
            name: name.to_string(),
        }
    }
}

impl Player for Human {
    fn name(&self) -> String {
        String::from(&self.name)
    }

    fn player_type(&self) -> PlayerType {
        PlayerType::HUMAN
    }

    fn choose_move(&self, piece: Piece, board: &Board) -> Result<Move, ErrorGame> {
        unimplemented!()
    }

    fn choose_piece_for_opponent(&self, board: &Board) -> Piece {
        unimplemented!()
    }
}

impl Display for dyn Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone)]
pub struct AI {
    name: String,
}

impl AI {
    pub fn new() -> AI {
        AI {
            name: AI::default_name(),
        }
    }

    pub fn default_name() -> String {
        String::from("AI")
    }

    // /// Func to decide which move to play
    // pub fn choose_move(&self, piece: Piece, board: &Board) -> Result<Move, ErrorGame> {
    //     info!("Calc move with depth = {}", 2);
    //     ai::MinMax::calc_move(board, 2, true, Some(piece))
    // }

    // /// Func to decide which piece to give to his opponent
    // pub fn choose_piece_for_opponent(&self, board: &Board) -> Piece {
    //     info!("Searching piece with depth = {}", 1);
    //     ai::MinMax::calc_worst_piece(board, 2).to_owned()
    // }
}

impl Player for AI {
    fn name(&self) -> String {
        String::from(&self.name)
    }

    fn player_type(&self) -> PlayerType {
        PlayerType::AI
    }

    fn choose_move(&self, piece: Piece, board: &Board) -> Result<Move, ErrorGame> {
        ai::MinMax::calc_move(board, 2, true, Some(piece))
    }

    fn choose_piece_for_opponent(&self, board: &Board) -> Piece {
        ai::MinMax::choose_piece_for_opponent(board, 2).to_owned()
    }
}

pub struct Game {
    /// The Quarto board
    board: Board,

    /// The 2 players of the game
    players: [Box<dyn Player>; 2],

    /// Current index player (I used index to avoid to borrow player and have to introduce lifetime)
    current_index_player: usize,
}

impl Game {
    /// Add player to the game
    fn add_player<P1: Player + 'static, P2: Player + 'static>(
        p1: P1,
        p2: P2,
    ) -> [Box<dyn Player>; 2] {
        [Box::new(p1), Box::new(p2)]
    }
}

impl Game {
    /// Start a new game
    pub fn pvp(p1_name: &str, p2_name: &str) -> Game {
        Game {
            board: Board::create(),
            players: Game::add_player(Human::new(p1_name), Human::new(p2_name)),
            current_index_player: 0,
        }
    }

    pub fn pve(p1_name: &str) -> Game {
        Game {
            board: Board::create(),
            players: Game::add_player(Human::new(p1_name), AI::new()),
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
        self.board.play(piece, cell)?;
        //Remove piece from available piece pool
        let piece_removed = self.board.remove(piece)?;
        Ok(piece_removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_new_game() {
        let new_game = Game::pvp("p1", "p2");
        assert_eq!(new_game.get_player(0).name(), "p1".to_string());
        assert_eq!(new_game.get_player(1).name(), "p2".to_string());

        assert_eq!(new_game.current_player().name(), "p1".to_string());
        assert_eq!(new_game.opponent_player().name(), "p2".to_string());

        let new_game = Game::pve("I'm human bro");
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
        let mut game = Game::pvp("p1", "p2");

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
        let mut game = Game::pvp("p1", "p2");

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
        let mut game = Game::pvp("p1", "p2");

        // let get_piece = |i: usize| Piece::from_index(&game.board, i).unwrap();
        // let get_cell = |i: usize| Cell::from_index(&game.board, i).unwrap();

        let piece_0 = game.play(Piece::from_index(&game.board, 0).unwrap(), Cell::from_index(&game.board, 0).unwrap())?;

        let piece_1 = game.play(Piece::from_index(&game.board, 1).unwrap(), Cell::from_index(&game.board, 5).unwrap())?;

        let piece_10 = game.play(Piece::from_index(&game.board, 10).unwrap(), Cell::from_index(&game.board, 12).unwrap())?;

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
        let mut game = Game::pvp("p1", "p2");
        let piece = Piece::from_index(&game.board, 0).unwrap();

        game.play(
            piece,
            Cell::from_index(&game.board, 0).unwrap(),
        )?;
        assert_eq!(
            game.play(
                piece,
                Cell::from_index(&game.board, 1).unwrap()
            ),
            Err(ErrorGame::PieceDoesNotBelongPlayable)
        );

        Ok(())
    }

    /// Play two pieces in the same cell should fail
    #[test]
    fn start_new_game_and_try_to_play_multiple_piece_same_cell_should_fail() -> Result<(), ErrorGame>
    {
        let mut game = Game::pvp("p1", "p2");

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
        let mut game = Game::pvp("p1", "p2");

        let piece_to_remove = Piece::from_index(&game.board, 0)?;
        game.get_board_mut().remove(piece_to_remove)?;

        assert_eq!(
            game.get_board_mut().remove(piece_to_remove),
            Err(ErrorGame::PieceDoesNotBelongPlayable)
        );

        Ok(())
    }
}
