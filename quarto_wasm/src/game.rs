use crate::BoardGame;
use crate::BoardPiece;
use log::info;
use quarto_game::board::Cell;
use quarto_game::{
    board::BoardIndex,
    piece::Piece,
    player::{Human, Player, PlayerType, AI},
};
use yew::{html, Callback, Component, Properties};

pub enum GameMsg {
    PieceSelected(usize),
    PiecePlayed(usize),
    GameIsFinish,
}

pub enum GameState {
    ChoosePiece,
    PlayPiece,
}

pub struct Game {
    game: quarto_game::game::Game,
    state: GameState,
    selected_piece: Option<Piece>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct GameProps {
    pub p1_name: String,
    pub p1_type: PlayerType,
    pub p2_name: String,
    pub p2_type: PlayerType,
}

impl Game {
    pub fn create_player(p_name: String, p_type: PlayerType) -> Box<dyn Player> {
        match p_type {
            PlayerType::Human => Box::new(Human::new(p_name.as_str())),
            PlayerType::AI => Box::new(AI::new()),
        }
    }
}

impl Component for Game {
    type Message = GameMsg;
    type Properties = GameProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        info!("Game component created");
        let new_game = quarto_game::game::Game::start_dyn(
            Game::create_player(ctx.props().p1_name.clone(), ctx.props().p1_type.clone()),
            Game::create_player(ctx.props().p2_name.clone(), ctx.props().p2_type.clone()),
        );

        Self {
            game: new_game,
            state: GameState::ChoosePiece,
            selected_piece: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameMsg::PieceSelected(index_piece) => {
                info!(
                    "Gotcha ! {} / {}",
                    index_piece,
                    Piece::from_index(self.game.get_board(), index_piece).unwrap()
                );
                self.state = GameState::PlayPiece;
                self.selected_piece = Some(Piece::from_index(self.game.get_board(), index_piece).unwrap());
            }
            GameMsg::PiecePlayed(index_cell) => {
                self.game.play(self.selected_piece.unwrap(), Cell::from_index(self.game.get_board(), index_cell).unwrap()).unwrap();
                self.state = GameState::ChoosePiece;
            }
            GameMsg::GameIsFinish => {}
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let callback_message_piece_selected = ctx
            .link()
            .callback(|index_piece: usize| GameMsg::PieceSelected(index_piece));

        let on_piece_selected = Callback::from(move |index_piece| {
            // info!("From App, piece selected = ({} /)", index_piece);
            callback_message_piece_selected.emit(index_piece)
        });

        let callback_message_piece_played = ctx
            .link()
            .callback(|index_cell: usize| GameMsg::PiecePlayed(index_cell));

        let on_cell_selected = Callback::from(move |index_cell| {
            callback_message_piece_played.emit(index_cell);
        });

        let html_state = match self.state {
            GameState::ChoosePiece => match self.game.opponent_player().player_type() {
                PlayerType::Human => {
                    html! {
                        <>
                            {self.game.opponent_player().name()} { " choose a piece for " } {self.game.current_player().name()}
                        </>
                    }
                }
                PlayerType::AI => {
                    html! {
                        <>
                            {self.game.opponent_player().name()} { " is searching a piece" }
                        </>
                    }
                }
            },
            GameState::PlayPiece => match self.game.current_player().player_type() {
                PlayerType::Human => {
                    html! {
                        <>
                            <div>{self.game.opponent_player().name()} { " has choosen the piece " } {self.selected_piece.unwrap().as_text()}</div>
                            <div>{self.game.current_player().name()} { " where do you play the piece " } {self.selected_piece.unwrap().as_text()}</div>
                        </>
                    }
                }
                PlayerType::AI => {
                    html! {
                        <>
                            <div>{self.game.current_player().name()} { " is searching" }</div>
                        </>
                    }
                }
            },
        };

        html! {
            <>
                <div>
                    <h2>
                        {self.game.current_player().name()} { " vs " } {self.game.opponent_player().name()}
                    </h2>
                    <p>
                        { html_state }
                    </p>
                </div>
                <div>
                    <BoardGame
                        cells={self.game.get_board().get_cells().clone()}
                        {on_cell_selected} />
                </div>
                <div>

                    <BoardPiece
                        pieces={self.game.get_board().get_available_pieces().clone()}
                        {on_piece_selected} />
                </div>
            </>
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render && self.game.opponent_player().player_type() == PlayerType::AI {
            let selected_piece = self
                .game
                .opponent_player()
                .choose_piece_for_opponent(self.game.get_board());
            info!("{}", selected_piece);

            let callback_message_piece_selected = ctx.link().callback(|index_piece: usize| {
                GameMsg::PieceSelected(index_piece)
            });

            callback_message_piece_selected.emit(selected_piece.to_index(self.game.get_board()).unwrap());
        }
    }

    fn destroy(&mut self, ctx: &yew::Context<Self>) {}
}
