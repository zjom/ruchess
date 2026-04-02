mod bitboard;
mod piece;
mod square;

use crate::bitboard::Bitboard;
use crate::piece::Piece;

pub struct Board {
    // white, black
    pub pawns: (Bitboard, Bitboard),
    pub rooks: (Bitboard, Bitboard),
    pub bishops: (Bitboard, Bitboard),
    pub knights: (Bitboard, Bitboard),
    pub kings: (Bitboard, Bitboard),
    pub queens: (Bitboard, Bitboard),

    // aggregates
    pub white: Bitboard,
    pub black: Bitboard,
    pub total: Bitboard,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pawns: (Bitboard(0xff00), Bitboard(0xff000000000000)),
            rooks: (Bitboard(0x81), Bitboard(0x8100000000000000)),
            bishops: (Bitboard(0x24), Bitboard(0x2400000000000000)),
            knights: (Bitboard(0x42), Bitboard(0x4200000000000000)),
            queens: (Bitboard(0x8), Bitboard(0x800000000000000)),
            kings: (Bitboard(0x10), Bitboard(0x1000000000000000)),
            white: Bitboard(0xffff),
            black: Bitboard(0xffff000000000000),
            total: Bitboard(0xffff00000000ffff),
        }
    }
}
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            write!(f, " {} ", rank + 1)?;
            for file in 0..8 {
                let sq = Bitboard(1u64 << (rank * 8 + file));
                let piece = if (self.pawns.0 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Pawn, Color::White))
                } else if (self.pawns.1 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Pawn, Color::Black))
                } else if (self.rooks.0 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Rook, Color::White))
                } else if (self.rooks.1 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Rook, Color::Black))
                } else if (self.bishops.0 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Bishop, Color::White))
                } else if (self.bishops.1 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Bishop, Color::Black))
                } else if (self.knights.0 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Knight, Color::White))
                } else if (self.knights.1 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Knight, Color::Black))
                } else if (self.queens.0 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Queen, Color::White))
                } else if (self.queens.1 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::Queen, Color::Black))
                } else if (self.kings.0 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::King, Color::White))
                } else if (self.kings.1 & sq).0 != 0 {
                    Some(PieceWithColor(Piece::King, Color::Black))
                } else {
                    None
                };

                match piece {
                    Some(p) => write!(f, " {p}")?,
                    None => write!(f, " .")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "    a b c d e f g h")
    }
}

enum Color {
    White,
    Black,
}

struct PieceWithColor(Piece, Color);

impl std::fmt::Display for PieceWithColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            // White Pieces
            PieceWithColor(Piece::Pawn, Color::White) => "♙",
            PieceWithColor(Piece::Knight, Color::White) => "♘",
            PieceWithColor(Piece::Bishop, Color::White) => "♗",
            PieceWithColor(Piece::Rook, Color::White) => "♖",
            PieceWithColor(Piece::Queen, Color::White) => "♕",
            PieceWithColor(Piece::King, Color::White) => "♔",

            // Black Pieces
            PieceWithColor(Piece::Pawn, Color::Black) => "♟",
            PieceWithColor(Piece::Knight, Color::Black) => "♞",
            PieceWithColor(Piece::Bishop, Color::Black) => "♝",
            PieceWithColor(Piece::Rook, Color::Black) => "♜",
            PieceWithColor(Piece::Queen, Color::Black) => "♛",
            PieceWithColor(Piece::King, Color::Black) => "♚",
        };

        write!(f, "{}", symbol)
    }
}

// const WHITE_PAWN_INIT: Bitboard = Bitboard(1);
