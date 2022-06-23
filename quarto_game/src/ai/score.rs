use crate::board::Board;
use crate::board::Cell;
use crate::board::HEIGHT_BOARD;
use crate::board::WIDTH_BOARD;
use crate::piece::Color;
use crate::piece::Height;
use crate::piece::Hole;
use crate::piece::Piece;
use crate::piece::Shape;

#[derive(Debug, PartialEq)]
pub enum Score {
    Point(usize),
    Win,
}

impl Score {
    pub fn calc_score(board: &Board) -> Score {
        let mut h_score: Vec<usize> = vec![];
        let mut v_score: Vec<usize> = vec![];

        for i in 0..WIDTH_BOARD {
            let mut horizontal_cells: Vec<Cell> = vec![];
            for j in 0..HEIGHT_BOARD {
                horizontal_cells.push(board.get_cells_from_position(j, i));
            }
            // println!("{:?}", horizontal_cells);
            h_score.push(Score::calc_range_point(&horizontal_cells));
            // println!("Score == {}", h_score);
            if h_score.last().unwrap() >= &1000 {
                return Score::Win;
            }
        }
        for i in 0..WIDTH_BOARD {
            let mut vertical_cells: Vec<Cell> = vec![];
            for j in 0..HEIGHT_BOARD {
                vertical_cells.push(board.get_cells_from_position(i, j));
            }
            v_score.push(Score::calc_range_point(&vertical_cells));
            if v_score.last().unwrap() >= &1000 {
                return Score::Win;
            }
        }

        Score::Point(h_score.iter().sum::<usize>() + v_score.iter().sum::<usize>())
    }

    fn calc_point(points: Vec<usize>) -> usize {
        points
            .into_iter()
            .map(|p| {
                /*
                 * 0 or 1 piece = 0 point
                 * 2 pieces = 3 points
                 * 3 pieces = 6 points
                 * 4 pieces = 1000 points
                 */
                match p {
                    2 => 1 as usize,
                    3 => 2 as usize,
                    4 => 1000 as usize,
                    _ => 0 as usize,
                }
            })
            .sum::<usize>()
    }

    fn calc_range_point(cells: &Vec<Cell>) -> usize {
        // We only get pieces which has been already played
        let mut pieces: Vec<Piece> = cells
            .into_iter()
            .filter(|c| c.piece.is_some())
            .map(|c| c.piece.unwrap())
            .collect();

        // No piece has been played -> score = 0
        if pieces.is_empty() {
            return 0 as usize;
        }

        // All the piece has been played and the line is not winning -> no score for this
        if pieces.len() == 4 && !Piece::check_piece_is_winning(&mut pieces) {
            return 0 as usize;
        }

        let b_pieces = &pieces;

        let points = vec![
            //Color
            b_pieces
                .into_iter()
                .filter(|f| f.color == Color::Dark)
                .count(),
            b_pieces
                .into_iter()
                .filter(|f| f.color == Color::White)
                .count(),
            //Height
            b_pieces
                .into_iter()
                .filter(|f| f.height == Height::Tall)
                .count(),
            b_pieces
                .into_iter()
                .filter(|f| f.height == Height::Small)
                .count(),
            //Hole
            b_pieces
                .into_iter()
                .filter(|f| f.hole == Hole::Full)
                .count(),
            b_pieces
                .into_iter()
                .filter(|f| f.hole == Hole::Empty)
                .count(),
            //Shape
            b_pieces
                .into_iter()
                .filter(|f| f.shape == Shape::Circle)
                .count(),
            b_pieces
                .into_iter()
                .filter(|f| f.shape == Shape::Square)
                .count(),
        ];
        println!("Points = {:?}", points);
        Score::calc_point(points)
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, piece::Piece};

    use super::Score;

    fn generate_pieces() -> Vec<(Piece, Score)> {
        vec![
            (Piece::from("WETS"), Score::Point(0)),
            (Piece::from("DFTC"), Score::Point(1)),
            (Piece::from("DFTS"), Score::Point(5)),
            (Piece::from("DFXS"), Score::Point(0)), // Should be 0 because this move sucks
            (Piece::from("WFTS"), Score::Point(3)),
            // Piece::from("WFXS"),
            // Piece::from("DETS"),
            // Piece::from("DFXC"),
            // Piece::from("WFXC"),
            // Piece::from("DEXS"),
            // Piece::from("WEXC"),
            // Piece::from("WETC"),
            // Piece::from("DETC"),
            // Piece::from("WFTC"),
            // Piece::from("WEXS"),
            // Piece::from("DEXC"),
        ]
    }
    #[test]
    pub fn test_calc_basic_score() {
        // Start a new game and play a piece
        let mut board = Board::create();
        let pieces = vec![
            (Piece::from("WETS"), Score::Point(0), 0),
            (Piece::from("DFTC"), Score::Point(1), 1),
            (Piece::from("DFTS"), Score::Point(5), 2),
            (Piece::from("DFXS"), Score::Point(0), 3), // Should be 0 because this move sucks
            (Piece::from("WFTS"), Score::Point(3), 4),
        ];

        for (piece_current, score_current, index_board) in pieces {
            let piece_index = board.get_piece_index(&piece_current).unwrap();
            board.play_piece(piece_index, index_board).unwrap();
            board.remove_piece(piece_index).unwrap();
            assert_eq!(Score::calc_score(&board), score_current);
            // println!("{}", board);
        }
    }
}
