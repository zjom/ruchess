use crate::{
    castles::Castles, halfmoveclock::HalfMoveClock, uci::Uci, unmoved_rooks::UnmovedRooks,
};

#[derive(Debug, Clone)]
pub struct History {
    pub last_move: Option<Uci>,
    pub castles: Castles,
    pub unmoved_rooks: UnmovedRooks,
    pub half_move_clock: HalfMoveClock,
}

impl History {
    pub fn new() -> Self {
        Self {
            last_move: None,
            castles: Castles::standard(),
            unmoved_rooks: UnmovedRooks::standard(),
            half_move_clock: HalfMoveClock::new(),
        }
    }

    pub fn with_castles(self, castles: Castles) -> Self {
        Self { castles, ..self }
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
