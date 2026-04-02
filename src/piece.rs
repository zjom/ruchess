use crate::color::Color;

pub const NUM_ROLES: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Role {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

impl Role {
    pub const ALL: [Role; NUM_ROLES] = [
        Role::Pawn,
        Role::Rook,
        Role::Bishop,
        Role::Knight,
        Role::Queen,
        Role::King,
    ];
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pawn => "P",
            Self::Rook => "R",
            Self::Knight => "N",
            Self::Bishop => "B",
            Self::Queen => "Q",
            Self::King => "K",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece(pub Role, pub Color);

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Piece(Role::Pawn, Color::White) => "♙",
            Piece(Role::Knight, Color::White) => "♘",
            Piece(Role::Bishop, Color::White) => "♗",
            Piece(Role::Rook, Color::White) => "♖",
            Piece(Role::Queen, Color::White) => "♕",
            Piece(Role::King, Color::White) => "♔",
            Piece(Role::Pawn, Color::Black) => "♟",
            Piece(Role::Knight, Color::Black) => "♞",
            Piece(Role::Bishop, Color::Black) => "♝",
            Piece(Role::Rook, Color::Black) => "♜",
            Piece(Role::Queen, Color::Black) => "♛",
            Piece(Role::King, Color::Black) => "♚",
        };
        write!(f, "{symbol}")
    }
}
