use quarto_game::{board::Board, game::Game};

fn main() {
    println!("Welcome to Quarto game");
    // let board = Board::init();

    // 'main : loop {
    //     // board.new_game();

    //     'game : loop {

    //     }

    //     println!("Start a new game ?");
    // }

    let mut game = Game::new("Romain", "Dana");
    // println!("{}", board);

    // let all_piece = board.get_available_pieces();
    // let selected_piece = all_piece.get(&0).unwrap();
    game.play_index(8, 0).expect("Zooob");
    game.play_index(4, 1).expect("Zooob");
    game.play_index(2, 2).expect("Zooob");
    game.play_index(3, 3).expect("Zooob");
    // game.play_index(10, 10).expect("Zooob");
    println!("{}", game.get_board());
    
}
