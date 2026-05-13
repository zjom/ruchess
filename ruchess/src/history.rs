use crate::{
    castles::Castles,
    halfmoveclock::HalfMoveClock,
    hash::{Hash, PositionHash},
    position::Position,
    uci::Uci,
    unmoved_rooks::UnmovedRooks,
};

#[derive(Debug, Clone)]
pub struct History {
    pub last_move: Option<Uci>,
    pub castles: Castles,
    pub unmoved_rooks: UnmovedRooks,
    pub half_move_clock: HalfMoveClock,
    pub positions: PositionHash,
}

impl History {
    pub fn new() -> Self {
        Self {
            last_move: None,
            castles: Castles::standard(),
            unmoved_rooks: UnmovedRooks::standard(),
            half_move_clock: HalfMoveClock::new(),
            positions: PositionHash::empty(),
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
            positions: entry.combine(&self.positions),
            ..self
        }
    }

    pub fn is_threefold_repetition(&self) -> bool {
        self.positions.is_repetition(3)
    }

    pub fn is_fivefold_repetition(&self) -> bool {
        self.positions.is_repetition(5)
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
