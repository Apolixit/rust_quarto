use crate::{game::Board, pwet::Pwet};

mod game;

fn main() {
    println!("Welcome to Quarto game");
    let board = Board::init();

    'main : loop {
        board.new_game();

        'game : loop {

        }

        println!("Start a new game ?");
    }
}
