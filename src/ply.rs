//! # Plies
//!
//! A "ply" is a half-move — a single move by one side. Two plies make one
//! full move in chess notation. [`Ply`] is the counter used by
//! [`Game`](crate::game::Game) to track whose turn it is and which full move
//! number the game is on.

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

    /// Creates a [`Ply`] at the start of the given full-move number (i.e., White's
    /// half-move within that move).
    ///
    /// This is the inverse of [`Ply::full_move_number`]:
    /// `Ply::from_full_moves(p.full_move_number()).full_move_number() == p.full_move_number()`
    ///
    /// # Example
    /// ```
    /// # use ruchess::ply::Ply;
    /// assert_eq!(Ply::from_full_moves(1), Ply::new());           // ply 0
    /// assert_eq!(Ply::from_full_moves(2), Ply::new().incr().incr()); // ply 2
    /// assert_eq!(Ply::from_full_moves(1).full_move_number(), 1);
    /// assert_eq!(Ply::from_full_moves(5).full_move_number(), 5);
    /// ```
    pub fn from_full_moves(full_moves: u32) -> Self {
        Ply((full_moves - 1) * 2)
    }

    fn is_even(&self) -> bool {
        (self.0 & 1) == 0
    }
}
