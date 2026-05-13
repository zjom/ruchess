use crate::{
    bitboard::Bitboard,
    color::Color,
    side::Side,
    square::{self, Square},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Castles(Bitboard);

impl Castles {
    /// All four castling rights available — the standard starting position.
    /// Bits set: A1, H1, A8, H8.
    const INIT: Castles = Castles(Bitboard(0x8100000000000081));
    const NONE: Castles = Castles(Bitboard::EMPTY);

    pub fn standard() -> Self {
        Self::INIT
    }

    pub fn new(
        white_king_side: bool,
        white_queen_side: bool,
        black_king_side: bool,
        black_queen_side: bool,
    ) -> Self {
        let mut bb = Bitboard(0);
        if white_king_side {
            bb |= Bitboard::from(square::H1);
        }
        if white_queen_side {
            bb |= Bitboard::from(square::A1);
        }
        if black_king_side {
            bb |= Bitboard::from(square::H8);
        }
        if black_queen_side {
            bb |= Bitboard::from(square::A8);
        }
        Castles(bb)
    }

    /// Construct from an arbitrary bitboard, masking off any bits that aren't
    /// one of the four valid castling squares.
    pub fn from_bitboard(bb: Bitboard) -> Self {
        Castles(bb & Self::INIT.0)
    }

    pub fn is_empty(self) -> bool {
        self == Self::NONE
    }

    pub fn contains(self, square: Square) -> bool {
        (self.0 & Bitboard::from(square)) != Bitboard(0)
    }

    /// Does `color` have any castling rights left?
    pub fn can(self, color: Color) -> bool {
        (self.0 & color.back_rank()) != Bitboard(0)
    }

    /// Does `color` have rights on this specific side?
    pub fn can_side(self, color: Color, side: Side) -> bool {
        self.contains(color.castle_square(side))
    }

    pub fn white_king_side(self) -> bool {
        self.contains(square::H1)
    }
    pub fn white_queen_side(self) -> bool {
        self.contains(square::A1)
    }
    pub fn black_king_side(self) -> bool {
        self.contains(square::H8)
    }
    pub fn black_queen_side(self) -> bool {
        self.contains(square::A8)
    }

    /// Remove every castling right belonging to `color` (e.g. when its king moves).
    pub fn without(self, color: Color) -> Self {
        Castles(self.0 & !Bitboard::from(color.back_rank()))
    }

    /// Remove a single castling right.
    pub fn without_side(self, color: Color, side: Side) -> Self {
        Castles(self.0 & !Bitboard::from(color.castle_square(side)))
    }

    pub fn add(self, color: Color, side: Side) -> Self {
        Castles(self.0 | Bitboard::from(color.castle_square(side)))
    }

    /// Replace `color`'s rights wholesale.
    pub fn update(self, color: Color, king_side: bool, queen_side: bool) -> Self {
        let stripped = self.without(color).0;
        let ks = if king_side {
            Bitboard::from(color.castle_square(Side::King))
        } else {
            Bitboard(0)
        };
        let qs = if queen_side {
            Bitboard::from(color.castle_square(Side::Queen))
        } else {
            Bitboard(0)
        };
        Castles(stripped | ks | qs)
    }

    pub fn to_array(self) -> [bool; 4] {
        [
            self.white_king_side(),
            self.white_queen_side(),
            self.black_king_side(),
            self.black_queen_side(),
        ]
    }

    /// FEN castling letter → the rook square it represents (X-FEN not supported).
    pub fn char_to_square(c: char) -> Option<Square> {
        match c {
            'K' => Some(square::H1),
            'Q' => Some(square::A1),
            'k' => Some(square::H8),
            'q' => Some(square::A8),
            _ => None,
        }
    }
}

impl From<Castles> for Bitboard {
    fn from(c: Castles) -> Bitboard {
        c.0
    }
}
