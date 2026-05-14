//! # Pieces
//!
//! A [`Piece`] pairs a [`Role`] (pawn, knight, …) with a [`Color`] (white,
//! black). It is the unit placed on a [`Board`](crate::board::Board) square.

use crate::{color::Color, role::Role};

/// A single chess piece: a role together with the color of the player who
/// owns it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    /// The piece type (pawn, knight, bishop, rook, queen, or king).
    pub role: Role,
    /// The owning side.
    pub color: Color,
}

impl Piece {
    /// Returns the FEN-style character for this piece: lowercase letter for
    /// White, uppercase for Black.
    ///
    /// Note that this is the inverse of the usual FEN convention; consumers
    /// rendering FEN should flip the case themselves.
    ///
    /// # Example
    /// ```
    /// # use ruchess::piece::Piece;
    /// # use ruchess::role::Role;
    /// # use ruchess::color::Color;
    /// let white_king = Piece { role: Role::King, color: Color::White };
    /// let black_king = Piece { role: Role::King, color: Color::Black };
    /// assert_eq!(white_king.as_char(), 'k');
    /// assert_eq!(black_king.as_char(), 'K');
    /// ```
    pub fn as_char(&self) -> char {
        let c = self.role.as_ascii();
        match self.color {
            Color::White => c,
            Color::Black => c.to_ascii_uppercase(),
        }
    }
}
