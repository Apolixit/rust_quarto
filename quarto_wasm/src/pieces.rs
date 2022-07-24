use std::collections::BTreeMap;

use log::info;
use quarto_game::{board::Board, piece::Piece};
use yew::{callback, prelude::*};

pub enum PieceMessage {
    Click(usize),
}

pub struct BoardPiece;

#[derive(PartialEq, Properties, Clone)]
pub struct BoardPieceProps {
    pub pieces: BTreeMap<usize, Piece>,
    #[prop_or(true)]
    pub active: bool,
    pub on_piece_selected: Callback<usize>,
}

impl Component for BoardPiece {
    type Message = PieceMessage;
    type Properties = BoardPieceProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PieceMessage::Click(index_piece) => {
                if ctx.props().active {
                    ctx.props().on_piece_selected.emit(index_piece)
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            for ctx.props().pieces.clone().into_iter().map(|(piece_index, piece)| {

                //info!("{}", piece.as_text());
                // let piece_file = format!("static/{}.png", piece.as_text());
                // <img src={piece_file} alt="Awesome image" class="w-8" onclick={ctx.link().callback(move |_| PieceMessage::Click(piece_index))} />

                html! {
                    <DisplayPiece
                        piece={piece}
                        on_piece_selected={ctx.link().callback(move |_| PieceMessage::Click(piece_index))} />
                }
            })
        }
    }
}

pub enum PieceMsg {
    Click,
}
#[derive(PartialEq, Properties)]
pub struct PieceProps {
    pub piece: Piece,
    #[prop_or(Callback::noop())]
    pub on_piece_selected: Callback<Piece>,
}

#[function_component(DisplayPiece)]
fn display_piece(PieceProps { piece, on_piece_selected }: &PieceProps) -> Html {
    let piece_file = format!("static/{}.png", piece.as_text());

    let on_piece_selected = on_piece_selected.clone();
    let piece = piece.clone();
    let on_piece_select =
        { Callback::from(move |_| on_piece_selected.emit(piece)) };

    html! {
        html! {
            <img src={piece_file} alt="Awesome image" class="w-8" onclick={on_piece_select} />
        }
    }
}
