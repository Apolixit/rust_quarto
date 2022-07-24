//! # Quarto game in console

use ansi_term::{Colour, Style};
use log::error;
use quarto_game::{
    board::{BoardIndex, BoardState, Cell},
    game::Game,
    piece::Piece,
    player::{Human, PlayerType, AI}, error::ErrorGame,
};
use std::{io};

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
        let game_type = read_input_index(
            format!("Wanna play :\n 1. Player vs Player\n 2. Player vs AI").as_str(),
        ) + 1;

        let p1_name = read_input_string("Player 1 name :");
        let mut game = if game_type == 1 {
            let p2_name = read_input_string("Player 2 name :");
            Game::start(Human::new(p1_name.as_str()), Human::new(p2_name.as_str()))
        } else {
            Game::start(Human::new(p1_name.as_str()), AI::new())
        };

        print!(
            "Ok {} and {}, let's start !",
            game.get_player(0),
            game.get_player(1)
        );

        'game: loop {
            println!("{}", game.get_board());

            loop {
                let piece_to_play = choose_piece_for_opponent(&mut game);

                // Check if the piece is always available
                if let Err(e) = piece_to_play {
                    error!("{}", e);
                    continue;
                }

                let piece_ok = piece_to_play.unwrap();
                let cell_selected: Cell = play_piece_in_cell(&mut game, &piece_ok);

                if let Err(e) = game.play(piece_ok, cell_selected) {
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

/// Ask to choose a piece for opponent
fn choose_piece_for_opponent(game: &mut Game) -> Result<Piece, ErrorGame> {
    match game.opponent_player().player_type() {
        PlayerType::Human => Piece::from_index(game.get_board(), read_input_index(
            format!(
                "{} choose a piece for {}\nEnter the piece number : ",
                game.opponent_player(), game.current_player()
            )
            .as_str(),
        )),
        PlayerType::AI => {
            println!("{} is searching a piece...", game.opponent_player());
            let piece = game.opponent_player().choose_piece_for_opponent(game.get_board());
            println!(
                "{} choose {} for {}",
                game.opponent_player(),
                &piece,
                game.current_player()
            );

            Ok(piece)
        }
    }
}

/// Ask in which cell the piece has to be played
fn play_piece_in_cell(game: &mut Game, piece_to_play: &Piece) -> Cell {
    match game.current_player().player_type() {
        PlayerType::Human => Cell::from_index(
            game.get_board(),
            read_input_index(
                format!(
                    "{} on which case do you wanna play the piece {} ?",
                    game.current_player(),
                    piece_to_play
                )
                .as_str(),
            ),
        )
        .unwrap(),
        PlayerType::AI => {
            let move_selected = game
                .current_player()
                .choose_move(*piece_to_play, game.get_board())
                .unwrap();
            println!(
                "{} plays this on cell num {}",
                game.current_player(),
                move_selected.cell().to_index() + 1
            );
            move_selected.cell()
        }
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
