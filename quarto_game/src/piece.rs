use ansi_term::Colour as ConsoleColor;
use std::fmt::Display;

    pub trait PieceFeature
    where Self: Sized
    {
        fn iterate() -> Vec<Self>;
        fn acronym(&self) -> &str;
        fn name(&self) -> &str;
        fn color(&self) -> ConsoleColor;
    }

    pub trait IterEnum {}

    #[derive(Debug, Clone, PartialEq)]
    pub enum Color {
        White,
        Dark,
    }

    impl IterEnum for Vec<Color> {}
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
                Self::Dark => "Dark"
            }
        }

        fn color(&self) -> ConsoleColor {
            ConsoleColor::Blue
        }

        fn iterate() -> Vec<Self> {
            vec![Self::White, Self::Dark]
        }
    }
    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.color().paint(self.acronym()))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Hole {
        Empty,
        Full,
    }
    impl IterEnum for Vec<Hole> {}
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
                Self::Full => "Full"
            }
        }

        fn color(&self) -> ConsoleColor {
            ConsoleColor::Purple
        }

        fn iterate() -> Vec<Self> {
            vec![Self::Empty, Self::Full]
        }
    }
    impl Display for Hole {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.color().paint(self.acronym()))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Height {
        Small,
        Tall,
    }

    impl IterEnum for Vec<Height> {}
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
                Self::Tall => "Tall"
            }
        }

        fn color(&self) -> ConsoleColor {
            ConsoleColor::Red
        }

        fn iterate() -> Vec<Self> {
            vec![Self::Small, Self::Tall]
        }
    }
    impl Display for Height {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.color().paint(self.acronym()))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Shape {
        Circle,
        Square,
    }

    impl IterEnum for Vec<Shape> {}
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
                Self::Square => "Square"
            }
        }

        fn color(&self) -> ConsoleColor {
            ConsoleColor::Green
        }

        fn iterate() -> Vec<Self> {
            vec![Self::Circle, Self::Square]
        }
    }
    impl Display for Shape {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.color().paint(self.acronym()))
        }
    }


    //Represent piece settings
    #[derive(Debug, Clone)]
    pub struct Piece {
        color: Color,
        hole: Hole, //○, □
        height: Height, //⇑, ⇓
        shape: Shape, //○, □
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
    }

    impl PartialEq for Piece {
        fn eq(&self, other: &Self) -> bool {
            self.color == other.color && self.hole == other.hole && self.height == other.height && self.shape == other.shape
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
}