use crate::error::ErrorGame;
use ansi_term::Style;
use enum_iterator::IntoEnumIterator;
use log::{error, info};
use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Index, IndexMut},
    panic, borrow::BorrowMut,
};

use prettytable::{Cell as pCell, Row as pRow, Table as pTable};

use crate::piece::{Color, Height, Hole, Piece, PieceFeature, Shape};

const WIDTH_BOARD: usize = 4;
const HEIGHT_BOARD: usize = 4;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    /// The x16 cells of the board
    cells: BTreeMap<usize, Cell>,
    /// Pieces which has not been played yet
    available_pieces: BTreeMap<usize, Piece>,
}

impl Board {
    ///Generate all pieces of the game
    fn generate_all_pieces() -> BTreeMap<usize, Piece> {
        let mut pieces = BTreeMap::new();
        let mut key = 0;

        //Loop over the enums to get all the possibilities
        for color in Color::into_enum_iter() {
            for hole in Hole::into_enum_iter() {
                for height in Height::into_enum_iter() {
                    for shape in Shape::into_enum_iter() {
                        pieces.insert(key, Piece::new(color, hole, height, shape));
                        key += 1;
                    }
                }
            }
        }

        pieces
    }

    /// Create the board with (WIDTH_BOARD * HEIGHT_BOARD) empty cells
    fn generate_all_cells() -> BTreeMap<usize, Cell> {
        let mut cells = BTreeMap::new();
        let mut key: usize = 0;

        for i in 0..WIDTH_BOARD * HEIGHT_BOARD {
            cells.insert(
                key,
                Cell {
                    piece: None,
                    background_color: if i % 2 == 0 {
                        CellColor::Black
                    } else {
                        CellColor::White
                    },
                },
            );

            key += 1;
        }

        cells
    }

    /// Return index from (x; y) coordinate
    fn get_index(x: usize, y: usize) -> Option<usize> {
        if x >= WIDTH_BOARD || y >= HEIGHT_BOARD {
            return None;
        }
        Some(y * HEIGHT_BOARD + x)
    }

    /// Return (x; y) from index
    fn get_cell_coordinate(index: usize) -> Option<(usize, usize)> {
        if index >= WIDTH_BOARD * HEIGHT_BOARD {
            return None;
        }
        Some((index % WIDTH_BOARD, index / HEIGHT_BOARD))
    }
}

impl Board {
    ///Create a new board to start a game
    pub fn create() -> Board {
        Board {
            cells: Board::generate_all_cells(),
            available_pieces: Board::generate_all_pieces(),
        }
    }

    pub fn play_piece(
        &mut self,
        piece_index: usize,
        cell_index: usize,
    ) -> Result<Piece, ErrorGame> {
        if !self.cells.contains_key(&cell_index) {
            error!("Try to play in cell num {} - out of bound", cell_index);
            return Err(ErrorGame::IndexOutOfBound);
        }

        let piece = self
            .available_pieces
            .get(&piece_index)
            .ok_or(ErrorGame::PieceDoesNotBelongPlayable)?;

        info!(
            "Cell before playing : {}",
            self.cells.get(&cell_index).unwrap()
        );

        self.cells
            .entry(cell_index)
            .and_modify(|f| f.piece = Some(*piece));

        info!(
            "Cell after playing : {}",
            self.cells.get(&cell_index).unwrap()
        );

        Ok(*piece)
    }

    pub fn remove_piece(&mut self, index: usize) -> Result<Piece, ErrorGame> {
        info!("Piece num {} remove from availables", index);
        self.available_pieces
            .remove(&index)
            .ok_or(ErrorGame::PieceDoesNotExists)
    }

    pub fn get_available_pieces(&self) -> BTreeMap<usize, Piece> {
        self.available_pieces.clone()
    }

    pub fn get_piece_from_available(&self, index: usize) -> Result<&Piece, ErrorGame> {
        self.available_pieces
            .get(&index)
            .ok_or(ErrorGame::PieceDoesNotBelongPlayable)
    }
    pub fn get_piece_index(&self, piece: &Piece) -> Result<usize, ErrorGame> {
        self.get_available_pieces()
            .into_iter()
            .position(|pos| pos.1 == *piece)
            .ok_or(ErrorGame::PieceDoesNotExists)
    }

    pub fn get_cells(&self) -> &BTreeMap<usize, Cell> {
        &self.cells
    }

    pub fn is_board_winning(&self) -> Option<Vec<&Cell>> {
        //Horizontal check
        'x_x: for i in 0..WIDTH_BOARD {
            // println!("x_x");
            let mut horizontal_cells: Vec<&Cell> = Vec::with_capacity(HEIGHT_BOARD);
            'y_x: for j in 0..HEIGHT_BOARD {
                //If the cell is empty -> break this loop iteration
                let current_cell = self.cells.get(&Board::get_index(j, i).unwrap()).unwrap();
                // println!("Current cell : {} / current index : {}", current_cell, Board::get_index(j, i).unwrap());

                if let None = current_cell.piece {
                    // println!("Cell empty, break");
                    break 'y_x;
                }
                horizontal_cells.push(current_cell);
            }

            // if horizontal_cells.len() == WIDTH_BOARD
            if Board::check_cell_is_winning(&mut horizontal_cells) {
                // println!("Winnnn horizontal_cells");
                return Some(horizontal_cells);
            }
        }

        'x_y: for i in 0..WIDTH_BOARD {
            let mut vertical_cells: Vec<&Cell> = Vec::with_capacity(HEIGHT_BOARD);
            'y_y: for j in 0..HEIGHT_BOARD {
                //If the cell is empty -> break this loop iteration
                let current_cell = self.cells.get(&Board::get_index(i, j).unwrap()).unwrap();
                if let None = current_cell.piece {
                    break 'y_y;
                }
                vertical_cells.push(current_cell);
            }
            
            if Board::check_cell_is_winning(&mut vertical_cells) {
                // println!("Winnnn vertical_cells");
                return Some(vertical_cells);
            }
        }

        //Diagonale
        // let mut diagonal_cells: Vec<&Cell> = Vec::with_capacity(HEIGHT_BOARD);
        // vertical_cells.push(current_cell);

        //Vertical check
        None
    }

    pub fn check_cell_is_winning(cells: &mut Vec<&Cell>) -> bool {
        if !cells.into_iter().all(|f| f.piece.is_some()) {
            return false;
        }
        // println!("All cells are filled");

        let mut pieces: Vec<Piece> = cells.into_iter().map(|c| c.piece.unwrap()).collect();
        Piece::check_piece_is_winning(&mut pieces)
    }
}

// Give access to cells directly from Board
impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        if index > WIDTH_BOARD * HEIGHT_BOARD {
            panic!("Index out of bounds");
        }

        self.cells.get(&index).unwrap()
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > WIDTH_BOARD * HEIGHT_BOARD {
            panic!("Index out of bounds");
        }

        self.cells.get_mut(&index).unwrap()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pieces_feature = Color::to_vec_boxed()
            .into_iter()
            .map(|p| p)
            .chain(Hole::to_vec_boxed().into_iter().map(|p| p))
            .chain(Height::to_vec_boxed().into_iter().map(|p| p))
            .chain(Shape::to_vec_boxed().into_iter().map(|p| p));

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

        for (index, piece) in self.available_pieces.iter().enumerate() {
            current_row.add_cell(pCell::new_align(
                format!("{:0>2}\n{}", piece.0 + 1, piece.1.to_string().as_str()).as_str(),
                prettytable::format::Alignment::CENTER,
            ));
            if (index + 1) % 8 == 0 && index != self.available_pieces.len() - 1 {
                table_available_piece.add_row(current_row);
                current_row = pRow::empty();
            }
        }
        table_available_piece.add_row(current_row);

        //Draw Board
        let mut table_board = pTable::new();
        current_row = pRow::empty();
        for (i, cell) in self.cells.iter() {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cell {
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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CellColor {
    Black,
    White,
    Green,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_index_from_coordinate() {
        assert_eq!(Board::get_index(0, 0), Some(0));
        assert_eq!(Board::get_index(1, 0), Some(1));
        assert_eq!(Board::get_index(0, 3), Some(12));
        assert_eq!(Board::get_index(2, 2), Some(10));

        assert_eq!(Board::get_index(4, 4), None);
    }

    #[test]
    fn get_coordinate_from_index() {
        assert_eq!(Board::get_cell_coordinate(0), Some((0, 0)));
        assert_eq!(Board::get_cell_coordinate(1), Some((1, 0)));
        assert_eq!(Board::get_cell_coordinate(12), Some((0, 3)));
        assert_eq!(Board::get_cell_coordinate(10), Some((2, 2)));

        assert_eq!(Board::get_cell_coordinate(20), None);
    }

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
    /// When create a new game, we need to have 16 cells
    fn test_create_new_board_should_have_16_cells() {
        assert_eq!(Board::generate_all_cells().len(), 16);
        assert_eq!(Board::create().cells.len(), 16);
        assert_eq!(Board::create().get_cells().len(), 16);
    }

    #[test]
    /// When create a new game, all cells must be empty
    fn test_create_new_board_should_be_empty() {
        assert_eq!(
            Board::create().cells.iter().all(|p| p.1.piece.is_none()),
            true
        );
    }

    /// When create a new game, all piece should be available to be played
    #[test]
    fn test_create_new_board_all_piece_should_be_available() {
        assert_eq!(Board::create().available_pieces.len(), 16);
    }

    #[test]
    fn test_play_piece_cell_not_empty() {
        const INDEX_PIECE: usize = 0;
        const INDEX_CELL: usize = 0;
        let mut board = Board::create();

        //Play the first piece in first cell of the board
        board.play_piece(INDEX_PIECE, INDEX_CELL).unwrap();

        //Should haven't none in the first cell after play
        let cell = board.get_cells().get(&INDEX_CELL).unwrap();
        assert_ne!(cell.piece, None);

        assert_ne!(board[INDEX_CELL].piece, None);
    }

    #[test]
    fn test_play_piece_founded_after_played() {
        const INDEX_PIECE: usize = 0;
        const INDEX_CELL: usize = 0;
        let mut board = Board::create();

        //Play the first piece in first cell of the board
        let piece_played = board.play_piece(INDEX_PIECE, INDEX_CELL).unwrap();
        assert_eq!(board[INDEX_CELL].piece.unwrap(), piece_played);
    }

    #[test]
    fn test_remove_piece() {
        const INDEX_PIECE: usize = 0;
        let mut board = Board::create();

        //Piece is now removed
        board.remove_piece(INDEX_PIECE).unwrap();

        //Piece is not playable anymore
        assert_eq!(board.available_pieces.get(&INDEX_PIECE), None);

        //And if you try to access, you got a PieceDoesNotBelongPlayable error
        let error_expected = Err(ErrorGame::PieceDoesNotBelongPlayable);
        assert_eq!(board.get_piece_from_available(INDEX_PIECE), error_expected);
    }

    ///Index board accessor out of range
    #[test]
    #[should_panic]
    fn test_access_index_board_out_of_bounds_should_panic() {
        Board::create()[20];
    }

    #[test]
    fn test_is_board_winning_vertical() {
        let mut board = Board::create();

        //Play the first piece in first cell of the board
        board.play_piece(0, 3).unwrap();
        board.play_piece(4, 7).unwrap();
        board.play_piece(2, 11).unwrap();
        board.play_piece(3, 15).unwrap();

        let maybe_cell_winning = board.is_board_winning();
        assert_ne!(maybe_cell_winning, None);
    }

    #[test]
    fn test_is_board_winning_horizontal() {
        let mut board = Board::create();

        //Play the first piece in first cell of the board
        board.play_piece(0, 0).unwrap();
        board.play_piece(4, 1).unwrap();
        board.play_piece(2, 2).unwrap();
        board.play_piece(3, 3).unwrap();

        let maybe_cell_winning = board.is_board_winning();
        assert_ne!(maybe_cell_winning, None);
    }

    #[test]
    fn test_is_board_loosing() {
        let mut board = Board::create();

        //Play the first piece in first cell of the board
        board.play_piece(8, 0).unwrap();
        let maybe_cell_winning = board.is_board_winning();
        assert_eq!(maybe_cell_winning, None);

        let mut board = Board::create();

        //Play the first piece in first cell of the board
        board.play_piece(8, 0).unwrap();
        board.play_piece(4, 1).unwrap();
        board.play_piece(2, 2).unwrap();
        board.play_piece(3, 4).unwrap();

        let maybe_cell_winning = board.is_board_winning();
        assert_eq!(maybe_cell_winning, None);
    }
}
