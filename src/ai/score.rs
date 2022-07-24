use core::cmp::Ordering;
use std::fmt::Display;
use std::iter::Sum;
use crate::board::Board;
use crate::board::Cell;
use crate::board::HEIGHT_BOARD;
use crate::board::WIDTH_BOARD;
use crate::piece::Color;
use crate::piece::Height;
use crate::piece::Hole;
use crate::piece::Piece;
use crate::piece::Shape;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Score {
    Point(usize),
    Win,
}

impl Score {
    /// Return the global board score or if the board is winning
    pub fn calc_score(board: &Board) -> Score {
        let mut h_score: Vec<Score> = vec![];
        let mut v_score: Vec<Score> = vec![];
        let mut d_score_1: Vec<Score> = vec![];
        let mut d_score_2: Vec<Score> = vec![];

        for i in 0..WIDTH_BOARD {
            let mut horizontal_cells: Vec<Cell> = vec![];
            let mut vertical_cells: Vec<Cell> = vec![];
            for j in 0..HEIGHT_BOARD {
                horizontal_cells.push(Cell::from_coordinate(&board, j, i).unwrap());
                vertical_cells.push(Cell::from_coordinate(&board, i, j).unwrap());
            }

            h_score.push(Score::calc_range_point(&horizontal_cells));
            v_score.push(Score::calc_range_point(&vertical_cells));

            if Score::has_win(&h_score) || Score::has_win(&v_score) {
                return Score::Win;
            }
        }

        let (diagonal_cells_top_left_bottom_right, diagonal_cells_top_right_bottom_left) = Board::get_diagonal_cells(&board);
        d_score_1.push(Score::calc_range_point(&diagonal_cells_top_left_bottom_right));
        d_score_2.push(Score::calc_range_point(&diagonal_cells_top_right_bottom_left));
        if Score::has_win(&d_score_1) || Score::has_win(&d_score_2) {
            return Score::Win;
        }

        Score::sum_scores(vec![h_score, v_score, d_score_1, d_score_2])
    }

    /// Add all horizontal / vertical and diagonal score to get a global board score
    fn sum_scores(scores: Vec<Vec<Score>>) -> Score {
        scores.into_iter().flatten().sum::<Score>()
    }

    /// Return the score for the current cells
    fn calc_range_point(cells: &Vec<Cell>) -> Score {
        // We only get pieces which has been already played
        let pieces: Vec<Piece> = cells
            .into_iter()
            .filter(|c| c.piece().is_some())
            .map(|c| c.piece().unwrap())
            .collect();

        // No piece has been played -> score = 0
        if pieces.is_empty() {
            return Score::Point(0);
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

        Score::calc_point(points)
    }

    /// Calc the score for the current pieces alignement
    fn calc_point(points: Vec<usize>) -> Score {
        points
            .into_iter()
            .map(|p| {
                /*
                 * 0 or 1 piece = 0 point
                 * 2 pieces = 1 points
                 * 3 pieces = 2 points
                 * 4 pieces = Win
                 */
                match p {
                    2 => Score::Point(1),
                    3 => Score::Point(2),
                    4 => Score::Win,
                    _ => Score::Point(0),
                }
            })
            .sum::<Score>()
    }


    fn has_win(score: &Vec<Score>) -> bool {
        if score.last().unwrap() == &Score::Win {
            return true;
        }
        false
    }
}

impl Default for Score {
    fn default() -> Self {
        Score::Point(0)
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s_val = match self {
            Self::Point(val) => {
                format!("{} points", val)
            },
            Self::Win => {
                "Winning board !".to_string()
            },
        };
        write!(f, "{}", s_val)
    }
}

impl PartialOrd for Score {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
     }
}

// Implement comparison trait to allow score compare
impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Score::Point(self_val) => {
                match other {
                    Score::Point(other_val) => {
                        return self_val.cmp(other_val);
                    },
                    Score::Win => {
                        return Ordering::Less;
                    }
                }
            },
            Score::Win => {
                match other {
                    Score::Point(_) => {
                        return Ordering::Greater;
                    },
                    Score::Win => {
                        return Ordering::Equal;
                    }
                }
            }
        }
    }
}

impl Sum for Score {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::Point(0), |x, y| {
            if x == Score::Win || y == Score::Win {
                return Score::Win;
            }
            if let (Score::Point(x_val), Score::Point(y_val)) = (x, y) {
                return Score::Point(x_val + y_val);
            }

            Score::Point(0)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::{Board, Cell}, piece::Piece};

    use super::Score;

    // Run this function before each test
    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    fn test_scenario(moves: Vec<(Piece, Score, usize)>) {
        let mut board = Board::create();

        for (piece_current, score_current, cell_index) in moves {
            let cell = Cell::from_index(&board, cell_index).unwrap();

            board.play(piece_current, cell).unwrap();
            board.remove(piece_current).unwrap();

            let score = Score::calc_score(&board);
            info!("Add piece = {} / cell = {:?} / Score = {:?}", piece_current, cell, score);
            assert_eq!(score, score_current);
        }
    }
    #[test]
    pub fn test_all_direction_should_have_same_score() {
        let pieces_horizontal_second_line = vec![
            (Piece::from("DFTC"), Score::Point(0), Board::coordinate_to_index(0, 1).unwrap()),
            (Piece::from("DFXS"), Score::Point(2), Board::coordinate_to_index(1, 1).unwrap()),
            (Piece::from("WETS"), Score::Point(4), Board::coordinate_to_index(2, 1).unwrap()),
            (Piece::from("WEXS"), Score::Point(8), Board::coordinate_to_index(3, 1).unwrap()),
        ];
        let pieces_vertical_third_line = vec![
            (Piece::from("DFTC"), Score::Point(0), Board::coordinate_to_index(2, 0).unwrap()),
            (Piece::from("DFXS"), Score::Point(2), Board::coordinate_to_index(2, 1).unwrap()),
            (Piece::from("WETS"), Score::Point(4), Board::coordinate_to_index(2, 2).unwrap()),
            (Piece::from("WEXS"), Score::Point(8), Board::coordinate_to_index(2, 3).unwrap()),
        ];
        let pieces_diagonal_top_left_to_bottom_right = vec![
            (Piece::from("DFTC"), Score::Point(0), Board::coordinate_to_index(0, 0).unwrap()),
            (Piece::from("DFXS"), Score::Point(2), Board::coordinate_to_index(1, 1).unwrap()),
            (Piece::from("WETS"), Score::Point(4), Board::coordinate_to_index(2, 2).unwrap()),
            (Piece::from("WEXS"), Score::Point(8), Board::coordinate_to_index(3, 3).unwrap()),
        ];
        let pieces_diagonal_top_right_to_bottom_left = vec![
            (Piece::from("DFTC"), Score::Point(0), Board::coordinate_to_index(3, 0).unwrap()),
            (Piece::from("DFXS"), Score::Point(2), Board::coordinate_to_index(2, 1).unwrap()),
            (Piece::from("WETS"), Score::Point(4), Board::coordinate_to_index(1, 2).unwrap()),
            (Piece::from("WEXS"), Score::Point(8), Board::coordinate_to_index(0, 3).unwrap()),
        ];

        for scenario in vec![
            pieces_horizontal_second_line,
            pieces_vertical_third_line,
            pieces_diagonal_top_left_to_bottom_right,
            pieces_diagonal_top_right_to_bottom_left,
        ] {
            test_scenario(scenario);
        }
    }

    #[test]
    pub fn test_calc_basic_score_no_point() {
        test_scenario(vec![
            (Piece::from("DFTC"), Score::Point(0), 0),
            (Piece::from("DFXS"), Score::Point(0), 6),
            (Piece::from("WEXS"), Score::Point(0), 15),
        ]);
    }

    #[test]
    pub fn test_calc_basic_score() {
        test_scenario(vec![
            (Piece::from("WETS"), Score::Point(0), 0),
            (Piece::from("DFTC"), Score::Point(1), 1),
            (Piece::from("DFTS"), Score::Point(5), 2),
            (Piece::from("WFTS"), Score::Point(8), 4),
            (Piece::from("DFXS"), Score::Point(13), 6),
            (Piece::from("DETC"), Score::Point(17), 10),
        ]);
    }

    #[test]
    pub fn test_calc_basic_score_2() {
        test_scenario(vec![
            (Piece::from("WETS"), Score::Point(0), 0),
            (Piece::from("DFTS"), Score::Point(2), 2),
            (Piece::from("WFTS"), Score::Point(5), 4),
            (Piece::from("DFXS"), Score::Point(10), 6),
            (Piece::from("DETC"), Score::Point(11), 9),
            (Piece::from("WFXS"), Score::Point(13), 15),
            (Piece::from("DEXS"), Score::Point(18), 14),
        ]);
        // (piece num 10 / cell num 15)
    }

    #[test]
    pub fn test_calc_basic_score_line_full_loosing() {
        test_scenario(vec![
            (Piece::from("WETS"), Score::Point(0), 0),
            (Piece::from("DFTC"), Score::Point(1), 1),
            (Piece::from("DFTS"), Score::Point(5), 2),
            (Piece::from("DFXS"), Score::Point(8), 3),
            (Piece::from("WFTS"), Score::Point(11), 4),
        ]);
    }

    #[test]
    pub fn test_calc_winning_score() {
        test_scenario(vec![
            (Piece::from("DFTC"), Score::Point(0), Board::coordinate_to_index(0, 0).unwrap()),
            (Piece::from("WETS"), Score::Point(1), Board::coordinate_to_index(1, 1).unwrap()),
            (Piece::from("WEXS"), Score::Point(4), Board::coordinate_to_index(1, 2).unwrap()),
            (Piece::from("WFTS"), Score::Point(9), Board::coordinate_to_index(2, 1).unwrap()),
            (Piece::from("DEXS"), Score::Point(12), Board::coordinate_to_index(1, 3).unwrap()),
            (Piece::from("DFTS"), Score::Win, Board::coordinate_to_index(1, 0).unwrap()),
        ]);
    }

    #[test]
    fn test_score_compare() {
        assert!(Score::Point(10) < Score::Point(20));
        assert!(Score::Point(1) > Score::Point(0));
        assert!(Score::Point(0) >= Score::Point(0));
        assert!(Score::Point(50) == Score::Point(50));

        assert!(Score::Point(50) < Score::Win);
        assert!(Score::Win == Score::Win);
    }

    #[test]
    fn bench_score() {
        let mut board = Board::create();
        board.with_scenario(vec![
            crate::r#move::Move::new(Piece::from("DETS"), Cell::from_index(&board, 5).unwrap()),
            crate::r#move::Move::new(Piece::from("DFTS"), Cell::from_index(&board, 6).unwrap()),
            crate::r#move::Move::new(Piece::from("WFXC"), Cell::from_index(&board, 9).unwrap()),
            crate::r#move::Move::new(Piece::from("WETC"), Cell::from_index(&board, 10).unwrap()),
            crate::r#move::Move::new(Piece::from("DFTC"), Cell::from_index(&board, 12).unwrap()),
            crate::r#move::Move::new(Piece::from("WFTC"), Cell::from_index(&board, 13).unwrap()),
            crate::r#move::Move::new(Piece::from("WFXS"), Cell::from_index(&board, 14).unwrap()),
            crate::r#move::Move::new(Piece::from("DEXC"), Cell::from_index(&board, 15).unwrap()),
        ]);

        let now = std::time::Instant::now();

        Score::calc_score(&board);

        info!("Score: {} nanosec", now.elapsed().as_nanos());
    }
}
