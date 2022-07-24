mod pieces;
mod board;
mod buttons;
mod game;

use game::{Game, GameProps};
use log::info;
use quarto_game::{board::{Board, BoardIndex}, player::{Human, AI, PlayerType}, piece::Piece};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::board::BoardGame;
use crate::pieces::BoardPiece;

pub enum AppMessage {
    NewPvpGame,
    NewPvAIGame,
}

pub struct App {
    pub game_props: Option<GameProps>
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // let piece_selected_callback = ctx.link().send_message(msg)

        Self { game_props: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::NewPvpGame => {
                self.game_props = Some(GameProps {
                    p1_name: "Apo".to_owned(),
                    p1_type: PlayerType::Human,
                    p2_name: "Opponent".to_owned(),
                    p2_type: PlayerType::Human
                });
                // self.game = Some(quarto_game::game::Game::start(Human::new("Apo"), Human::new("Opponent")));
                // self.state = Some(GameState::ChoosePiece);
            },
            AppMessage::NewPvAIGame => {
                self.game_props = Some(GameProps {
                    p1_name: "Apo".to_owned(),
                    p1_type: PlayerType::Human,
                    p2_name: "AI".to_owned(),
                    p2_type: PlayerType::AI
                });
                // self.game = Some(quarto_game::game::Game::start(Human::new("Apo"), AI::new()));
                // self.state = Some(GameState::ChoosePiece);
            },
            _ => {}
        }
        info!("A new game is started !");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let start_game = html! {
            <div class="flex flex-rox">
                <button type="button" class="basis-1/4 bg-blue-400" onclick={ctx.link().callback(|_| AppMessage::NewPvpGame)}>{ "Start game vs Human" }</button>
                <button type="button" class="basis-1/4 bg-blue-400" onclick={ctx.link().callback(|_| AppMessage::NewPvAIGame)}>{ "Start game vs AI" }</button>
            </div>
        };

        html! {
           <div class="flex flex-row">
                {
                    if self.game_props.is_none() {
                        html! { start_game.clone() }
                    } else {
                        let props = self.game_props.as_ref().unwrap();
                        html! {
                            <Game ..props.clone() />
                        }
                    }
                }
           </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Rust logs initialized");

    // let new_board = BoardGame {};
    yew::start_app::<App>();
}

// pub enum BoardMessage {
//     Click(usize),
// }

// #[derive(Debug)]
// pub struct BoardGame {}

// #[derive(Debug, Properties)]
// pub struct BoardGameProps {
//     cells: BTreeMap<usize, quarto_game::board::Cell>
// }

// impl Component for BoardGame {
//     type Message = BoardMessage;
//     type Properties = BoardGameProps;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             BoardMessage::Click(index_cell) => {
//                 info!("You clicked on cell {}", index_cell);
//             }
//         }

//         true
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let mut cells: Vec<usize> = vec![];
//         (0..(quarto_game::board::WIDTH_BOARD * quarto_game::board::HEIGHT_BOARD))
//             .into_iter()
//             .for_each(|i| cells.push(i));

//         let display_board = html! {
//             for cells.into_iter().map(|cell_index| {
//                 let mut cell_class = vec![
//                     "board-cell",
//                     "w-1/4 h-1/4 rounded"];
//                     if cell_index % 2 == 0 {
//                         cell_class.push("cell-even");
//                         cell_class.push("bg-blue-200 hover:bg-blue-300");
//                      } else {
//                         cell_class.push("cell-odd");
//                         cell_class.push("bg-gray-200 hover:bg-gray-300");
//                      }

//                 html! {
//                     <div class={classes!(cell_class)} onclick={ctx.link().callback(move |_| BoardMessage::Click(cell_index))}></div>
//                 }
//             })
//         };

//         // let display_row = html! {
//         //     for vertical.into_iter().map(|v| {
//         //         html! {
//         //             <div class="row">
//         //             { display_col.clone() }
//         //         </div>
//         //         }

//         //     })
//         // };

//         html! {
//             <div class="board flex flex-wrap w-96 h-96">
//             {
//                 display_board.clone()
//                 // for cells.into_iter().map(|cell| {
//                 //     let cell_class = vec!["board-cell", if cell % 2 == 0 { "cell-even" } else { "cell-odd" }];

//                 //     html! {
//                 //         <>
//                 //         // {
//                 //         //     if cell % quarto_game::board::WIDTH_BOARD == 0 {
//                 //         //         {"salut"}
//                 //         //     }
//                 //         //     else {

//                 //         //     }
//                 //         // }
//                 //         <div class={classes!(cell_class)} onclick={ctx.link().callback(move |_| BoardMessage::Click(cell))}>{ cell }</div>
//                 //         </>
//                 //     }
//                 // })
//                 // for horizontal.iter() {
//                 //     for vertical.iter() {
//                 //         <div onclick={ctx.link().callback(|_| BoardMessage::Click(width * height))}>Hello ({ width }x{ height }) </div>
//                 //     }
//                 // }
//             }
//             </div>
//         }
//     }
// }

// pub enum PieceMessage {}

// pub struct BoardPiece;

// #[derive(PartialEq, Properties)]
// pub struct BoardPieceProps {
//     pieces: BTreeMap<usize, Piece>
// }

// impl Component for BoardPiece {
//     type Message = PieceMessage;
//     type Properties = BoardPieceProps;

//     fn create(ctx: &Context<Self>) -> Self {
//         todo!()
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             "I'm the available piece"
//         }
//     }
// }
