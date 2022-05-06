use crate::{
    board::{Board},
    error::ErrorGame,
    piece::Piece,
};

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

pub struct Game {
    board: Board,
    players: [Player; 2],
    current_index_player: usize,
}

impl Game {
    fn add_player(p1: Player, p2: Player) -> [Player; 2] {
        [p1, p2]
    }
}

impl Game {
    pub fn new(p1_name: &str, p2_name: &str) -> Game {
        Game {
            board: Board::create(),
            players: Game::add_player(Player::new(p1_name), Player::new(p2_name)),
            current_index_player: 0,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_player(&self, index: usize) -> &Player {
        &self.players[index]
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current_index_player]
    }

    ///Switch the current player to the other
    pub fn switch_current_player(&mut self) {
        self.current_index_player = (self.current_index_player as isize - 1).abs() as usize;
    }

    ///Play a turn with cell selected
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
    }

    #[test]
    fn start_new_game_and_play_one_turn_with_struct() -> Result<(), ErrorGame> {
        const INDEX_PIECE: usize = 0;
        const INDEX_CELL: usize = 0;
        let mut game = Game::new("p1", "p2");

        let available_pieces = &game.board.get_available_pieces();
        //let cells = &game.board.get_cells();
        // println!("cells = {:?}", cells);
        let nb_initial_piece = available_pieces.len();

        let selected_piece = available_pieces.get(&INDEX_PIECE).unwrap();
        println!("selected_piece = {}", selected_piece);
        //let selected_cell = cells.get(&0).unwrap();

        // println!("selected_piece = {:?} / selected_cell = {:?}", selected_piece, selected_cell);

        game.play(
            selected_piece,
            INDEX_CELL,
        )?;

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

        game.play_index(
            0,
            0,
        )?;

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

        let piece_0 = game.play_index(
            0,
            0,
        )?;

        let piece_1 = game.play_index(
            1,
            5,
        )?;

        let piece_10 = game.play_index(
            10,
            12,
        )?;

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
}
