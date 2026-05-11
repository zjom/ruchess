pub mod bitboard;

pub mod attacks;
pub mod magic;

pub mod board;

pub mod square;

pub mod role;

pub mod color;

pub mod piece {
    use crate::{color::Color, role::Role};
    #[derive(Debug, Clone, Copy)]
    pub struct Piece {
        pub role: Role,
        pub color: Color,
    }

    impl Piece {
        pub fn as_char(&self) -> char {
            let c = self.role.as_ascii();
            match self.color {
                Color::White => c,
                Color::Black => c.to_ascii_uppercase(),
            }
        }
    }
}

mod mve {}

mod game {}
