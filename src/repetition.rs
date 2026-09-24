//! # Repetition trail
//!
//! The record of positions that occurred before the current one, used to
//! answer [`Position::repetitions`]. It holds only *past* positions: the
//! current position is hashed when asked, so builders that edit a
//! [`Position`] can never leave a stale entry behind.
//!
//! [`Position::make`] records the position it leaves and
//! [`Position::unmake`] forgets it again.

use crate::{hash::Hash, position::Position};

/// Hashes of earlier positions in the game, oldest first, or `Disabled` when
/// repetition tracking is off (perft, fixed-depth search).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum RepetitionTrail {
    #[default]
    Disabled,
    Enabled(Vec<Hash>),
}

impl RepetitionTrail {
    /// An empty, enabled trail.
    pub(crate) fn new() -> Self {
        Self::Enabled(Vec::new())
    }

    /// Records `position` as the one being left by the next move. No-op when
    /// disabled.
    #[inline]
    pub(crate) fn record(&mut self, position: &Position) {
        if let Self::Enabled(trail) = self {
            trail.push(Hash::from_position(position));
        }
    }

    /// Forgets the most recent [`Self::record`]. No-op when disabled.
    #[inline]
    pub(crate) fn forget_last(&mut self) {
        if let Self::Enabled(trail) = self {
            trail.pop();
        }
    }

    /// How many times `current` has occurred, counting itself.
    ///
    /// Only positions with the same side to move, and no further back than
    /// the half-move clock (a pawn move or capture can't be undone), are
    /// compared. Always 1 when disabled.
    pub(crate) fn repetitions(&self, current: &Position) -> usize {
        let Self::Enabled(trail) = self else {
            return 1;
        };
        let window = (current.history().half_moves() as usize).min(trail.len());
        let hash = Hash::from_position(current);
        1 + (2..=window)
            .step_by(2)
            .filter(|&plies_back| trail[trail.len() - plies_back] == hash)
            .count()
    }
}
