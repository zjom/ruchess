//! # Plies
//!
//! A "ply" is a half-move — a single move by one side. Two plies make one
//! full move in chess notation. [`Ply`] is the counter a
//! [`Position`](crate::position::Position) uses to know whose turn it is and
//! which full move number the game is on.

use crate::color::Color;

/// A counter of half-moves played, starting at zero.
///
/// Even values correspond to White to move; odd values to Black to move.
/// A 6-ply search depth, for example, means three full moves.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ply(u32);

impl Ply {
    /// Returns a new ply counter initialized to zero (White to move,
    /// full move 1).
    ///
    /// # Example
    /// ```
    /// # use ruchess::ply::Ply;
    /// # use ruchess::color::Color;
    /// let p = Ply::new();
    /// assert_eq!(p.turn(), Color::White);
    /// assert_eq!(p.full_move_number(), 1);
    /// ```
    pub fn new() -> Self {
        Self(0)
    }

    /// Constructs a new ply counter initialized to `ply`.
    pub fn with(ply: u32) -> Self {
        Self(ply)
    }

    /// Returns the side whose turn it is.
    ///
    /// # Example
    /// ```
    /// # use ruchess::ply::Ply;
    /// # use ruchess::color::Color;
    /// assert_eq!(Ply::new().turn(), Color::White);
    /// assert_eq!(Ply::new().incr().turn(), Color::Black);
    /// ```
    #[inline]
    pub fn turn(&self) -> Color {
        if self.is_even() {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Returns a new ply with the counter advanced by one half-move.
    ///
    /// # Example
    /// ```
    /// # use ruchess::ply::Ply;
    /// let p = Ply::new().incr().incr();
    /// assert_eq!(p.full_move_number(), 2);
    /// ```
    pub fn incr(self) -> Self {
        Self(self.0 + 1)
    }

    /// Returns a new ply with the counter rolled back one half-move.
    ///
    /// # Panics
    /// Panics if called on `Ply::new()` (counter would underflow).
    ///
    /// # Example
    /// ```
    /// # use ruchess::ply::Ply;
    /// assert_eq!(Ply::new().incr().decr(), Ply::new());
    /// ```
    pub fn decr(self) -> Self {
        Self(self.0 - 1)
    }

    /// Returns the current full-move number (1-based), as it would appear in
    /// chess notation.
    ///
    /// # Example
    /// ```
    /// # use ruchess::ply::Ply;
    /// assert_eq!(Ply::new().full_move_number(), 1);
    /// assert_eq!(Ply::new().incr().full_move_number(), 1); // still move 1
    /// assert_eq!(Ply::new().incr().incr().full_move_number(), 2);
    /// ```
    pub fn full_move_number(&self) -> u32 {
        1 + self.0 / 2
    }

    /// Creates the [`Ply`] for full-move number `full_moves` (1-based) with
    /// `turn` to move — the inverse of [`Ply::full_move_number`] and
    /// [`Ply::turn`]. A full-move number of 0 is treated as 1.
    ///
    /// # Example
    /// ```
    /// # use ruchess::ply::Ply;
    /// # use ruchess::color::Color;
    /// assert_eq!(Ply::from_full_moves(1, Color::White), Ply::new());
    /// assert_eq!(Ply::from_full_moves(1, Color::Black), Ply::new().incr());
    /// let p = Ply::from_full_moves(5, Color::Black);
    /// assert_eq!(p.full_move_number(), 5);
    /// assert_eq!(p.turn(), Color::Black);
    /// ```
    pub fn from_full_moves(full_moves: u32, turn: Color) -> Self {
        let black = matches!(turn, Color::Black) as u32;
        Ply(full_moves.saturating_sub(1) * 2 + black)
    }

    /// Returns this ply adjusted so `turn` is to move, keeping the full-move
    /// number.
    ///
    /// # Example
    /// ```
    /// # use ruchess::ply::Ply;
    /// # use ruchess::color::Color;
    /// let p = Ply::from_full_moves(3, Color::White).with_turn(Color::Black);
    /// assert_eq!(p, Ply::from_full_moves(3, Color::Black));
    /// ```
    #[must_use]
    pub fn with_turn(self, turn: Color) -> Self {
        Self::from_full_moves(self.full_move_number(), turn)
    }

    fn is_even(&self) -> bool {
        (self.0 & 1) == 0
    }
}
