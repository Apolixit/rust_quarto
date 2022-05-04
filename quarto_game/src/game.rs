use crate::{
    board::{Board, Cell},
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
    pub board: Board,
    pub player_1: Player,
    pub player_2: Player,
}

impl Game {
    pub fn new(p1_name: &str, p2_name: &str) -> Game {
        Game {
            player_1: Player::new(p1_name),
            player_2: Player::new(p2_name),
            board: Board::create(),
        }
    }

    pub fn play(&self, piece: &Piece, cell: &Cell) {
        //*mut self.play_index(piece, cell.index);
    }

    pub fn play_index(&mut self, piece: &Piece, cell_index: usize) {
        self.board[cell_index].piece = Some(*piece);
    }

    pub fn choose_piece_opponent(&self, piece: &Piece) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_new_game() {
        let new_game = Game::new("p1", "p2");
        assert_eq!(new_game.player_1.name, "p1".to_string());
        assert_eq!(new_game.player_2.name, "p2".to_string());
    }
}
