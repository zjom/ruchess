use crate::{
    castles::Castles,
    halfmoveclock::HalfMoveClock,
    hash::{Hash, PositionHash},
    mve::Move,
    position::Position,
    role::Role,
    uci::Uci,
    unmoved_rooks::UnmovedRooks,
};

#[derive(Debug, Clone, PartialEq)]
pub struct History {
    pub last_move: Option<Uci>,
    pub castles: Castles,
    pub unmoved_rooks: UnmovedRooks,
    pub half_move_clock: HalfMoveClock,
    pub position_hashes: PositionHash,
}

impl History {
    pub fn new() -> Self {
        Self {
            last_move: None,
            castles: Castles::standard(),
            unmoved_rooks: UnmovedRooks::standard(),
            half_move_clock: HalfMoveClock::new(),
            position_hashes: PositionHash::empty(),
        }
    }

    #[must_use]
    pub fn with_castles(self, castles: Castles) -> Self {
        Self { castles, ..self }
    }

    #[must_use]
    pub fn push_position(self, position: &Position) -> Self {
        let entry = PositionHash::from_hash(Hash::from_position(position));
        Self {
            position_hashes: entry.combine(&self.position_hashes),
            ..self
        }
    }

    #[must_use]
    pub fn update(self, prev: &Position, mve: &Move) -> Self {
        let entry = PositionHash::from_hash(Hash::from_position(prev));
        let position_hashes = entry.combine(&self.position_hashes);

        let half_move_clock = if mve.piece.role == Role::Pawn || mve.capture.is_some() {
            self.half_move_clock.reset()
        } else {
            self.half_move_clock.incr()
        };

        Self {
            position_hashes,
            last_move: Some((*mve).into()),
            castles: self.castles.update(mve),
            unmoved_rooks: self.unmoved_rooks.update(mve),
            half_move_clock,
        }
    }

    pub fn is_threefold_repetition(&self) -> bool {
        self.position_hashes.is_repetition(3)
    }

    pub fn is_fivefold_repetition(&self) -> bool {
        self.position_hashes.is_repetition(5)
    }

    pub fn half_moves(&self) -> u8 {
        self.half_move_clock.get()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
