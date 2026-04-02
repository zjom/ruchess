mod bitboard;
mod color;
mod error;
mod piece;
mod square;

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::piece::{NUM_PIECES, Piece};
use crate::square::Square;

pub const NUM_COLORS: usize = 2;

#[derive(Clone, Copy)]
pub struct Board {
    /// Indexed as pieces[color][piece]
    pub pieces: [[Bitboard; NUM_PIECES]; NUM_COLORS],
    /// Aggregate per side, indexed as sides[color]
    pub sides: [Bitboard; NUM_COLORS],
    pub total: Bitboard,
}

impl Board {
    pub fn new() -> Board {
        let pieces = [
            // White
            [
                Bitboard(0xff00), // Pawn
                Bitboard(0x81),   // Rook
                Bitboard(0x42),   // Knight
                Bitboard(0x24),   // Bishop
                Bitboard(0x8),    // Queen
                Bitboard(0x10),   // King
            ],
            // Black
            [
                Bitboard(0xff000000000000),   // Pawn
                Bitboard(0x8100000000000000), // Rook
                Bitboard(0x4200000000000000), // Knight
                Bitboard(0x2400000000000000), // Bishop
                Bitboard(0x800000000000000),  // Queen
                Bitboard(0x1000000000000000), // King
            ],
        ];

        Board {
            pieces,
            sides: [Bitboard(0xffff), Bitboard(0xffff000000000000)],
            total: Bitboard(0xffff00000000ffff),
        }
    }

    pub fn bb(&self, color: Color, piece: Piece) -> Bitboard {
        self.pieces[color as usize][piece as usize]
    }

    pub fn bb_mut(&mut self, color: Color, piece: Piece) -> &mut Bitboard {
        &mut self.pieces[color as usize][piece as usize]
    }

    pub fn side(&self, color: Color) -> Bitboard {
        self.sides[color as usize]
    }

    pub fn move_(&self, from: Square, to: Square) -> Board {
        let mut board = *self;

        let color = if self.sides[0].contains(&from) {
            Color::White
        } else {
            Color::Black
        };
        let opponent = if color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        // Find which piece is on `from` and move it
        for piece_idx in 0..NUM_PIECES {
            if board.pieces[color as usize][piece_idx].contains(&from) {
                board.pieces[color as usize][piece_idx] =
                    board.pieces[color as usize][piece_idx].move_(&from, &to);
                break;
            }
        }

        // Clear captures from all opponent boards (except king)
        for piece_idx in 0..NUM_PIECES - 1 {
            board.pieces[opponent as usize][piece_idx] =
                board.pieces[opponent as usize][piece_idx].clear_pos(&to);
        }

        self.compute_agg()
    }

    pub fn compute_agg(&self) -> Board {
        let mut board = *self;
        for c in 0..NUM_COLORS {
            board.sides[c] = Bitboard(board.pieces[c].iter().fold(0u64, |acc, bb| acc | bb.0));
        }
        board.total = Bitboard(board.sides[0].0 | board.sides[1].0);

        board
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            write!(f, " {} ", rank + 1)?;
            for file in 0..8 {
                let sq = Square(rank * 8 + file);
                let mut found = None;
                'search: for color in [Color::White, Color::Black] {
                    for piece in Piece::ALL {
                        if self.bb(color, piece).contains(&sq) {
                            found = Some(PieceWithColor(piece, color));
                            break 'search;
                        }
                    }
                }
                match found {
                    Some(p) => write!(f, " {p}")?,
                    None => write!(f, " .")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "    a b c d e f g h")
    }
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
