//! # Position History
//!
//! [`History`] bundles the per-position bookkeeping that the bare
//! [`Board`](crate::board::Board) doesn't carry: the previous move (as UCI),
//! castling rights, which rooks have not moved, the fifty-move clock, and a
//! rolling trail of Zobrist hashes used for repetition detection.
//!
//! `History` is updated only by
//! [`Position::make`](crate::position::Position::make) and
//! [`Position::unmake`](crate::position::Position::unmake).

use crate::{
    castles::Castles, halfmoveclock::HalfMoveClock, hash::PositionHash, uci::Uci,
    unmoved_rooks::UnmovedRooks,
};

/// All the per-position state that lives outside the [`Board`](crate::board::Board).
#[derive(Debug, Clone, PartialEq)]
pub struct History {
    /// The most recent move, in UCI form, or `None` at the start of a game.
    pub last_move: Option<Uci>,
    /// Castling rights currently available to both sides.
    pub castles: Castles,
    /// Squares that still contain rooks that have never moved.
    pub unmoved_rooks: UnmovedRooks,
    /// Half-moves since the last pawn move or capture (for the 50-move rule).
    pub half_move_clock: HalfMoveClock,
    /// Trail of Zobrist hashes used to detect repetitions.
    pub position_hashes: PositionHash,
}

impl History {
    /// Returns the starting-position history: no prior move, standard
    /// castling rights, all four rooks unmoved, clock at zero, empty hash
    /// trail.
    ///
    /// # Example
    /// ```
    /// # use ruchess::history::History;
    /// let h = History::new();
    /// assert!(h.last_move.is_none());
    /// assert_eq!(h.half_moves(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            last_move: None,
            castles: Castles::standard(),
            unmoved_rooks: UnmovedRooks::standard(),
            half_move_clock: HalfMoveClock::new(),
            position_hashes: PositionHash::empty(),
        }
    }

    /// Like [`Self::new`], but with repetition tracking permanently disabled.
    ///
    /// Skips the Zobrist hash computation and the `Vec<u8>` growth that
    /// [`Position::make`](crate::position::Position::make) would otherwise do
    /// on every move. Use this for
    /// perft, fixed-depth search, and any other workload that never calls
    /// [`Self::is_threefold_repetition`] or [`Self::is_fivefold_repetition`].
    ///
    /// Once disabled, the `Disabled` state propagates through every
    /// descendant position.
    pub fn new_no_repetition() -> Self {
        Self {
            last_move: None,
            castles: Castles::standard(),
            unmoved_rooks: UnmovedRooks::standard(),
            half_move_clock: HalfMoveClock::new(),
            position_hashes: PositionHash::disabled(),
        }
    }

    /// Returns a new history with `castles` replacing the current rights,
    /// leaving every other field unchanged.
    ///
    /// # Example
    /// ```
    /// # use ruchess::history::History;
    /// # use ruchess::castles::Castles;
    /// let h = History::new().with_castles(Castles::NONE);
    /// assert!(h.castles.is_empty());
    /// ```
    #[must_use]
    pub fn with_castles(self, castles: Castles) -> Self {
        Self { castles, ..self }
    }

    /// Returns `true` if the current position has appeared at least three
    /// times in the recorded hash trail.
    pub fn is_threefold_repetition(&self) -> bool {
        self.position_hashes.is_repetition(3)
    }

    /// Returns `true` if the current position has appeared at least five
    /// times in the recorded hash trail.
    pub fn is_fivefold_repetition(&self) -> bool {
        self.position_hashes.is_repetition(5)
    }

    /// Returns the current half-move clock value.
    ///
    /// # Example
    /// ```
    /// # use ruchess::history::History;
    /// assert_eq!(History::new().half_moves(), 0);
    /// ```
    pub fn half_moves(&self) -> u8 {
        self.half_move_clock.get()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
