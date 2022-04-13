use super::piece::Piece;
use array2d::Array2D;

const WIDTH_BOARD: usize = 4;
const HEIGHT_BOARD: usize = 4;

pub struct Board {
    cells: Array2D<Cell>,
    pieces: Vec<Piece>
}

impl Board {

    // pub fn init() -> Self {
    //     Self {
    //         cells : build_board()
    //     }
    // }

    fn new_board(&self) -> Board {
        let mut grid_raw: Vec<Vec<Cell>> = vec![vec![Cell { piece: None, background_color: CellColor::Black }; WIDTH_BOARD]; HEIGHT_BOARD];

        for i in 0..WIDTH_BOARD {
            for j in 0..HEIGHT_BOARD {
                grid_raw[i][j].background_color = if j % 2 == 0 { CellColor::Black } else { CellColor::White }
            }
        }
        Board {
            cells: Array2D::from_columns(&grid_raw)
        }
    }

    fn all_pieces() -> Vec<Piece>{

    }

    pub fn new_game(&self) {
        let mut cells_array = vec![vec![0; 4]; 4];
    }


}

#[derive(Clone)]
pub struct Cell {
    piece : Option<Piece>,
    background_color: CellColor
}

#[derive(Clone)]
pub enum CellColor {
    Black,
    White
}