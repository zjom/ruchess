use crate::{bitboard::Bitboard, board::Board, color::Color, mve::Move, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnmovedRooks(Bitboard);
impl UnmovedRooks {
    /// All four rooks unmoved — the standard starting position.
    /// Bits set: A1, H1, A8, H8.
    const INIT: UnmovedRooks = UnmovedRooks(Bitboard(0x8100000000000081));
    const NONE: UnmovedRooks = UnmovedRooks(Bitboard::EMPTY);
    pub fn standard() -> Self {
        Self::INIT
    }
    pub fn from_board(board: Board) -> Self {
        let wr = board.rooks() & board.white() & Color::White.back_rank();
        let br = board.rooks() & board.black() & Color::Black.back_rank();
        Self(wr | br)
    }

    pub fn update(self, m: &Move) -> Self {
        UnmovedRooks(!(Bitboard::from(m.orig) | Bitboard::from(m.dest)) & self)
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::NONE
    }

    pub fn contains(&self, sq: Square) -> bool {
        self.0.is_set(sq)
    }
}

impl From<UnmovedRooks> for Bitboard {
    fn from(value: UnmovedRooks) -> Self {
        value.0
    }
}
