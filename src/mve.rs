//! # Moves
//!
//! A [`Move`] is a chess move spec packed into 16 bits: origin and
//! destination squares (6 bits each), a 2-bit promotion-role field, and a
//! 2-bit flag distinguishing normal moves from promotions, en-passant
//! captures, and castling. It does not carry the moving piece, captured
//! piece, or the resulting board — those are derived from the
//! [`Position`](crate::position::Position) when the move is applied via
//! [`Position::make`](crate::position::Position::make).
//!
//! Construct moves through the named constructors ([`Move::normal`],
//! [`Move::castle`], [`Move::enpassant`], [`Move::promotion`]) rather than by
//! hand — they fill in the flag and promotion bits correctly.

use arrayvec::ArrayVec;

use crate::{file::File, role::PromotableRole, side::Side, square::Square};

const ORIG_SHIFT: u16 = 0;
const DEST_SHIFT: u16 = 6;
const PROMO_SHIFT: u16 = 12;
const FLAG_SHIFT: u16 = 14;

const SQ_MASK: u16 = 0b11_1111;
const PROMO_MASK: u16 = 0b11;
const FLAG_MASK: u16 = 0b11;

const FLAG_NORMAL: u16 = 0;
const FLAG_PROMOTION: u16 = 1;
const FLAG_ENPASSANT: u16 = 2;
const FLAG_CASTLE: u16 = 3;

/// A chess move, packed into 16 bits.
///
/// Layout:
/// - bits 0-5: origin square (0..64)
/// - bits 6-11: destination square (0..64)
/// - bits 12-13: promotion role (only meaningful when the flag is `Promotion`)
/// - bits 14-15: flag (`Normal`, `Promotion`, `EnPassant`, `Castle`)
///
/// Whether a move is a capture is not encoded — it is determined at apply
/// time by whether the destination square is occupied by an opponent piece
/// (or, for en-passant, by the flag).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Move(u16);

impl Move {
    /// Builds a normal move (quiet push or simple capture) from `orig` to `dest`.
    #[inline]
    pub const fn normal(orig: Square, dest: Square) -> Self {
        Self::pack(orig, dest, 0, FLAG_NORMAL)
    }

    /// Builds a castling move from the king's origin to its destination.
    /// The castling side falls out of the destination file.
    #[inline]
    pub const fn castle(king_from: Square, king_to: Square) -> Self {
        Self::pack(king_from, king_to, 0, FLAG_CASTLE)
    }

    /// Builds an en-passant capture from `orig` to `dest`. The captured
    /// pawn's square is derived from `dest` and the side to move at apply
    /// time.
    #[inline]
    pub const fn enpassant(orig: Square, dest: Square) -> Self {
        Self::pack(orig, dest, 0, FLAG_ENPASSANT)
    }

    /// Builds a promotion move (capture or push) from `orig` to `dest`,
    /// promoting to `role`.
    #[inline]
    pub const fn promotion(orig: Square, dest: Square, role: PromotableRole) -> Self {
        Self::pack(orig, dest, promo_bits(role), FLAG_PROMOTION)
    }

    #[inline]
    const fn pack(orig: Square, dest: Square, promo: u16, flag: u16) -> Self {
        let bits = ((orig.0 as u16) << ORIG_SHIFT)
            | ((dest.0 as u16) << DEST_SHIFT)
            | (promo << PROMO_SHIFT)
            | (flag << FLAG_SHIFT);
        Self(bits)
    }

    /// Returns the origin square.
    #[inline]
    pub const fn orig(self) -> Square {
        Square::new(((self.0 >> ORIG_SHIFT) & SQ_MASK) as u8)
    }

    /// Returns the destination square.
    #[inline]
    pub const fn dest(self) -> Square {
        Square::new(((self.0 >> DEST_SHIFT) & SQ_MASK) as u8)
    }

    /// Returns the promotion role if this is a promotion move.
    #[inline]
    pub fn promo_role(self) -> Option<PromotableRole> {
        if self.flag() == FLAG_PROMOTION {
            Some(promo_from_bits((self.0 >> PROMO_SHIFT) & PROMO_MASK))
        } else {
            None
        }
    }

    /// Returns `true` if this is a promotion move.
    #[inline]
    pub fn is_promotion(self) -> bool {
        self.flag() == FLAG_PROMOTION
    }

    /// Returns `true` if this is an en-passant capture.
    #[inline]
    pub fn is_enpassant(self) -> bool {
        self.flag() == FLAG_ENPASSANT
    }

    /// Returns `true` if this is a castling move.
    #[inline]
    pub fn is_castle(self) -> bool {
        self.flag() == FLAG_CASTLE
    }

    /// Returns the castling side if this is a castle, otherwise `None`.
    /// Derived from the destination file (G → king-side, C → queen-side).
    #[inline]
    pub fn castle_side(self) -> Option<Side> {
        if !self.is_castle() {
            return None;
        }
        Some(if self.dest().file() == File::G {
            Side::King
        } else {
            Side::Queen
        })
    }

    #[inline]
    fn flag(self) -> u16 {
        (self.0 >> FLAG_SHIFT) & FLAG_MASK
    }
}

const fn promo_bits(role: PromotableRole) -> u16 {
    match role {
        PromotableRole::Knight => 0,
        PromotableRole::Bishop => 1,
        PromotableRole::Rook => 2,
        PromotableRole::Queen => 3,
    }
}

const fn promo_from_bits(bits: u16) -> PromotableRole {
    match bits & PROMO_MASK {
        0 => PromotableRole::Knight,
        1 => PromotableRole::Bishop,
        2 => PromotableRole::Rook,
        _ => PromotableRole::Queen,
    }
}

/// A container for moves that can be stored inline on the stack.
///
/// The capacity is limited, but there is enough space to hold the legal
/// moves of any chess position, including any of the supported chess variants,
/// if enabled.
///
/// # Example
///
/// ```
/// use ruchess::{position::Position, role::Role};
///
/// let pos = Position::new();
/// let moves = pos.valid_moves();
/// let pawn_count = moves.iter().filter(|m| {
///     pos.board().role_at(m.orig()) == Some(Role::Pawn)
/// }).count();
/// assert_eq!(pawn_count, 16);
/// ```
pub type MoveList = ArrayVec<Move, 270>;
