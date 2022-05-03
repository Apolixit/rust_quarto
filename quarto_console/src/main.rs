use quarto_game::game::Game;

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
}
