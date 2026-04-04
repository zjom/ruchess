use std::cell::RefCell;

use crate::bitboard::Bitboard;
use crate::color::{ByColor, Color};
use crate::piece::{ByRole, Piece, Role};
use crate::square::Square;

/// Board does not care or know about the rules of the game.
/// Board only cares about the state of the board and moving pieces.
#[derive(Clone)]
pub struct Board {
    by_role: RefCell<ByRole<Bitboard>>,
    by_color: RefCell<ByColor<Bitboard>>,
    occupied: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    /// returns bitboard of all squares occupied by Piece
    pub fn bb(&self, Piece(role, color): Piece) -> Bitboard {
        self.by_color
            .borrow()
            .get(color)
            .intersect(self.by_role.borrow().get(role))
    }

    /// checks if Square is occupied by anything
    pub fn is_occupied(&self, square: &Square) -> bool {
        self.occupied.contains(square)
    }

    /// returns Color at Square if any
    pub fn color_at(&self, square: &Square) -> Option<Color> {
        self.by_color.borrow().find(|bb| bb.contains(square))
    }

    /// returns Role at Square if any
    pub fn role_at(&self, square: &Square) -> Option<Role> {
        self.by_role.borrow().find(|bb| bb.contains(square))
    }

    /// returns Piece at Square if any
    pub fn piece_at(&self, square: &Square) -> Option<Piece> {
        self.color_at(square).map(|color| {
            self.by_role
                .borrow()
                .find_or_king(|bb| bb.contains(square))
                .of(color)
        })
    }

    /// removes piece at Square if any. returns removed piece.
    /// Use [`Board::discard_at`] if you don't need the value.
    pub fn remove_at(&mut self, square: &Square) -> Option<Piece> {
        let piece = self.piece_at(square);
        if let Some(Piece(role, color)) = piece {
            self.by_role
                .borrow_mut()
                .update(role, |bb| bb.clear(square));
            self.by_color
                .borrow_mut()
                .update(color, |bb| bb.clear(square));
            self.occupied = self.occupied.clear(square);
        }
        piece
    }

    /// removes piece at Square if any.
    /// Use [`Board::remove_at`] if you need the value.
    pub fn discard_at(&mut self, square: &Square) {
        _ = self.remove_at(square);
    }

    /// sets the square to be occupied by piece.
    pub fn set_at(&mut self, square: &Square, Piece(role, color): Piece) -> &Self {
        self.discard_at(square);

        self.by_role.borrow_mut().update(role, |bb| bb.set(square));
        self.by_color
            .borrow_mut()
            .update(color, |bb| bb.set(square));
        self.occupied = self.occupied.set(square);
        self
    }

    pub fn occupied(&self) -> Bitboard {
        self.occupied
    }

    pub fn color_bb(&self, color: Color) -> Bitboard {
        *self.by_color.borrow().get(color)
    }

    // pub fn compute_agg(&self) -> Board {
    //     let mut board = *self;
    //     for c in 0..NUM_COLORS {
    //         board.sides[c] = Bitboard(board.pieces[c].iter().fold(0u64, |acc, bb| acc | bb.0));
    //     }
    //     board.occupied = Bitboard(board.sides[0].0 | board.sides[1].0);
    //     board
    // }

    pub fn as_grid(&self) -> [[Option<Piece>; 8]; 8] {
        let mut grid = [[None; 8]; 8];
        for color in [Color::White, Color::Black] {
            for piece in Role::ALL {
                let mut bb = self.bb(piece.of(color));
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
        Self {
            by_role: RefCell::new(ByRole {
                pawn: Bitboard(0x00ff_0000_0000_ff00),
                knight: Bitboard(0x4200_0000_0000_0042),
                bishop: Bitboard(0x2400_0000_0000_0024),
                rook: Bitboard(0x8100_0000_0000_0081),
                queen: Bitboard(0x0800_0000_0000_0008),
                king: Bitboard(0x1000_0000_0000_0010),
            }),
            by_color: RefCell::new(ByColor {
                black: Bitboard(0xffff_0000_0000_0000),
                white: Bitboard(0xffff),
            }),
            occupied: Bitboard(0xffff_0000_0000_ffff),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::square::{A1, D4, H8};
//
//     use super::*;
//
//     #[test]
//     fn piece() {
//         let board = Board::new();
//         assert_eq!(board.piece_at(&A1), Some(Piece(Role::Rook, Color::White)));
//
//         assert_eq!(board.piece_at(&H8), Some(Piece(Role::Rook, Color::Black)));
//
//         assert_eq!(board.piece_at(&D4), None);
//     }
// }
