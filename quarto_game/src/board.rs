use ansi_term::Style;
use enum_iterator::IntoEnumIterator;
use std::{fmt::Display, ops::{Index, IndexMut}, panic};

use prettytable::{Cell as pCell, Row as pRow, Table as pTable};

use crate::piece::{Color, Height, Hole, Piece, PieceFeature, Shape};

const WIDTH_BOARD: usize = 4;
const HEIGHT_BOARD: usize = 4;

pub struct Board {
    /// The x16 cells of the board
    pub cells: Vec<Cell>,
    /// Pieces which has not been played yet
    pub available_pieces: Vec<Piece>,
}

impl Board {
    ///Create a new board to start a game
    pub fn create() -> Board {
        let mut cells: Vec<Cell> = Vec::with_capacity(WIDTH_BOARD * HEIGHT_BOARD);

        for i in 0..WIDTH_BOARD * HEIGHT_BOARD {
            cells.push(Cell {
                index: i,
                piece: None,
                background_color: if i % 2 == 0 {
                    CellColor::Black
                } else {
                    CellColor::White
                },
            })
        }

        Board {
            cells: cells,
            available_pieces: Board::generate_all_pieces(),
        }
    }

    ///Generate all pieces of the game
    fn generate_all_pieces() -> Vec<Piece> {
        let mut pieces = vec![];

        //Loop over the enums to get all the possibilities
        for color in Color::into_enum_iter() {
            for hole in Hole::into_enum_iter() {
                for height in Height::into_enum_iter() {
                    for shape in Shape::into_enum_iter() {
                        pieces.push(Piece::new(
                            color.to_owned(),
                            hole.to_owned(),
                            height.to_owned(),
                            shape.to_owned(),
                        ));
                    }
                }
            }
        }

        pieces
    }

    pub fn play_piece(&mut self, piece: &Piece, cell_index: usize) {
        self.cells[cell_index].piece = Some(*piece);
    }
    // fn get_index(&self, row: usize, col: usize) -> usize {
    //     row * WIDTH_BOARD + col
    // }
}

impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        if index > WIDTH_BOARD * HEIGHT_BOARD {
            panic!("Index out of bounds");
        }

        &self.cells[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > WIDTH_BOARD * HEIGHT_BOARD {
            panic!("Index out of bounds");
        }

        &mut self.cells[index]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pieces_feature = Color::to_vec_box()
            .into_iter()
            .map(|p| p)
            .chain(Hole::to_vec_box().into_iter().map(|p| p))
            .chain(Height::to_vec_box().into_iter().map(|p| p))
            .chain(Shape::to_vec_box().into_iter().map(|p| p));

        let mut legend = format!(
            "{}{}{}",
            "\n",
            Style::new().bold().underline().paint("Legend:"),
            "\n"
        );

        //Draw legend
        for (i, e) in pieces_feature.into_iter().enumerate() {
            legend = format!(
                "{} \t {}: {}",
                legend,
                e.color().paint(e.acronym()),
                e.name()
            );
            if (i + 1) % 2 == 0 {
                legend = format!("{}\n", legend);
            }
        }
        
        //Draw available piece
        legend = format!(
            "{}\n\n{}",
            legend,
            Style::new().bold().underline().paint("Piece available:")
        );
        let mut table_available_piece = pTable::new();
        let mut current_row = pRow::empty();

        for (i, piece) in self.available_pieces.iter().enumerate() {
            current_row.add_cell(pCell::new_align(
                format!("{:0>2}\n{}", i + 1, piece.to_string().as_str()).as_str(),
                prettytable::format::Alignment::CENTER,
            ));
            if (i + 1) % 8 == 0 && i != self.available_pieces.len() - 1 {
                table_available_piece.add_row(current_row);
                current_row = pRow::empty();
            }
        }
        table_available_piece.add_row(current_row);

        //Draw Board
        let mut table_board = pTable::new();
        current_row = pRow::empty();
        for (i, cell) in self.cells.iter().enumerate() {
            // current_row.add_cell(pCell::new(cell.to_string().as_str()));
            current_row.add_cell(pCell::new_align(
                format!("{:0>2}\n{}", i + 1, cell.to_string().as_str()).as_str(),
                prettytable::format::Alignment::CENTER,
            ));
            if (i + 1) % WIDTH_BOARD == 0 {
                table_board.add_row(current_row);
                current_row = pRow::empty();
            }
        }
        legend = format!("{}\n{}\n{}", legend, table_available_piece, table_board);
        return write!(f, "{}", legend);
    }
}

#[derive(Clone)]
pub struct Cell {
    pub index: usize,
    pub piece: Option<Piece>,
    pub background_color: CellColor,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if let Some(p) = &self.piece {
                p.to_string()
            } else {
                "    ".to_string()
            }
        )
    }
}
#[derive(Clone)]
pub enum CellColor {
    Black,
    White,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Quarto game has 16 piece
    #[test]
    fn test_create_all_pieces_should_has_16_elements() {
        assert_eq!(Board::generate_all_pieces().len(), 16);
    }

    /// All pieces should be unique
    #[test]
    fn test_all_pieces_should_be_unique() {
        let pieces = Board::generate_all_pieces();

        for current_piece in pieces.iter() {
            assert_eq!(pieces.iter().filter(|p| *p == current_piece).count(), 1);
        }
    }

    #[test]
    /// When create a new game, all cells must be empty
    fn test_create_new_board_should_be_empty() {
        assert_eq!(
            Board::create().cells.iter().all(|p| p.piece.is_none()),
            true
        );
    }

    /// When create a new game, all piece should be available to be played
    #[test]
    fn test_create_new_board_all_piece_should_be_available() {
        assert_eq!(Board::create().available_pieces.len(), 16);
    }

    ///Index board accessor
    #[test]
    #[should_panic]
    fn test_access_index_board_out_of_bounds_should_panic() {
        let mut board = Board::create();
        let cell_out_of_bound = &board[20];
    }
}
