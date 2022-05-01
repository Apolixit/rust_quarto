use std::fmt::Display;



use prettytable::{Table as pTable, Cell as pCell, Row as pRow};

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
        for color in Color::iterate() {
            for hole in Hole::iterate() {
                for height in Height::iterate() {
                    for shape in Shape::iterate() {
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

    // pub fn new_game(&self) {
    //     let mut cells_array = vec![vec![0; 4]; 4];
    // }

    // fn get_index(&self, row: usize, col: usize) -> usize {
    //     row * WIDTH_BOARD + col
    // }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = pTable::new();

        // Add a row per time
        table.add_row(pRow::new(vec![pCell::new("ABC"), pCell::new("DEFG"), pCell::new("HIJKLMN")]));
        table.add_row(pRow::new(vec![pCell::new("foobar"), pCell::new("bar"), pCell::new("foo")]));
        // A more complicated way to add a row:
        table.add_row(pRow::new(vec![
            pCell::new("foobar2"),
            pCell::new("bar2"),
            pCell::new("foo2")]));
        //eturn write!(f, "{}", table);

        // let mut display: String = String::from("");

        // for (i, cell) in self.cells.iter().enumerate() {
        //     table.add_row(pRow::new(vec![
        //         pCell::new("foobar2"),
        //         pCell::new("bar2"),
        //         pCell::new("foo2")]));

        //     // s = format!("XFPG\n");

        //     s = format!(
        //         "XFPG{}{}",
        //         if (i / HEIGHT_BOARD) < (HEIGHT_BOARD - 1) { "____" } else { "    " },
        //         if (i + 1) % WIDTH_BOARD == 0 { "\n" } else { "|" }
        //     );

        //     // display = format!("{}{}", display, s);
        // }
        return write!(f, "{}", table);
        // return write!(f, "{}", format!("\n{}\n", display));
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
            " {} ",
            if let Some(p) = &self.piece {
                p.to_string()
            } else {
                "  ".to_string()
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
}
