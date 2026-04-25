use crate::color::Color;
use crate::role::Role;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece(pub Role, pub Color);

impl Piece {
    pub fn as_char(&self) -> char {
        match self {
            Piece(Role::Pawn, Color::White) => '♙',
            Piece(Role::Knight, Color::White) => '♘',
            Piece(Role::Bishop, Color::White) => '♗',
            Piece(Role::Rook, Color::White) => '♖',
            Piece(Role::Queen, Color::White) => '♕',
            Piece(Role::King, Color::White) => '♔',
            Piece(Role::Pawn, Color::Black) => '♟',
            Piece(Role::Knight, Color::Black) => '♞',
            Piece(Role::Bishop, Color::Black) => '♝',
            Piece(Role::Rook, Color::Black) => '♜',
            Piece(Role::Queen, Color::Black) => '♛',
            Piece(Role::King, Color::Black) => '♚',
        }
    }
}

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
