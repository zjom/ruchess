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
#[derive(Debug, Default, Clone, Copy)]
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

    fn is_even(&self) -> bool {
        (self.0 & 1) == 0
    }
}
