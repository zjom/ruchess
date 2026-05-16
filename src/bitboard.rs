//! # Bitboards
//!
//! A [`Bitboard`] is the fundamental data structure used in `ruchess` to represent the state of
//! the board. It uses a single `u64` where each of the 64 bits represents a specific square
//! on the chess board.
//!
//! ### Why Bitboards?
//! Bitboards are the industry standard for high-performance chess engines because they allow:
//! * **Parallelism:** Perform operations on all 64 squares simultaneously using bitwise logic.
//! * **Efficiency:** Extremely low memory footprint and high cache locality.
//! * **Speed:** Modern CPUs can execute bitwise operations (AND, OR, XOR) in a single cycle.
//!
//!
//!
//! ---
//!
//! ## Board Mapping: LERF
//!
//! `ruchess` employs the **Little-Endian Rank-File (LERF)** mapping convention.
//! In this system:
//! - **Bit 0** (LSB) represents **A1**.
//! - **Bit 63** (MSB) represents **H8**.
//! - Bits progress **File-wise** (A to H) and then **Rank-wise** (1 to 8).
//!
//! ### Mapping Table
//!
//! ```text
//!   8 | 56 57 58 59 60 61 62 63
//!   7 | 48 49 50 51 52 53 54 55
//!   6 | 40 41 42 43 44 45 46 47
//!   5 | 32 33 34 35 36 37 38 39
//!   4 | 24 25 26 27 28 29 30 31
//!   3 | 16 17 18 19 20 21 22 23
//!   2 |  8  9 10 11 12 13 14 15
//!   1 |  0  1  2  3  4  5  6  7
//!     +------------------------
//!        A  B  C  D  E  F  G  H
//! ```
//!
//! ---
//!
//! ## Flexible Operations
//!
//! As a convenience, [`Bitboard`] implements `std::ops` for any type `T` that implements
//! `Into<Bitboard>`. This allows you to mix and match types like [`Square`], [`Rank`],
//! or [`File`] directly in bitwise expressions.
//!
//! ### Example: Masking a Rank
//! ```
//! # use ruchess::bitboard::Bitboard;
//! # use ruchess::square;
//! # use ruchess::rank::Rank;
//! let b = Bitboard::EMPTY;
//!
//! // You can OR a Square or a Rank directly into a Bitboard
//! let rank_4 = b | Rank::Fourth;
//! let target = rank_4 | square::A1;
//!
//! assert!(target.is_set(square::A4)); // Part of Rank 4
//! assert!(target.is_set(square::A1)); // Added explicitly
//! ```
//!
//! ## Iteration
//! [`Bitboard`] implements [`Iterator`], allowing you to easily loop over all **occupied**
//! squares. The iterator is highly optimized, using the `BLSR` (Reset Lowest Set Bit)
//! pattern to clear bits as it yields them.
//!
//! ```
//! # use ruchess::bitboard::Bitboard;
//! # use ruchess::square;
//! let bb = Bitboard::from(square::A1) | Bitboard::from(square::H8);
//!
//! for square in bb {
//!     println!("Occupied square: {:?}", square);
//! }
//! ```

use crate::{file::File, rank::Rank, square::Square};

/// A 64-bit representation of the chess board.
///
/// [`Bitboard`] is the core, irreducible data structure in `ruchess`.
/// Bitboards are used to keep track of which squares in a given position are occupied.
///
/// [`Bitboard`] uses a **Little-Endian Rank-File (LERF)** mapping, where the least significant bit (bit 0) represents the square `A1`, and the most significant bit (bit 63) represents `H8`.
///
/// The bits are ordered first by rank (1-8) and then by file (A-H).
///
/// ### Mapping Table
///
/// ```text
///   8 | 56 57 58 59 60 61 62 63
///   7 | 48 49 50 51 52 53 54 55
///   6 | 40 41 42 43 44 45 46 47
///   5 | 32 33 34 35 36 37 38 39
///   4 | 24 25 26 27 28 29 30 31
///   3 | 16 17 18 19 20 21 22 23
///   2 |  8  9 10 11 12 13 14 15
///   1 |  0  1  2  3  4  5  6  7
///     +------------------------
///        A  B  C  D  E  F  G  H
/// ```
///
/// # Example
///
/// ```
/// # use ruchess::bitboard::Bitboard;
/// # use ruchess::square;
/// assert!(Bitboard::new(1).is_set(square::A1));
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);
    pub const LIGHT: Bitboard = Bitboard(0x55AA_55AA_55AA_55AA);
    pub const DARK: Bitboard = Bitboard(0xAA55_AA55_AA55_AA55);

    /// Constructs a new [`Bitboard`] from `value`.
    ///
    /// See [`Bitboard`]
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns `true` if all bits are "off".
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// assert_eq!(Bitboard::new(0).is_empty(), true);
    /// assert_eq!(Bitboard::new(1).is_empty(), false);
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if any bits are "on".
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// assert_eq!(Bitboard::new(0).is_non_empty(), false);
    /// assert_eq!(Bitboard::new(1).is_non_empty(), true);
    #[inline]
    pub const fn is_non_empty(self) -> bool {
        self.0 != 0
    }

    /// Returns a new [`Bitboard`] with the bits specified in `other` flipped.
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// let a = Bitboard(0b1100);
    /// let b = Bitboard(0b1010);
    /// assert_eq!(a.toggle(b), Bitboard(0b0110));
    ///
    /// // toggle twice is identity
    /// assert_eq!(a.toggle(b).toggle(b), a)
    #[inline]
    pub fn toggle(self, other: impl Into<Bitboard>) -> Self {
        self ^ other
    }

    /// Flips the bits specified in `other` in place.
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// let mut a = Bitboard(0b1100);
    /// let b = Bitboard(0b1010);
    /// a.toggle_mut(b);
    /// assert_eq!(a, Bitboard(0b0110));
    ///
    /// // toggle twice is identity
    /// a.toggle_mut(b);
    /// assert_eq!(a, Bitboard(0b1100))
    #[inline]
    pub fn toggle_mut(&mut self, other: impl Into<Bitboard>) {
        *self ^= other
    }

    /// Returns a new [`Bitboard`] with the bits specified in `other` set.
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// let a = Bitboard(0b1100);
    /// let b = Bitboard(0b0011);
    /// assert_eq!(a.set(b), Bitboard(0b1111));
    ///
    /// // setting already-set bits is a no-op
    /// assert_eq!(a.set(a), a);
    /// ```
    #[inline]
    pub fn set(self, other: impl Into<Bitboard>) -> Self {
        self | other
    }

    /// Updates [`Bitboard`] in place with the bits specified in `other` set.
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// let mut a = Bitboard(0b1100);
    /// let b = Bitboard(0b0011);
    /// a.set_mut(b);
    /// assert_eq!(a, Bitboard(0b1111));
    ///
    /// // setting already-set bits is a no-op
    /// a.set_mut(a);
    /// assert_eq!(a, a);
    /// ```
    #[inline]
    pub fn set_mut(&mut self, other: impl Into<Bitboard>) {
        *self |= other
    }

    /// Returns a new [`Bitboard`] with the bits specified in `other` unset.
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// let a = Bitboard(0b1111);
    /// let b = Bitboard(0b0011);
    /// assert_eq!(a.unset(b), Bitboard(0b1100));
    ///
    /// // unsetting already-unset bits is a no-op
    /// assert_eq!(a.unset(Bitboard::EMPTY), a);
    /// ```
    #[inline]
    pub fn unset(self, other: impl Into<Bitboard>) -> Bitboard {
        let mask = other.into();
        self & !mask
    }

    /// Updates [`Bitboard`] in place with the bits specified in `other` unset.
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// let mut a = Bitboard(0b1111);
    /// let b = Bitboard(0b0011);
    /// a.unset_mut(b);
    /// assert_eq!(a, Bitboard(0b1100));
    ///
    /// // unsetting already-unset bits is a no-op
    /// a.unset_mut(Bitboard::EMPTY);
    /// assert_eq!(a, Bitboard(0b1100));
    /// ```
    #[inline]
    pub fn unset_mut(&mut self, other: impl Into<Bitboard>) {
        let mask = other.into();
        *self &= !mask
    }

    /// Returns `true` if any of the bits specified in `other` are set.
    ///
    /// # Example
    ///
    /// ```
    /// # use ruchess::bitboard::Bitboard;
    /// let a = Bitboard(0b1100);
    /// assert!(a.is_set(Bitboard(0b1000)));  // overlapping bit
    /// assert!(a.is_set(Bitboard(0b1111)));  // partial overlap also returns true
    /// assert!(!a.is_set(Bitboard(0b0011))); // no overlap
    /// assert!(!a.is_set(Bitboard::EMPTY));  // empty mask is never set
    /// ```
    #[inline]
    pub fn is_set(self, other: impl Into<Bitboard>) -> bool {
        (self & other).0 != 0
    }

    /// Returns the last square.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruchess::{Bitboard, square};
    ///
    /// assert_eq!(Bitboard::EMPTY.last(), None);
    /// assert_eq!(Bitboard::FULL.last(), Some(square::H8));
    /// ```
    #[inline]
    pub const fn last(self) -> Option<Square> {
        if let Some(index) = self.0.checked_ilog2() {
            Some(Square::new(index as u8))
        } else {
            None
        }
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        // Find the index of the least significant 1-bit (the first occupied square)
        let square_index = self.0.trailing_zeros() as u8;
        // Clear the least significant 1-bit using a standard bit-manipulation trick
        self.0 &= self.0 - 1;

        Some(Square(square_index))
    }
}

impl From<u64> for Bitboard {
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Rank> for Bitboard {
    #[inline]
    fn from(value: Rank) -> Self {
        Bitboard(Rank::MASKS[value.as_u8() as usize])
    }
}
impl From<File> for Bitboard {
    #[inline]
    fn from(value: File) -> Self {
        Bitboard(File::MASKS[value.as_u8() as usize])
    }
}

impl From<Square> for Bitboard {
    #[inline]
    fn from(value: Square) -> Self {
        Bitboard(1_u64 << value.0)
    }
}

impl TryFrom<Bitboard> for Square {
    type Error = ();

    #[inline]
    fn try_from(value: Bitboard) -> Result<Self, Self::Error> {
        if value.0.count_ones() == 1 {
            Ok(Square(value.0.trailing_zeros() as u8))
        } else {
            Err(())
        }
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<T: Into<Bitboard>> std::ops::BitAnd<T> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitand(self, rhs: T) -> Self::Output {
        Bitboard(self.0 & rhs.into().0)
    }
}
impl<T: Into<Bitboard>> std::ops::BitOr<T> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitor(self, rhs: T) -> Self::Output {
        Bitboard(self.0 | rhs.into().0)
    }
}
impl<T: Into<Bitboard>> std::ops::BitXor<T> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitxor(self, rhs: T) -> Self::Output {
        Bitboard(self.0 ^ rhs.into().0)
    }
}

impl<T: Into<Bitboard>> std::ops::Shl<T> for Bitboard {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: T) -> Self::Output {
        Bitboard(self.0 << rhs.into().0)
    }
}

impl<T: Into<Bitboard>> std::ops::Shr<T> for Bitboard {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: T) -> Self::Output {
        Bitboard(self.0 >> rhs.into().0)
    }
}

impl<T: Into<Bitboard>> std::ops::BitOrAssign<T> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: T) {
        self.0 |= rhs.into().0;
    }
}

impl<T: Into<Bitboard>> std::ops::BitXorAssign<T> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: T) {
        self.0 ^= rhs.into().0
    }
}

impl<T: Into<Bitboard>> std::ops::BitAndAssign<T> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: T) {
        self.0 &= rhs.into().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Constants ────────────────────────────────────────────────────────────

    #[test]
    fn empty_is_zero() {
        assert_eq!(Bitboard::EMPTY.0, 0);
    }

    #[test]
    fn full_is_all_ones() {
        assert_eq!(Bitboard::FULL.0, u64::MAX);
    }

    #[test]
    fn light_and_dark_partition_full() {
        assert_eq!(Bitboard::LIGHT | Bitboard::DARK, Bitboard::FULL);
        assert_eq!(Bitboard::LIGHT & Bitboard::DARK, Bitboard::EMPTY);
    }

    #[test]
    fn new_roundtrips_value() {
        assert_eq!(Bitboard::new(42).0, 42);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    fn bb() -> impl Strategy<Value = Bitboard> {
        any::<u64>().prop_map(Bitboard)
    }

    fn sq() -> impl Strategy<Value = Square> {
        (0u8..64).prop_map(Square)
    }

    proptest! {
        // ── Boolean-algebra laws ─────────────────────────────────────────────

        #[test]
        fn and_is_commutative(a in bb(), b in bb()) {
            prop_assert_eq!(a & b, b & a);
        }

        #[test]
        fn or_is_commutative(a in bb(), b in bb()) {
            prop_assert_eq!(a | b, b | a);
        }

        #[test]
        fn xor_is_commutative(a in bb(), b in bb()) {
            prop_assert_eq!(a ^ b, b ^ a);
        }

        #[test]
        fn and_is_associative(a in bb(), b in bb(), c in bb()) {
            prop_assert_eq!((a & b) & c, a & (b & c));
        }

        #[test]
        fn or_is_associative(a in bb(), b in bb(), c in bb()) {
            prop_assert_eq!((a | b) | c, a | (b | c));
        }

        #[test]
        fn xor_is_associative(a in bb(), b in bb(), c in bb()) {
            prop_assert_eq!((a ^ b) ^ c, a ^ (b ^ c));
        }

        #[test]
        fn and_distributes_over_or(a in bb(), b in bb(), c in bb()) {
            prop_assert_eq!(a & (b | c), (a & b) | (a & c));
        }

        #[test]
        fn or_distributes_over_and(a in bb(), b in bb(), c in bb()) {
            prop_assert_eq!(a | (b & c), (a | b) & (a | c));
        }

        #[test]
        fn and_is_idempotent(a in bb()) {
            prop_assert_eq!(a & a, a);
        }

        #[test]
        fn or_is_idempotent(a in bb()) {
            prop_assert_eq!(a | a, a);
        }

        #[test]
        fn xor_with_self_is_empty(a in bb()) {
            prop_assert_eq!(a ^ a, Bitboard::EMPTY);
        }

        #[test]
        fn and_empty_is_empty(a in bb()) {
            prop_assert_eq!(a & Bitboard::EMPTY, Bitboard::EMPTY);
        }

        #[test]
        fn and_full_is_identity(a in bb()) {
            prop_assert_eq!(a & Bitboard::FULL, a);
        }

        #[test]
        fn or_empty_is_identity(a in bb()) {
            prop_assert_eq!(a | Bitboard::EMPTY, a);
        }

        #[test]
        fn or_full_is_full(a in bb()) {
            prop_assert_eq!(a | Bitboard::FULL, Bitboard::FULL);
        }

        #[test]
        fn xor_empty_is_identity(a in bb()) {
            prop_assert_eq!(a ^ Bitboard::EMPTY, a);
        }

        #[test]
        fn xor_full_is_complement(a in bb()) {
            prop_assert_eq!(a ^ Bitboard::FULL, !a);
        }

        #[test]
        fn not_is_involution(a in bb()) {
            prop_assert_eq!(!!a, a);
        }

        #[test]
        fn de_morgan_and(a in bb(), b in bb()) {
            prop_assert_eq!(!(a & b), !a | !b);
        }

        #[test]
        fn de_morgan_or(a in bb(), b in bb()) {
            prop_assert_eq!(!(a | b), !a & !b);
        }

        // ── set / unset / toggle / is_set ────────────────────────────────────

        #[test]
        fn set_matches_bitor(a in bb(), b in bb()) {
            prop_assert_eq!(a.set(b), a | b);
        }

        #[test]
        fn unset_matches_bitand_not(a in bb(), b in bb()) {
            prop_assert_eq!(a.unset(b), a & !b);
        }

        #[test]
        fn toggle_matches_bitxor(a in bb(), b in bb()) {
            prop_assert_eq!(a.toggle(b), a ^ b);
        }

        #[test]
        fn set_is_idempotent(a in bb(), b in bb()) {
            prop_assert_eq!(a.set(b).set(b), a.set(b));
        }

        #[test]
        fn unset_is_idempotent(a in bb(), b in bb()) {
            prop_assert_eq!(a.unset(b).unset(b), a.unset(b));
        }

        #[test]
        fn toggle_twice_is_identity(a in bb(), b in bb()) {
            prop_assert_eq!(a.toggle(b).toggle(b), a);
        }

        #[test]
        fn set_empty_is_identity(a in bb()) {
            prop_assert_eq!(a.set(Bitboard::EMPTY), a);
        }

        #[test]
        fn unset_empty_is_identity(a in bb()) {
            prop_assert_eq!(a.unset(Bitboard::EMPTY), a);
        }

        #[test]
        fn toggle_empty_is_identity(a in bb()) {
            prop_assert_eq!(a.toggle(Bitboard::EMPTY), a);
        }

        #[test]
        fn set_full_is_full(a in bb()) {
            prop_assert_eq!(a.set(Bitboard::FULL), Bitboard::FULL);
        }

        #[test]
        fn unset_full_is_empty(a in bb()) {
            prop_assert_eq!(a.unset(Bitboard::FULL), Bitboard::EMPTY);
        }

        #[test]
        fn toggle_full_is_complement(a in bb()) {
            prop_assert_eq!(a.toggle(Bitboard::FULL), !a);
        }

        #[test]
        fn set_then_unset_clears(a in bb(), b in bb()) {
            prop_assert_eq!(a.set(b).unset(b), a.unset(b));
        }

        #[test]
        fn unset_then_set_fills(a in bb(), b in bb()) {
            prop_assert_eq!(a.unset(b).set(b), a.set(b));
        }

        #[test]
        fn is_set_iff_intersection_nonempty(a in bb(), b in bb()) {
            prop_assert_eq!(a.is_set(b), !(a & b).is_empty());
        }

        #[test]
        fn is_empty_negates_is_non_empty(a in bb()) {
            prop_assert_eq!(a.is_empty(), !a.is_non_empty());
        }

        #[test]
        fn empty_is_never_set(a in bb()) {
            prop_assert!(!Bitboard::EMPTY.is_set(a));
            prop_assert!(!a.is_set(Bitboard::EMPTY));
        }

        // ── Single-square set / unset / is_set ───────────────────────────────

        #[test]
        fn set_single_square_adds_bit(a in bb(), s in sq()) {
            let after = a.set(s);
            prop_assert!(after.is_set(s));
            prop_assert_eq!(after.0 | a.0, after.0); // never clears existing bits
        }

        #[test]
        fn unset_single_square_clears_bit(a in bb(), s in sq()) {
            let after = a.unset(s);
            prop_assert!(!after.is_set(s));
            prop_assert_eq!(after.0 & a.0, after.0); // only clears bits
        }

        #[test]
        fn is_set_matches_underlying_bit(a in bb(), s in sq()) {
            let expected = (a.0 >> s.0) & 1 == 1;
            prop_assert_eq!(a.is_set(s), expected);
        }

        // ── Shifts ───────────────────────────────────────────────────────────

        #[test]
        fn shl_zero_is_identity(a in bb()) {
            prop_assert_eq!(a << Bitboard(0), a);
        }

        #[test]
        fn shr_zero_is_identity(a in bb()) {
            prop_assert_eq!(a >> Bitboard(0), a);
        }

        #[test]
        fn shl_then_shr_clears_low_bits(a in bb(), n in 0u64..64) {
            let shifted = (a << Bitboard(n)) >> Bitboard(n);
            let expected = Bitboard((a.0 << n) >> n);
            prop_assert_eq!(shifted, expected);
        }

        // ── BitOrAssign ──────────────────────────────────────────────────────

        #[test]
        fn bit_or_assign_matches_bit_or(a in bb(), b in bb()) {
            let mut x = a;
            x |= b;
            prop_assert_eq!(x, a | b);
        }

        // ── Conversions ──────────────────────────────────────────────────────

        #[test]
        fn from_u64_roundtrip(v in any::<u64>()) {
            prop_assert_eq!(Bitboard::from(v).0, v);
        }

        #[test]
        fn from_square_has_one_bit(s in sq()) {
            let bb = Bitboard::from(s);
            prop_assert_eq!(bb.0.count_ones(), 1);
            prop_assert_eq!(bb.0, 1u64 << s.0);
        }

        #[test]
        fn square_try_from_singleton_roundtrip(s in sq()) {
            let bb = Bitboard::from(s);
            prop_assert_eq!(Square::try_from(bb), Ok(s));
        }

        #[test]
        fn square_try_from_fails_for_non_singletons(a in bb()) {
            prop_assume!(a.0.count_ones() != 1);
            prop_assert_eq!(Square::try_from(a), Err(()));
        }

        #[test]
        fn from_rank_has_eight_bits(r in 0u32..8) {
            let bb = Bitboard::from(Rank::new(r));
            prop_assert_eq!(bb.0.count_ones(), 8);
        }

        #[test]
        fn from_file_has_eight_bits(f in 0u32..8) {
            let bb = Bitboard::from(File::new(f));
            prop_assert_eq!(bb.0.count_ones(), 8);
        }

        #[test]
        fn square_belongs_to_its_rank_and_file(s in sq()) {
            let r = Bitboard::from(s.rank());
            let f = Bitboard::from(s.file());
            prop_assert!(r.is_set(s));
            prop_assert!(f.is_set(s));
            prop_assert_eq!(r & f, Bitboard::from(s));
        }

        // ── Iterator ─────────────────────────────────────────────────────────

        #[test]
        fn iter_yields_count_ones_items(a in bb()) {
            let squares: Vec<Square> = a.into_iter().collect();
            prop_assert_eq!(squares.len() as u32, a.0.count_ones());
        }

        #[test]
        fn iter_yields_ascending_squares(a in bb()) {
            let squares: Vec<Square> = a.into_iter().collect();
            for w in squares.windows(2) {
                prop_assert!(w[0].0 < w[1].0);
            }
        }

        #[test]
        fn iter_yields_only_set_bits(a in bb()) {
            for s in a {
                prop_assert!(a.is_set(s));
            }
        }

        #[test]
        fn iter_reconstructs_original(a in bb()) {
            let mut acc = Bitboard::EMPTY;
            for s in a {
                acc |= Bitboard::from(s);
            }
            prop_assert_eq!(acc, a);
        }

        #[test]
        fn empty_iter_yields_nothing(()in Just(())) {
            prop_assert_eq!(Bitboard::EMPTY.into_iter().count(), 0);
        }

    }
}
