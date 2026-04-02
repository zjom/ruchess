use crate::bitboard::Bitboard;
use crate::color::{Color, NUM_COLORS};
use crate::piece::{NUM_PIECES, Piece};
use crate::square::Square;

// Board does not care or know about the rules of the game.
// Board only cares about the state of the board and moving pieces.
#[derive(Clone, Copy)]
pub struct Board {
    /// Indexed as `pieces[color][piece]`
    pieces: [[Bitboard; NUM_PIECES]; NUM_COLORS],
    /// Aggregate bitboard per side, indexed as `sides[color]`
    sides: [Bitboard; NUM_COLORS],
    total: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Self::default()
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

    pub fn total(&self) -> Bitboard {
        self.total
    }

    pub fn piece_at(&self, square: &Square) -> Option<PieceWithColor> {
        if !self.total.contains(square) {
            return None;
        }

        let color = if self.sides[Color::White as usize].contains(square) {
            Color::White
        } else {
            Color::Black
        };
        for piece_idx in 0..NUM_PIECES {
            if self.pieces[color as usize][piece_idx].contains(square) {
                return Some(PieceWithColor(Piece::ALL[piece_idx], color));
            }
        }
        None
    }

    pub fn move_(&self, from: &Square, to: &Square) -> Self {
        if !self.total.contains(from) {
            return *self;
        }
        let mut board = *self;

        let color = if self.sides[Color::White as usize].contains(&from) {
            Color::White
        } else {
            Color::Black
        };
        let opponent = color.opponent();

        for piece_idx in 0..NUM_PIECES {
            if board.pieces[color as usize][piece_idx].contains(&from) {
                board.pieces[color as usize][piece_idx] =
                    board.pieces[color as usize][piece_idx].move_(&from, &to);
                break;
            }
        }

        for piece_idx in 0..NUM_PIECES {
            board.pieces[opponent as usize][piece_idx] =
                board.pieces[opponent as usize][piece_idx].clear(&to);
        }

        board.compute_agg()
    }

    pub fn compute_agg(&self) -> Board {
        let mut board = *self;
        for c in 0..NUM_COLORS {
            board.sides[c] = Bitboard(board.pieces[c].iter().fold(0u64, |acc, bb| acc | bb.0));
        }
        board.total = Bitboard(board.sides[0].0 | board.sides[1].0);
        board
    }

    pub fn as_grid(&self) -> [[Option<PieceWithColor>; 8]; 8] {
        let mut grid = [[None; 8]; 8];
        for color in [Color::White, Color::Black] {
            for piece in Piece::ALL {
                let mut bb = self.bb(color, piece);
                while bb.0 != 0 {
                    let sq = bb.0.trailing_zeros() as usize;
                    let rank = sq / 8;
                    let file = sq % 8;
                    grid[rank][file] = Some(PieceWithColor(piece, color));
                    bb.0 &= bb.0 - 1; // clear lowest set bit
                }
            }
        }
        grid
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (rank, row) in self.as_grid().iter().rev().enumerate() {
            write!(f, " {} ", 8 - rank)?;
            for cell in row {
                match cell {
                    Some(p) => write!(f, " {p}")?,
                    None => write!(f, " .")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "    a b c d e f g h")
    }
}

impl Default for Board {
    fn default() -> Self {
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
                Bitboard(0x00ff000000000000), // Pawn
                Bitboard(0x8100000000000000), // Rook
                Bitboard(0x4200000000000000), // Knight
                Bitboard(0x2400000000000000), // Bishop
                Bitboard(0x0800000000000000), // Queen
                Bitboard(0x1000000000000000), // King
            ],
        ];

        Self {
            pieces,
            sides: [Bitboard(0xffff), Bitboard(0xffff000000000000)],
            total: Bitboard(0xffff00000000ffff),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PieceWithColor(pub Piece, pub Color);

impl std::fmt::Display for PieceWithColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            PieceWithColor(Piece::Pawn, Color::White) => "♙",
            PieceWithColor(Piece::Knight, Color::White) => "♘",
            PieceWithColor(Piece::Bishop, Color::White) => "♗",
            PieceWithColor(Piece::Rook, Color::White) => "♖",
            PieceWithColor(Piece::Queen, Color::White) => "♕",
            PieceWithColor(Piece::King, Color::White) => "♔",
            PieceWithColor(Piece::Pawn, Color::Black) => "♟",
            PieceWithColor(Piece::Knight, Color::Black) => "♞",
            PieceWithColor(Piece::Bishop, Color::Black) => "♝",
            PieceWithColor(Piece::Rook, Color::Black) => "♜",
            PieceWithColor(Piece::Queen, Color::Black) => "♛",
            PieceWithColor(Piece::King, Color::Black) => "♚",
        };
        write!(f, "{symbol}")
    }
}

#[cfg(test)]
mod tests {
    use crate::square::{A1, D4, H8};

    use super::*;

    #[test]
    fn piece() {
        let board = Board::new();
        assert_eq!(
            board.piece_at(&A1),
            Some(PieceWithColor(Piece::Rook, Color::White))
        );

        assert_eq!(
            board.piece_at(&H8),
            Some(PieceWithColor(Piece::Rook, Color::Black))
        );

        assert_eq!(board.piece_at(&D4), None);
    }
}
