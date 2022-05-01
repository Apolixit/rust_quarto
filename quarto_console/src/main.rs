use quarto_game::game::Game;
use ansi_term::{Colour::Red, Style};

fn main() {
    println!("Welcome to Quarto game");
    // let board = Board::init();

    // 'main : loop {
    //     // board.new_game();

    //     'game : loop {

    //     }

    //     println!("Start a new game ?");
    // }

    let game = Game::new("Romain", "Dana");
    println!("{}", game.board);



    // println!("This is in red: {}", Red.paint("a red string"));
    // println!("How about some {} and {}?",
    //      Style::new().bold().paint("bold"),
    //      Style::new().underline().paint("underline"));
}
