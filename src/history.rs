//! # Position History
//!
//! [`History`] bundles the per-position bookkeeping that the bare
//! [`Board`](crate::board::Board) doesn't carry: the previous move (as UCI),
//! castling rights, which rooks have not moved, the fifty-move clock, and the
//! repetition trail.
//!
//! `History` is updated only by
//! [`Position::make`](crate::position::Position::make) and
//! [`Position::unmake`](crate::position::Position::unmake).

use crate::{
    castles::Castles, halfmoveclock::HalfMoveClock, repetition::RepetitionTrail, uci::Uci,
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
    /// Earlier positions of the game, for
    /// [`Position::repetitions`](crate::position::Position::repetitions).
    pub(crate) repetition_trail: RepetitionTrail,
}

impl History {
    /// Returns the starting-position history: no prior move, standard
    /// castling rights, all four rooks unmoved, clock at zero, empty
    /// repetition trail.
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
            repetition_trail: RepetitionTrail::new(),
        }
    }

    /// Like [`Self::new`], but with repetition tracking permanently disabled.
    ///
    /// Skips the Zobrist hash that
    /// [`Position::make`](crate::position::Position::make) would otherwise
    /// compute on every move. Use this for perft, fixed-depth search, and any
    /// other workload that never asks for
    /// [`Position::repetitions`](crate::position::Position::repetitions).
    ///
    /// Tracking stays off for every position played from this one.
    pub fn new_no_repetition() -> Self {
        Self {
            last_move: None,
            castles: Castles::standard(),
            unmoved_rooks: UnmovedRooks::standard(),
            half_move_clock: HalfMoveClock::new(),
            repetition_trail: RepetitionTrail::Disabled,
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
