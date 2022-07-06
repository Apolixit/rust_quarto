use ansi_term::{Colour, Style};
use quarto_game::{
    board::BoardState,
    game::{Game, PlayerType},
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
        // let p2_name = read_input_string("Player 2 name :");

        // let mut game = Game::new(p1_name.as_str(), p2_name.as_str());
        let mut game = Game::pve(p1_name.as_str());
        print!(
            "Ok {} and {}, let's start !",
            game.get_player(0),
            game.get_player(1)
        );

        'game: loop {
            println!("{}", game.get_board());

            loop {
                let piece_key: usize = match game.current_player().player_type() {
                    PlayerType::AI => read_input_index(
                        format!(
                            "{} choose a piece for {}\nEnter the piece number : ",
                            game.opponent_player(),
                            game.current_player()
                        )
                        .as_str(),
                    ),
                    PlayerType::HUMAN => {
                        println!("Searching a piece...");
                        let piece = game
                            .opponent_player()
                            .choose_piece_for_opponent(game.get_board());
                        println!("I choose {} for you", &piece);

                        game.get_board().get_piece_index(&piece).unwrap()
                    }
                };
                // let piece_key = read_input_index(
                //     format!(
                //         "{} choose a piece for {}\nEnter the piece number : ",
                //         game.opponent_player(),
                //         game.current_player()
                //     )
                //     .as_str(),
                // );

                // Check if the piece is always available
                if let Err(e) = game.get_board().get_piece_from_available(piece_key) {
                    println!("{}", e);
                    continue;
                }

                let piece_to_play = game
                    .get_board()
                    .get_piece_from_available(piece_key)
                    .unwrap();

                // let cell_key = read_input_index(
                //     format!(
                //         "{} on which case do you wanna play the piece {} ?",
                //         game.current_player(),
                //         piece_to_play
                //     )
                //     .as_str(),
                // );

                let cell_key: usize = match game.current_player().player_type() {
                    PlayerType::HUMAN => read_input_index(
                        format!(
                            "{} on which case do you wanna play the piece {} ?",
                            game.current_player(),
                            piece_to_play
                        )
                        .as_str(),
                    ),
                    PlayerType::AI => {
                        let move_selected = game
                            .current_player()
                            .choose_move(piece_to_play, game.get_board())
                            .unwrap();
                        println!("I play this on cell num {}", move_selected.index_cell() + 1);
                        move_selected.index_cell() + 1
                    }
                };

                if let Err(e) = game.play_index(piece_key, cell_key) {
                    println!("{}", e);
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

                    let win_position: Vec<usize> = winning_cells
                        .into_iter()
                        .map(|(position, _)| position + 1)
                        .collect();
                    println!(
                        "{} win the game with combinaison : {:?}",
                        Style::new()
                            .bold()
                            .underline()
                            .paint(game.current_player().to_string()),
                        win_position
                    );

                    break 'game;
                }
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
                "Impossible to convert {} to number, please try again (error = {})",
                s_input, e
            );
        } else {
            return n_input.unwrap() - 1;
        }
    }
}
