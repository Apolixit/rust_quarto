use crate::board::Board;

pub struct Game {
    pub board: Board,
    pub player_1: Player,
    pub player_2: Player
}

pub struct Player {
    pub name: String,
    pub score: Option<u8>
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            score: None
        }
    }
}

impl Game {
    pub fn new(p1_name: &str, p2_name: &str) -> Game {
        Game {
            player_1: Player::new(p1_name),
            player_2: Player::new(p2_name),
            board: Board::create()
        }
    }
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