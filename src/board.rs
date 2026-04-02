use crate::bitboard::Bitboard;
use crate::color::{Color, NUM_COLORS};
use crate::piece::{NUM_ROLES, Piece, Role};
use crate::square::Square;

// Board does not care or know about the rules of the game.
// Board only cares about the state of the board and moving pieces.
#[derive(Clone, Copy)]
pub struct Board {
    /// Indexed as `pieces[color][piece]`
    pieces: [[Bitboard; NUM_ROLES]; NUM_COLORS],
    /// Aggregate bitboard per side, indexed as `sides[color]`
    sides: [Bitboard; NUM_COLORS],
    occupied: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bb(&self, color: Color, piece: Role) -> Bitboard {
        self.pieces[color as usize][piece as usize]
    }

    pub fn bb_mut(&mut self, color: Color, piece: Role) -> &mut Bitboard {
        &mut self.pieces[color as usize][piece as usize]
    }

    pub fn side(&self, color: Color) -> Bitboard {
        self.sides[color as usize]
    }

    pub fn total(&self) -> Bitboard {
        self.occupied
    }

    // whether a square occupied by a piece
    pub fn is_occupied(&self, s: &Square) -> bool {
        self.occupied.contains(s)
    }

    pub fn piece_at(&self, s: &Square) -> Option<Piece> {
        if !self.is_occupied(s) {
            return None;
        }

        let color = if self.sides[Color::White as usize].contains(s) {
            Color::White
        } else {
            Color::Black
        };
        for piece_idx in 0..NUM_ROLES {
            if self.pieces[color as usize][piece_idx].contains(s) {
                return Some(Piece(Role::ALL[piece_idx], color));
            }
        }
        None
    }

    pub fn move_(&self, from: &Square, to: &Square) -> Self {
        if !self.is_occupied(from) {
            return *self;
        }
        let mut board = *self;

        let color = if self.sides[Color::White as usize].contains(&from) {
            Color::White
        } else {
            Color::Black
        };
        let opponent = color.opponent();

        for piece_idx in 0..NUM_ROLES {
            if board.pieces[color as usize][piece_idx].contains(&from) {
                board.pieces[color as usize][piece_idx] =
                    board.pieces[color as usize][piece_idx].move_(&from, &to);
                break;
            }
        }

        for piece_idx in 0..NUM_ROLES {
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
        board.occupied = Bitboard(board.sides[0].0 | board.sides[1].0);
        board
    }

    pub fn as_grid(&self) -> [[Option<Piece>; 8]; 8] {
        let mut grid = [[None; 8]; 8];
        for color in [Color::White, Color::Black] {
            for piece in Role::ALL {
                let mut bb = self.bb(color, piece);
                while bb.0 != 0 {
                    let sq = bb.0.trailing_zeros() as usize;
                    let rank = sq / 8;
                    let file = sq % 8;
                    grid[rank][file] = Some(Piece(piece, color));
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
            occupied: Bitboard(0xffff00000000ffff),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::square::{A1, D4, H8};

    use super::*;

    #[test]
    fn piece() {
        let board = Board::new();
        assert_eq!(board.piece_at(&A1), Some(Piece(Role::Rook, Color::White)));

        assert_eq!(board.piece_at(&H8), Some(Piece(Role::Rook, Color::Black)));

        assert_eq!(board.piece_at(&D4), None);
    }
}
