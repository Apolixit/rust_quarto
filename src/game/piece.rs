use std::fmt::Display;

//Represent piece settings
#[derive(Clone)]
pub struct Piece {
    color: Color,
    hole: Hole,
    height: Height,
    shape: Shape
}

pub trait PieceFeature {
    fn acronym(&self) -> &str;
}

#[derive(Clone)]
pub enum Color {
    White,
    Dark
}

impl PieceFeature for Color {
    fn acronym(&self) -> &str {
        match self {
            Self::White => "WT",
            Self::Dark => "DK"
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.acronym())
    }
}

#[derive(Clone)]
pub enum Hole {
    Empty,
    Full
}

impl PieceFeature for Hole {
    fn acronym(&self) -> &str {
        match self {
            Self::Empty => "EM",
            Self::Full => "FU"
        }
    }
}
impl Display for Hole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.acronym())
    }
}

#[derive(Clone)]
pub enum Height {
    Small,
    Tall
}
impl PieceFeature for Height {
    fn acronym(&self) -> &str {
        match self {
            Self::Small => "SM",
            Self::Tall => "TL"
        }
    }
}
impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.acronym())
    }
}


#[derive(Clone)]
pub enum Shape {
    Circle,
    Square
}

impl PieceFeature for Shape {
    fn acronym(&self) -> &str {
        match self {
            Self::Circle => "CL",
            Self::Square => "SQ"
        }
    }
}
impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.acronym())
    }
}


impl Piece {
    pub fn new(color: Color, hole: Hole, height: Height, shape: Shape) -> Self {
        Self { color, hole, height, shape }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}