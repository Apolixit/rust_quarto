use crate::{error::ErrorGame};
use enum_iterator::IntoEnumIterator;
use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Index, IndexMut},
};

//https://doc.rust-lang.org/reference/conditional-compilation.html
// let my_directory = if cfg!(windows) {
//     "windows-specific-directory"
// } else {
//     "unix-directory"
// };
// #[cfg(target_arch = "wasm32")]
// use prettytable::{Cell as pCell, Row as pRow, Table as pTable};

use crate::piece::{Color, Height, Hole, Piece, PieceFeature, Shape};
use crate::r#move::Move;

pub const WIDTH_BOARD: usize = 4;
pub const HEIGHT_BOARD: usize = 4;

pub trait BoardIndex {
    /// Create the trait from specified index
    fn from_index(board: &Board, index: usize) -> Result<Self, ErrorGame>
    where
        Self: Sized;

    /// Return the index from the struct
    fn to_index(&self, board: &Board) -> Result<usize, ErrorGame>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Represent the state of the game
pub enum BoardState {
    /// No current winner and game currently in progress
    GameInProgress,

    /// We found a win combinaison
    Win(BTreeMap<usize, Cell>),

    /// No piece left, it's a draw
    Draw,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Board {
    /// The x16 cells of the board
    cells: BTreeMap<usize, Cell>,

    /// Pieces which has not been played yet
    available_pieces: BTreeMap<usize, Piece>,
}

impl Board {
    /// Generate all pieces of the game
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

        assert_eq!(pieces.len(), WIDTH_BOARD * HEIGHT_BOARD);
        trace!("All pieces have been generated");
        pieces
    }

    /// Create the board with (WIDTH_BOARD * HEIGHT_BOARD) empty cells
    fn generate_all_cells() -> BTreeMap<usize, Cell> {
        let mut cells = BTreeMap::new();

        for i in 0..WIDTH_BOARD * HEIGHT_BOARD {
            cells.insert(i, Cell::new(i).unwrap());
        }

        assert_eq!(cells.len(), WIDTH_BOARD * HEIGHT_BOARD);
        trace!("All cells have been generated");
        cells
    }

    /// Return index from (x; y) coordinate
    pub fn coordinate_to_index(x: usize, y: usize) -> Result<usize, ErrorGame> {
        if x >= WIDTH_BOARD || y >= HEIGHT_BOARD {
            return Err(ErrorGame::IndexOutOfBound);
        }
        Ok(y * HEIGHT_BOARD + x)
    }

    /// Return (x; y) coordinate from index
    pub fn index_to_coordinate(index: usize) -> Result<(usize, usize), ErrorGame> {
        if index >= WIDTH_BOARD * HEIGHT_BOARD {
            return Err(ErrorGame::IndexOutOfBound);
        }
        Ok((index % WIDTH_BOARD, index / HEIGHT_BOARD))
    }

    /// Return the (top left to bottom right) (top right to bottom left) list of cells
    pub fn get_diagonal_cells(board: &Board) -> (Vec<Cell>, Vec<Cell>) {
        let mut diagonal_cells_top_left_bottom_right: Vec<Cell> = vec![];
        let mut diagonal_cells_top_right_bottom_left: Vec<Cell> = vec![];
        for i in 0..WIDTH_BOARD {
            diagonal_cells_top_left_bottom_right.push(Cell::from_coordinate(board, i, i).unwrap());
            diagonal_cells_top_right_bottom_left
                .push(Cell::from_coordinate(board, WIDTH_BOARD - i - 1, i).unwrap());
        }

        (
            diagonal_cells_top_left_bottom_right,
            diagonal_cells_top_right_bottom_left,
        )
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

    #[cfg(test)]
    pub fn with_scenario(&mut self, scenario: Vec<Move>) {
        scenario.into_iter().for_each(|m| {
            self
            .play_and_remove_piece(&m)
            .unwrap();
        });
    }

    /// Can we play an other turn ?
    pub fn can_play_another_turn(&self) -> bool {
        self.get_available_pieces().len() > 0
            || self.get_cells().into_iter().any(|c| c.1.piece.is_none())
    }

    /// Play a piece on the board
    /// Piece and cell are identify by their index in the HashMap
    pub fn play(&mut self, piece: Piece, cell: Cell) -> Result<Piece, ErrorGame> {
        if !self.can_play_another_turn() {
            return Err(ErrorGame::PieceDoesNotBelongPlayable);
        }

        let cell_before_playing = cell;

        if let Some(piece) = cell.piece {
            return Err(ErrorGame::CellIsNotEmpty(cell, piece));
        }

        let cell_index = cell.to_index();
        self.cells
            .entry(cell_index)
            .and_modify(|f| f.piece = Some(piece));

        trace!(
            "Cell (i = {}) before playing = {} / after playing : {}",
            cell_index,
            cell_before_playing,
            Cell::from_index(&self, cell_index).unwrap()
        );

        Ok(piece)
    }

    /// Remove the piece from available playable list
    pub fn remove(&mut self, piece: Piece) -> Result<Piece, ErrorGame> {
        let index = piece.to_index(&self)?;

        trace!("Piece {} num {} remove from availables", piece, index);
        self.available_pieces
            .remove(&index)
            .ok_or(ErrorGame::PieceDoesNotExists)
    }

    /// The combination of the two previous functions
    pub fn play_and_remove_piece(&mut self, m: &Move) -> Result<Piece, ErrorGame> {
        self.play(m.piece(), m.cell())?;
        Ok(self.remove(m.piece())?)
    }

    /// Get the list of available piece that can be played
    pub fn get_available_pieces(&self) -> BTreeMap<usize, Piece> {
        self.available_pieces.clone()
    }

    /// Get the piece from the available stack
    pub fn get_piece_from_available(&self, index: usize) -> Result<&Piece, ErrorGame> {
        self.available_pieces
            .get(&index)
            .ok_or(ErrorGame::PieceDoesNotBelongPlayable)
    }

    pub fn get_cells(&self) -> &BTreeMap<usize, Cell> {
        &self.cells
    }

    /// Return the empty cells available in the board
    pub fn get_empty_cells(&self) -> BTreeMap<usize, Cell> {
        self.get_cells()
            .clone()
            .into_iter()
            .filter(|f| f.1.piece.is_none())
            .collect()
    }

    /// Scan the board and check if a position is winning.
    /// Return None if no winning position has been found
    /// Return Some() with the list of winning cells
    pub fn board_state(&self) -> BoardState {
        //Horizontal check
        for i in 0..WIDTH_BOARD {
            let mut horizontal_cells: Vec<Cell> = Vec::with_capacity(HEIGHT_BOARD);
            'y_x: for j in 0..HEIGHT_BOARD {
                //If the cell is empty -> break this loop iteration
                let current_cell = Cell::from_coordinate(&self, j, i).unwrap();
                if let None = current_cell.piece {
                    break 'y_x;
                }
                horizontal_cells.push(current_cell);
            }

            if Board::check_cell_is_winning(&mut horizontal_cells) {
                info!("Horizontal win with cells {:?}", horizontal_cells);
                return BoardState::Win(self.to_btree(horizontal_cells));
            }
        }

        for i in 0..WIDTH_BOARD {
            let mut vertical_cells: Vec<Cell> = Vec::with_capacity(HEIGHT_BOARD);
            'y_y: for j in 0..HEIGHT_BOARD {
                //If the cell is empty -> break this loop iteration
                let current_cell = Cell::from_coordinate(&self, i, j).unwrap();
                if let None = current_cell.piece {
                    break 'y_y;
                }
                vertical_cells.push(current_cell);
            }

            if Board::check_cell_is_winning(&mut vertical_cells) {
                info!("Vertical win with cells {:?}", vertical_cells);
                return BoardState::Win(self.to_btree(vertical_cells));
            }
        }

        //Diagonale
        let (mut diagonal_cells_top_left_bottom_right, mut diagonal_cells_top_right_bottom_left) =
            Board::get_diagonal_cells(&self);

        if Board::check_cell_is_winning(&mut diagonal_cells_top_left_bottom_right) {
            info!(
                "Diagonal win with cells {:?}",
                diagonal_cells_top_left_bottom_right
            );
            return BoardState::Win(self.to_btree(diagonal_cells_top_left_bottom_right));
        }

        if Board::check_cell_is_winning(&mut diagonal_cells_top_right_bottom_left) {
            info!(
                "Diagonal win with cells {:?}",
                diagonal_cells_top_right_bottom_left
            );
            return BoardState::Win(self.to_btree(diagonal_cells_top_right_bottom_left));
        }

        // No win condition, the game continue. If we don't have any other piece to play, it's a draw
        if !self.can_play_another_turn() {
            return BoardState::Draw;
        }
        BoardState::GameInProgress
    }

    /// Do the reverse mapping by filtering  the original BTree from the Vec in parameter
    pub fn to_btree(&self, v: Vec<Cell>) -> BTreeMap<usize, Cell> {
        self.cells
            .clone()
            .into_iter()
            .filter(|&(_, c)| v.contains(&c))
            .collect()
    }

    /// Return if the current cells are a winning position
    pub fn check_cell_is_winning(cells: &mut Vec<Cell>) -> bool {
        if !cells.into_iter().all(|f| f.piece.is_some()) {
            trace!("check_cell_is_winning : some cells are empty");
            return false;
        }

        let mut pieces: Vec<Piece> = cells.into_iter().map(|c| c.piece.unwrap()).collect();
        let is_win = Piece::check_piece_is_winning(&mut pieces);

        is_win
    }

    /// Return the list of the immediate available move from the current board
    pub fn get_available_moves(&self) -> Vec<Move> {
        let mut available_next_move: Vec<Move> = vec![];

        for (_, piece) in &self.get_available_pieces() {
            for (_, cell) in &self.get_empty_cells() {
                available_next_move.push(Move::new(*piece, *cell));
            }
        }

        available_next_move
    }

    /// Return the list of the immediate available move from the current board
    pub fn get_available_moves_from_piece(&self, piece: Piece) -> Vec<Move> {
        self.get_empty_cells()
            .into_iter()
            .map(|(index_cell, _)| Move::new(piece, Cell::from_index(&self, index_cell).unwrap()))
            .collect::<Vec<Move>>()
    }

    // #[cfg(target_arch = "wasm32")]
    #[cfg(not(feature = "display_console"))]
    fn display_board(&self) -> String {
        String::from("")
    }

    // #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "display_console")]
    fn display_board(&self) -> String {
        use prettytable::{Cell as pCell, Row as pRow, Table as pTable};
        use ansi_term::Style;

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
                e.color()[i % 2].paint(e.acronym()),
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
            let draw_cell = pCell::new_align(
                format!("{:0>2}\n{}", i + 1, cell.to_string().as_str()).as_str(),
                prettytable::format::Alignment::CENTER,
            );

            current_row.add_cell(draw_cell);
            if (i + 1) % WIDTH_BOARD == 0 {
                table_board.add_row(current_row);
                current_row = pRow::empty();
            }
        }
        legend = format!("{}\n{}\n{}", legend, table_available_piece, table_board);
        legend
    }
}

/// Give access to cells directly from Board (board[0], board[10])
impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index <= WIDTH_BOARD * HEIGHT_BOARD, "Index out of bounds");
        self.cells.get(&index).unwrap()
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index <= WIDTH_BOARD * HEIGHT_BOARD, "Index out of bounds");
        self.cells.get_mut(&index).unwrap()
    }
}

/// Draw the board
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let legend = self.display_board();
        return write!(f, "{}", legend);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cell {
    /// Determine if a piece is present on the cell or not
    piece: Option<Piece>,
    /// The horizontal index
    x: usize,
    /// The vertical index
    y: usize,
}

impl Cell {
    /// Create a new Cell
    pub fn new(index: usize) -> Result<Cell, ErrorGame> {
        let (x, y) = Board::index_to_coordinate(index)?;
        Ok(Cell { piece: None, x, y })
    }

    pub fn to_index(&self) -> usize {
        self.x + self.y * HEIGHT_BOARD
    }

    pub fn to_coordinate(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn from_index(board: &Board, index: usize) -> Result<Self, ErrorGame> {
        Ok(*board
            .get_cells()
            .get(&index)
            .ok_or(ErrorGame::IndexOutOfBound)?)
    }

    pub fn from_coordinate(board: &Board, x: usize, y: usize) -> Result<Self, ErrorGame> {
        Ok(*board
            .get_cells()
            .get(&Board::coordinate_to_index(x, y)?)
            .unwrap())
    }

    pub fn piece(&self) -> Option<Piece> {
        self.piece
    }
}

/// Draw a cell
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_and_coordinate() {
        assert_eq!(Board::coordinate_to_index(0, 0).unwrap(), 0);
        assert_eq!(Board::coordinate_to_index(1, 0).unwrap(), 1);
        assert_eq!(Board::coordinate_to_index(0, 3).unwrap(), 12);
        assert_eq!(Board::coordinate_to_index(2, 2).unwrap(), 10);

        assert_eq!(Board::coordinate_to_index(3, 0).unwrap(), 3);
        assert_eq!(Board::coordinate_to_index(2, 1).unwrap(), 6);
        assert_eq!(Board::coordinate_to_index(1, 2).unwrap(), 9);
        assert_eq!(Board::coordinate_to_index(0, 3).unwrap(), 12);

        assert_eq!(
            Board::coordinate_to_index(4, 4),
            Err(ErrorGame::IndexOutOfBound)
        );

        assert_eq!(Cell::new(0).unwrap().to_index(), 0);
        assert_eq!(Cell::new(10).unwrap().to_index(), 10);
        assert_eq!(Cell::new(15).unwrap().to_index(), 15);

        assert_eq!(Board::index_to_coordinate(0).unwrap(), (0, 0));
        assert_eq!(Board::index_to_coordinate(1).unwrap(), (1, 0));
        assert_eq!(Board::index_to_coordinate(12).unwrap(), (0, 3));
        assert_eq!(Board::index_to_coordinate(10).unwrap(), (2, 2));

        assert_eq!(
            Board::index_to_coordinate(20),
            Err(ErrorGame::IndexOutOfBound)
        );
    }

    #[test]
    fn test_cell_manipulation() {
        assert_eq!(
            Cell::new(0).unwrap(),
            Cell {
                piece: None,
                x: 0,
                y: 0
            }
        );
        assert_eq!(
            Cell::new(10).unwrap(),
            Cell {
                piece: None,
                x: 2,
                y: 2
            }
        );
        assert_eq!(
            Cell::new(15).unwrap(),
            Cell {
                piece: None,
                x: 3,
                y: 3
            }
        );

        assert_eq!(Cell::new(20), Err(ErrorGame::IndexOutOfBound));

        let cell = Cell::new(10).unwrap();
        assert_eq!(cell.to_index(), 10);
        assert_eq!(cell.to_coordinate(), (2, 2));

        let board = Board::create();
        let cell_2 = Cell::from_coordinate(&board, 0, 0).unwrap();
        assert_eq!(
            cell_2,
            Cell {
                piece: None,
                x: 0,
                y: 0
            }
        );
        assert_eq!(cell_2.to_index(), 0);

        assert_eq!(
            Cell::from_coordinate(&board, 20, 0),
            Err(ErrorGame::IndexOutOfBound)
        );
        assert_eq!(
            Cell::from_coordinate(&board, 0, 20),
            Err(ErrorGame::IndexOutOfBound)
        );
        assert_eq!(
            Cell::from_coordinate(&board, 20, 20),
            Err(ErrorGame::IndexOutOfBound)
        );

        assert_eq!(
            Cell::from_index(&board, 0).unwrap(),
            Cell {
                piece: None,
                x: 0,
                y: 0
            }
        );
        assert_eq!(Cell::from_index(&board, 0).unwrap().to_coordinate(), (0, 0));

        assert_eq!(
            Cell::from_index(&board, 20),
            Err(ErrorGame::IndexOutOfBound)
        );
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
    fn test_play_piece_cell_should_not_be_empty() {
        const INDEX_PIECE: usize = 0;
        const INDEX_CELL: usize = 0;
        let mut board = Board::create();

        //Play the first piece in first cell of the board
        board
            .play(
                Piece::from_index(&board, INDEX_PIECE).unwrap(),
                Cell::from_index(&board, INDEX_CELL).unwrap(),
            )
            .unwrap();

        //Should haven't none in the first cell after play
        let cell = board.get_cells().get(&INDEX_CELL).unwrap();
        assert_ne!(cell.piece, None);

        assert_ne!(board[INDEX_CELL].piece, None);
    }

    #[test]
    fn test_play_piece_founded_after_played_should_succeed() {
        const INDEX_PIECE: usize = 0;
        const INDEX_CELL: usize = 0;
        let mut board = Board::create();

        //Play the first piece in first cell of the board

        let piece_played = board
            .play(
                Piece::from_index(&board, INDEX_PIECE).unwrap(),
                Cell::from_index(&board, INDEX_CELL).unwrap(),
            )
            .unwrap();
        assert_eq!(board[INDEX_CELL].piece.unwrap(), piece_played);
    }

    #[test]
    fn test_remove_piece_should_succeed() {
        const INDEX_PIECE: usize = 0;
        let mut board = Board::create();

        //Piece is now removed
        board
            .remove(Piece::from_index(&board, INDEX_PIECE).unwrap())
            .unwrap();

        //Piece is not playable anymore
        assert_eq!(board.available_pieces.get(&INDEX_PIECE), None);

        //And if you try to access, you got a PieceDoesNotBelongPlayable error
        let error_expected = Err(ErrorGame::PieceDoesNotBelongPlayable);
        assert_eq!(Piece::from_index(&board, INDEX_PIECE), error_expected);
    }

    ///Index board accessor out of range
    #[test]
    #[should_panic]
    fn test_access_index_board_out_of_bounds_should_panic() {
        Board::create()[20];
    }

    #[test]
    fn test_find_piece_index_should_succeed() {
        let mut board = Board::create();

        let p1 = Piece::from("DEXC"); // index 8
        let p2 = Piece::from("DEXS"); // index 9
        let p3 = Piece::from("DETC"); // index 10
        let p4 = Piece::from("DETS"); // index 11

        assert_eq!(p1.to_index(&board).unwrap(), 8);
        assert_eq!(p1.to_index(&board).unwrap(), 8);
        assert_eq!(p2.to_index(&board).unwrap(), 9);
        assert_eq!(p3.to_index(&board).unwrap(), 10);
        assert_eq!(p4.to_index(&board).unwrap(), 11);
        assert_eq!(p4.to_index(&board).unwrap(), 11);
        assert_eq!(p4.to_index(&board).unwrap(), 11);

        //Now we remove each piece from available pool of piece
        assert_eq!(p1.to_index(&board).unwrap(), 8);
        board.remove(Piece::from_index(&board, 8).unwrap()).unwrap();
        assert_eq!(p2.to_index(&board).unwrap(), 9);
        board.remove(Piece::from_index(&board, 9).unwrap()).unwrap();
        assert_eq!(p3.to_index(&board).unwrap(), 10);
        board.remove(Piece::from_index(&board, 10).unwrap()).unwrap();
        assert_eq!(p4.to_index(&board).unwrap(), 11);
        board.remove(Piece::from_index(&board, 11).unwrap()).unwrap();
    }

    #[test]
    fn test_is_board_winning_vertical_should_win() {
        let mut board = Board::create();
        let plays: Vec<(usize, usize)> = vec![(0, 3), (4, 7), (2, 11), (3, 15)];

        //Play the first piece in first cell of the board
        for play in &plays {
            board
                .play(
                    Piece::from_index(&board, play.0).unwrap(),
                    Cell::from_index(&board, play.1).unwrap(),
                )
                .unwrap();
        }

        let mut btree_win: BTreeMap<usize, Cell> = BTreeMap::new();
        for play in &plays {
            btree_win.insert(play.1, *board.cells.get(&play.1).unwrap());
        }

        let maybe_cell_winning = board.board_state();
        assert_eq!(maybe_cell_winning, BoardState::Win(btree_win));
    }

    #[test]
    fn test_is_board_winning_horizontal() {
        let mut board = Board::create();
        let plays: Vec<(usize, usize)> = vec![(0, 0), (4, 1), (2, 2), (3, 3)];
        let mut btree_win: BTreeMap<usize, Cell> = BTreeMap::new();

        //Play the first piece in first cell of the board
        for play in &plays {
            board
                .play(
                    Piece::from_index(&board, play.0).unwrap(),
                    Cell::from_index(&board, play.1).unwrap(),
                )
                .unwrap();
        }

        for play in &plays {
            btree_win.insert(play.1, *board.cells.get(&play.1).unwrap());
        }

        let maybe_cell_winning = board.board_state();
        assert_eq!(maybe_cell_winning, BoardState::Win(btree_win));
    }

    #[test]
    fn test_is_board_winning_diagonal() {
        let mut board = Board::create();
        let plays: Vec<(Piece, usize)> = vec![
            (Piece::from("DFXS"), 3),
            (Piece::from("DETS"), 6),
            (Piece::from("DEXS"), 9),
            (Piece::from("DETC"), 12),
        ];
        let mut btree_win: BTreeMap<usize, Cell> = BTreeMap::new();

        //Play the first piece in first cell of the board
        for play in &plays {
            board
                .play(play.0, Cell::from_index(&board, play.1).unwrap())
                .unwrap();
        }

        for play in &plays {
            btree_win.insert(play.1, *board.cells.get(&play.1).unwrap());
        }

        debug!("{}", board);
        let maybe_cell_winning = board.board_state();
        assert_eq!(maybe_cell_winning, BoardState::Win(btree_win));
    }

    #[test]
    fn test_is_board_loosing() {
        let mut board = Board::create();

        //Play the first piece in first cell of the board
        board
            .play(
                Piece::from_index(&board, 8).unwrap(),
                Cell::from_index(&board, 0).unwrap(),
            )
            .unwrap();
        let maybe_cell_winning = board.board_state();
        assert_eq!(maybe_cell_winning, BoardState::GameInProgress);

        let mut board = Board::create();

        //Play the first piece in first cell of the board
        board
            .play(
                Piece::from_index(&board, 8).unwrap(),
                Cell::from_index(&board, 0).unwrap(),
            )
            .unwrap();
        board
            .play(
                Piece::from_index(&board, 4).unwrap(),
                Cell::from_index(&board, 1).unwrap(),
            )
            .unwrap();
        board
            .play(
                Piece::from_index(&board, 2).unwrap(),
                Cell::from_index(&board, 2).unwrap(),
            )
            .unwrap();
        board
            .play(
                Piece::from_index(&board, 3).unwrap(),
                Cell::from_index(&board, 4).unwrap(),
            )
            .unwrap();

        let maybe_cell_winning = board.board_state();
        assert_eq!(maybe_cell_winning, BoardState::GameInProgress);
    }

    #[test]
    fn test_board_draw() {
        let mut board = Board::create();

        // Cell Index, Piece
        let pieces: Vec<Piece> = vec![
            Piece::from("WETS"),
            Piece::from("DFTC"),
            Piece::from("DFTS"),
            Piece::from("DFXS"),
            Piece::from("WFTS"),
            Piece::from("WFXS"),
            Piece::from("DETS"),
            Piece::from("DFXC"),
            Piece::from("DEXS"),
            Piece::from("WEXC"),
            Piece::from("WFXC"),
            Piece::from("WETC"),
            Piece::from("DETC"),
            Piece::from("WFTC"),
            Piece::from("WEXS"),
            Piece::from("DEXC"),
        ];

        let cloned_board = board.clone();

        //Play the first piece in first cell of the board
        for (cell, piece) in (0..16)
            .map(|i| Cell::from_index(&cloned_board, i).unwrap())
            .zip(pieces)
        {
            // let piece_index = Piece::from board.get_piece_index(&play.1).unwrap();
            board.play(piece, cell).unwrap();
            board.remove(piece).unwrap();
        }

        error!("{}", board);

        let maybe_cell_winning = board.board_state();
        assert_eq!(maybe_cell_winning, BoardState::Draw);
    }

    #[test]
    fn test_display_board_not_empty() {
        if cfg!(feature = "display_console") {
            debug!("console_display ACTIVATED");
        } else {
            debug!("console_display DISABLED");
        }
        let board = Board::create();

        assert!(format!("{}", board).as_str() != "");
    }
}
