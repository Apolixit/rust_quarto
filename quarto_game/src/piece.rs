use ansi_term::Colour as ConsoleColor;
use enum_iterator::IntoEnumIterator;
use std::fmt::Display;

pub trait PieceFeature {
    fn acronym(&self) -> &str;
    fn name(&self) -> &str;
    fn color(&self) -> ConsoleColor;
    fn to_vec_boxed() -> Vec<Box<dyn PieceFeature>>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoEnumIterator)]
pub enum Color {
    White,
    Dark,
}

impl PieceFeature for Color {
    fn acronym(&self) -> &str {
        match self {
            Self::White => "W",
            Self::Dark => "D",
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::White => "White",
            Self::Dark => "Dark",
        }
    }

    fn color(&self) -> ConsoleColor {
        ConsoleColor::Blue
    }

    fn to_vec_boxed() -> Vec<Box<dyn PieceFeature>> {
        vec![Box::new(Color::White), Box::new(Color::Dark)]
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color().paint(self.acronym()))
    }
}

impl From<&str> for Color {
    fn from(c: &str) -> Self {
        match c {
            "W" => Self::White,
            "D" => Self::Dark,
            e => panic!("Input {} cannot be convert to color", e),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoEnumIterator)]
pub enum Hole {
    Empty,
    Full,
}
impl PieceFeature for Hole {
    fn acronym(&self) -> &str {
        match self {
            Self::Empty => "E",
            Self::Full => "F",
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Empty => "Empty",
            Self::Full => "Full",
        }
    }

    fn color(&self) -> ConsoleColor {
        ConsoleColor::Purple
    }

    fn to_vec_boxed() -> Vec<Box<dyn PieceFeature>> {
        vec![Box::new(Hole::Empty), Box::new(Hole::Full)]
    }
}

impl Display for Hole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color().paint(self.acronym()))
    }
}

impl From<&str> for Hole {
    fn from(c: &str) -> Self {
        match c {
            "E" => Self::Empty,
            "F" => Self::Full,
            e => panic!("Input {} cannot be convert to color", e),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoEnumIterator)]
pub enum Height {
    Small,
    Tall,
}

impl PieceFeature for Height {
    fn acronym(&self) -> &str {
        match self {
            Self::Small => "X",
            Self::Tall => "T",
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Small => "Small",
            Self::Tall => "Tall",
        }
    }

    fn color(&self) -> ConsoleColor {
        ConsoleColor::Red
    }

    fn to_vec_boxed() -> Vec<Box<dyn PieceFeature>> {
        vec![Box::new(Height::Small), Box::new(Height::Tall)]
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color().paint(self.acronym()))
    }
}

impl From<&str> for Height {
    fn from(c: &str) -> Self {
        match c {
            "X" => Self::Small,
            "T" => Self::Tall,
            e => panic!("Input {} cannot be convert to color", e),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoEnumIterator)]
pub enum Shape {
    Circle,
    Square,
}

impl PieceFeature for Shape {
    fn acronym(&self) -> &str {
        match self {
            Self::Circle => "C",
            Self::Square => "S",
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Circle => "Circle",
            Self::Square => "Square",
        }
    }

    fn color(&self) -> ConsoleColor {
        ConsoleColor::Green
    }

    fn to_vec_boxed() -> Vec<Box<dyn PieceFeature>> {
        vec![Box::new(Shape::Circle), Box::new(Shape::Square)]
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color().paint(self.acronym()))
    }
}

impl From<&str> for Shape {
    fn from(c: &str) -> Self {
        match c {
            "C" => Self::Circle,
            "S" => Self::Square,
            e => panic!("Input {} cannot be convert to color", e),
        }
    }
}

//Represent piece settings
#[derive(Debug, Clone, Copy, Eq)]
pub struct Piece {
    pub color: Color,
    pub hole: Hole,     //○, □
    pub height: Height, //⇑, ⇓
    pub shape: Shape,   //○, □
}

impl Piece {
    pub fn new(color: Color, hole: Hole, height: Height, shape: Shape) -> Self {
        Self {
            color,
            hole,
            height,
            shape,
        }
    }

    pub fn check_piece_is_winning(pieces: &mut Vec<Piece>) -> bool {
        //We need at least a 4 size vector
        if pieces.len() < 4 { return false; }

        let winning_condition = vec![
            pieces.into_iter().all(|p| p.color == Color::Dark),
            pieces.into_iter().all(|p| p.color == Color::White),
            pieces.into_iter().all(|p| p.height == Height::Small),
            pieces.into_iter().all(|p| p.height == Height::Tall),
            pieces.into_iter().all(|p| p.hole == Hole::Empty),
            pieces.into_iter().all(|p| p.hole == Hole::Empty),
            pieces.into_iter().all(|p| p.shape == Shape::Circle),
            pieces.into_iter().all(|p| p.shape == Shape::Square),
        ];
        print!("Vec<Piece> : {:?}", winning_condition);

        winning_condition.iter().any(|w| *w)
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.hole == other.hole
            && self.height == other.height
            && self.shape == other.shape
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.color, self.hole, self.height, self.shape
        )
    }
}

impl From<&str> for Piece {
    fn from(s: &str) -> Self {
        //4 character max
        if s.chars().count() != 4 {
            panic!("Out of bound");
        }

        let lower_s = s.to_uppercase();
        Piece::new(
            Color::from(&lower_s[0..=0]),
            Hole::from(&lower_s[1..=1]),
            Height::from(&lower_s[2..=2]),
            Shape::from(&lower_s[3..=3]),
        )
    }
}

impl From<[char; 4]> for Piece {
    fn from(c: [char; 4]) -> Self {
        Piece::from(String::from_iter(c).as_str())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality_basic() {
        let piece_base = Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle);
        let piece_base_equal = Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle);

        let piece_white = Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Circle);
        let piece_full = Piece::new(Color::Dark, Hole::Full, Height::Small, Shape::Circle);
        let piece_tall = Piece::new(Color::Dark, Hole::Full, Height::Tall, Shape::Circle);
        let piece_square = Piece::new(Color::Dark, Hole::Full, Height::Small, Shape::Square);

        let piece_base_clone = piece_base.clone();
        let piece_white_clone = piece_white.clone();

        assert_eq!(piece_base, piece_base_equal);
        assert_eq!(piece_base, piece_base_clone);
        assert_eq!(piece_base_equal, piece_base_clone);

        assert_ne!(piece_base, piece_white);
        assert_ne!(piece_base, piece_white_clone);
        assert_ne!(piece_base, piece_full);
        assert_ne!(piece_base, piece_tall);
        assert_ne!(piece_base, piece_square);
    }

    #[test]
    fn from_into_string_slice() {
        assert_eq!(
            Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle),
            Piece::from("DEXC")
        );

        assert_eq!(
            Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle),
            Piece::from("dexc")
        );

        assert_eq!(
            Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle),
            "DEXC".into()
        );

        assert_eq!(
            Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle),
            "dexc".into()
        );
    }

    #[test]
    #[should_panic]
    fn from_error_string_slice() {
        let x = Piece::from("DESCC");
        println!("{}", x);
    }

    #[test]
    fn from_into_chars() {
        assert_eq!(
            Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle),
            Piece::from(['D', 'E', 'X', 'C'])
        );
        assert_eq!(
            Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle),
            Piece::from(['d', 'e', 'x', 'c'])
        );

        assert_eq!(
            Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle),
            ['D', 'E', 'X', 'C'].into()
        );
        assert_eq!(
            Piece::new(Color::Dark, Hole::Empty, Height::Small, Shape::Circle),
            ['d', 'e', 'x', 'c'].into()
        );
    }

    #[test]
    fn test_are_cells_winning() {
        let mut v_3 = vec![
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Circle),
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Circle),
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Circle)
        ];
        assert_eq!(Piece::check_piece_is_winning(&mut v_3),  false);

        let mut v_4_1 = vec![
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Circle),
            Piece::new(Color::Dark, Hole::Full, Height::Tall, Shape::Circle),
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Circle),
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Square)
        ];
        assert_eq!(Piece::check_piece_is_winning(&mut v_4_1),  false);

        let mut v_4_2 = vec![
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Circle),
            Piece::new(Color::White, Hole::Full, Height::Tall, Shape::Circle),
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Circle),
            Piece::new(Color::White, Hole::Empty, Height::Small, Shape::Square)
        ];
        assert_eq!(Piece::check_piece_is_winning(&mut v_4_2),  true);
    }

    #[test]
    #[should_panic]
    fn from_error_chars() {
        let x = Piece::from(['D', 'E', 'S', 'Z']);
        println!("{}", x);
    }
}
