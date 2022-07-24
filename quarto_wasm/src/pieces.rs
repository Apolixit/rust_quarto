use std::collections::BTreeMap;

use log::info;
use quarto_game::piece::Piece;
use yew::{prelude::*, callback};

pub enum PieceMessage {
    Click(usize)
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
                ctx.props().on_piece_selected.emit(index_piece)
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            for ctx.props().pieces.clone().into_iter().map(|(piece_index, piece)| {
                //info!("{}", piece.as_text());
                let piece_file = format!("static/{}.png", piece.as_text());

                // html! { format!("Coucou, je suis la piece {}{}{}{}", piece.color, piece.hole, piece.height, piece.shape).as_str() }
                html! {
                    <img src={piece_file} alt="Awesome image" class="w-8" onclick={ctx.link().callback(move |_| PieceMessage::Click(piece_index))} />
                }
            })
        }
    }
}
