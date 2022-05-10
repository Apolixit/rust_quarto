use ansi_term::{Colour, Style};
use quarto_game::{
    board::{BoardState},
    game::Game,
};
use std::io;

fn main() {
    quarto_game::init();

    println!(
        "{}",
        Style::new()
            .fg(Colour::RGB(144, 255, 10))
            .bold()
            .paint("Welcome to Quarto game")
    );

    loop {
        let p1_name = read_input_string("Player 1 name :");
        let p2_name = read_input_string("Player 2 name :");

        let mut game = Game::new(p1_name.as_str(), p2_name.as_str());
        print!(
            "Ok {} and {}, let's start !",
            game.get_player(0),
            game.get_player(1)
        );

        'game: loop {
            println!("{}", game.get_board());

            loop {
                let piece_key = read_input_index(
                    format!(
                        "{} choose a piece for {}\nEnter the piece number : ",
                        game.opponent_player(),
                        game.current_player()
                    )
                    .as_str(),
                );

                let cell_key = read_input_index(
                    format!(
                        "{} on which case do you wanna play the piece {} ?",
                        game.current_player(),
                        game.get_board()
                            .get_piece_from_available(piece_key)
                            .unwrap()
                    )
                    .as_str(),
                );

                if let Err(e) = game.play_index(piece_key, cell_key) {
                    println!("Error : {}", e);
                    continue;
                } else {
                    break;
                }
            }

            match game.get_board().board_state() {
                BoardState::GameInProgress => {
                    //No winner, let's continue
                    game.switch_current_player();
                }
                BoardState::Win(winning_cells) => {
                    //We display the board for the last time to show the winning combinaison
                    println!("{}", game.get_board());

                    let win_position: Vec<usize> = winning_cells.into_iter().map(|f| f.0).collect();
                    println!(
                        "{} win the game with combinaison : {:?}",
                        Style::new()
                            .bold()
                            .underline()
                            .paint(game.current_player().to_string()),
                            win_position
                    );

                    break 'game;
                },
                BoardState::Draw => {
                    println!("Draw ! No winner for this game, well played.")
                }
            }
        }

        println!("Start a new game ?");
    }
}

/// Read the input from console and return a string
fn read_input_string(label: &str) -> String {
    let std_input = io::stdin();
    let mut name_buffer = String::new();

    loop {
        println!("{}", label);
        if let Err(e) = std_input.read_line(&mut name_buffer) {
            println!(
                "Invalid input {}, please try again (error = {})",
                name_buffer, e
            );
        } else {
            // name_buffer.truncate(name_buffer.len() - 1); //To remove \ns
            return name_buffer.trim().to_string();
        }
    }
}

/// Read the input from console and return a number
fn read_input_index(label: &str) -> usize {
    loop {
        let s_input = read_input_string(label);
        let n_input = s_input.parse::<usize>();

        if let Err(e) = n_input {
            println!(
                "Impossible de convert {} to number, please try again (error = {})",
                s_input, e
            );
        } else {
            return n_input.unwrap() - 1;
        }
    }
}
