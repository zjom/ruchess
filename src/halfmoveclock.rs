//! # Half-Move Clock
//!
//! Tracks the number of reversible half-moves played since the last pawn
//! advance or capture, for use with the fifty-move draw rule.

/// The halfmove clock specifies a decimal number of half moves with respect to the 50 move draw rule.
/// It is reset to zero after a capture or a pawn move and incremented otherwise.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct HalfMoveClock(u8);
impl HalfMoveClock {
    /// Returns a fresh clock, initialized to zero.
    ///
    /// # Example
    /// ```
    /// # use ruchess::halfmoveclock::HalfMoveClock;
    /// assert_eq!(HalfMoveClock::new().get(), 0);
    /// ```
    pub fn new() -> Self {
        Self(0)
    }

    /// Returns a new clock incremented by one half-move.
    ///
    /// # Example
    /// ```
    /// # use ruchess::halfmoveclock::HalfMoveClock;
    /// assert_eq!(HalfMoveClock::new().incr().get(), 1);
    /// ```
    pub fn incr(self) -> Self {
        Self(self.0 + 1)
    }

    /// Returns the underlying half-move count.
    ///
    /// # Example
    /// ```
    /// # use ruchess::halfmoveclock::HalfMoveClock;
    /// let c = HalfMoveClock::new().incr().incr();
    /// assert_eq!(c.get(), 2);
    /// ```
    pub fn get(self) -> u8 {
        self.0
    }

    /// Returns a clock reset back to zero.
    ///
    /// Called after a pawn move or capture.
    ///
    /// # Example
    /// ```
    /// # use ruchess::halfmoveclock::HalfMoveClock;
    /// let c = HalfMoveClock::new().incr().incr().reset();
    /// assert_eq!(c.get(), 0);
    /// ```
    pub fn reset(self) -> Self {
        Self(0)
    }
}

impl From<HalfMoveClock> for u8 {
    fn from(value: HalfMoveClock) -> Self {
        value.get()
    }
}
