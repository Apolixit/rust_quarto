use std::collections::BTreeMap;
use log::info;
use yew::prelude::*;

pub enum BoardMessage {
    Click(usize),
}

#[derive(Debug)]
pub struct BoardGame;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct BoardGameProps {
    pub cells: BTreeMap<usize, quarto_game::board::Cell>,
    #[prop_or(true)]
    pub active: bool,
    pub on_cell_selected: Callback<usize>,
}

impl Component for BoardGame {
    type Message = BoardMessage;
    type Properties = BoardGameProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMessage::Click(index_cell) => {
                info!("You clicked on cell {}", index_cell);
                if ctx.props().active {
                    ctx.props().on_cell_selected.emit(index_cell);
                }
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut cells: Vec<usize> = vec![];
        (0..(ctx.props().cells.len()))
            .into_iter()
            .for_each(|i| cells.push(i));

        let display_board = html! {
            for ctx.props().clone().cells.into_iter().map(|(cell_index, current_cell)| {
                let mut cell_class = vec![
                    "board-cell",
                    "w-1/4 h-1/4 rounded-full border-2"];
                    if cell_index % 2 == 0 {
                        cell_class.push("cell-even");
                        cell_class.push("border-blue-200 hover:bg-blue-300");
                     } else {
                        cell_class.push("cell-odd");
                        cell_class.push("border-gray-200 hover:bg-gray-300");
                     }

                html! {
                    <div class={classes!(cell_class)} onclick={ctx.link().callback(move |_| BoardMessage::Click(cell_index))}>
                        {
                            if current_cell.piece().is_some() {
                                let current_piece = current_cell.piece().unwrap();
                                let piece_file = format!("static/{}.png", current_piece.as_text());

                                html! {
                                    <img src={piece_file} alt="Awesome image" class="w-8" />
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                }
            })
        };

        // rotate-45 mt-20 ml-20
        html! {
            <div class="board flex flex-wrap w-96 h-96 ">
            {
                display_board.clone()
            }
            </div>
        }
    }
}