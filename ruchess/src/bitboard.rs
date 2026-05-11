use crate::square::{File, Rank, Square};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    pub fn is_non_empty(self) -> bool {
        self != Self::EMPTY
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
    pub fn toggle(self, other: impl Into<Bitboard>) -> Self {
        self ^ other
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
    pub fn set(self, other: impl Into<Bitboard>) -> Self {
        self | other
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
    pub fn unset(self, other: impl Into<Bitboard>) -> Bitboard {
        let mask = other.into();
        self & !mask
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
    pub fn is_set(self, other: impl Into<Bitboard>) -> bool {
        (self & other) != Self::EMPTY
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Rank> for Bitboard {
    fn from(value: Rank) -> Self {
        Bitboard(Rank::MASKS[value.as_u8() as usize])
    }
}
impl From<File> for Bitboard {
    fn from(value: File) -> Self {
        Bitboard(File::MASKS[value.as_u8() as usize])
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Bitboard(1_u64 << value.0)
    }
}

impl TryFrom<Bitboard> for Square {
    type Error = ();

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
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<T: Into<Bitboard>> std::ops::BitAnd<T> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: T) -> Self::Output {
        Bitboard(self.0 & rhs.into().0)
    }
}
impl<T: Into<Bitboard>> std::ops::BitOr<T> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: T) -> Self::Output {
        Bitboard(self.0 | rhs.into().0)
    }
}
impl<T: Into<Bitboard>> std::ops::BitXor<T> for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: T) -> Self::Output {
        Bitboard(self.0 ^ rhs.into().0)
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
    fn new_roundtrips_value() {
        assert_eq!(Bitboard::new(42).0, 42);
    }

    // ── set / unset / toggle / is_set ────────────────────────────────────────

    #[test]
    fn set_adds_bits() {
        let a = Bitboard(0b0011);
        let b = Bitboard(0b1100);
        assert_eq!(a.set(b), Bitboard(0b1111));
    }

    #[test]
    fn set_is_idempotent() {
        let a = Bitboard(0b1111);
        assert_eq!(a.set(Bitboard(0b1010)), Bitboard(0b1111));
    }

    #[test]
    fn unset_clears_bits() {
        let a = Bitboard(0b1111);
        let b = Bitboard(0b0101);
        assert_eq!(a.unset(b), Bitboard(0b1010));
    }

    #[test]
    fn unset_with_non_overlapping_mask_is_noop() {
        let a = Bitboard(0b1010);
        assert_eq!(a.unset(Bitboard(0b0101)), Bitboard(0b1010));
    }

    #[test]
    fn unset_all_yields_empty() {
        assert_eq!(Bitboard::FULL.unset(Bitboard::FULL), Bitboard::EMPTY);
    }

    #[test]
    fn toggle_flips_bits() {
        let a = Bitboard(0b1100);
        let b = Bitboard(0b1010);
        assert_eq!(a.toggle(b), Bitboard(0b0110));
    }

    #[test]
    fn toggle_twice_is_identity() {
        let a = Bitboard(0b1010_1010);
        let mask = Bitboard(0b1111_0000);
        assert_eq!(a.toggle(mask).toggle(mask), a);
    }

    #[test]
    fn is_set_returns_true_when_bit_present() {
        assert!(Bitboard(0b1010).is_set(Bitboard(0b0010)));
    }

    #[test]
    fn is_set_returns_false_when_bit_absent() {
        assert!(!Bitboard(0b1010).is_set(Bitboard(0b0101)));
    }

    #[test]
    fn is_set_on_empty_is_always_false() {
        assert!(!Bitboard::EMPTY.is_set(Bitboard::FULL));
    }

    // ── Not ─────────────────────────────────────────────────────────────────

    #[test]
    fn not_inverts_all_bits() {
        assert_eq!(!Bitboard::EMPTY, Bitboard::FULL);
        assert_eq!(!Bitboard::FULL, Bitboard::EMPTY);
    }

    #[test]
    fn not_inverts_partial_bits() {
        assert_eq!(
            !Bitboard(0x00FF_FFFF_FFFF_FFFF),
            Bitboard(0xFF00_0000_0000_0000)
        );
    }

    #[test]
    fn not_twice_is_identity() {
        let b = Bitboard(0xDEAD_BEEF_CAFE_1234);
        assert_eq!(!!b, b);
    }

    // ── BitAnd / BitOr / BitXor ──────────────────────────────────────────────

    #[test]
    fn bitand_intersects() {
        assert_eq!(Bitboard(0b1100) & Bitboard(0b1010), Bitboard(0b1000));
    }

    #[test]
    fn bitand_with_empty_yields_empty() {
        assert_eq!(Bitboard::FULL & Bitboard::EMPTY, Bitboard::EMPTY);
    }

    #[test]
    fn bitand_with_full_is_identity() {
        let b = Bitboard(0xABCD);
        assert_eq!(b & Bitboard::FULL, b);
    }

    #[test]
    fn bitor_unions() {
        assert_eq!(Bitboard(0b1100) | Bitboard(0b0011), Bitboard(0b1111));
    }

    #[test]
    fn bitor_with_empty_is_identity() {
        let b = Bitboard(0xABCD);
        assert_eq!(b | Bitboard::EMPTY, b);
    }

    #[test]
    fn bitor_with_full_yields_full() {
        assert_eq!(Bitboard(0x1234) | Bitboard::FULL, Bitboard::FULL);
    }

    #[test]
    fn bitxor_differs() {
        assert_eq!(Bitboard(0b1100) ^ Bitboard(0b1010), Bitboard(0b0110));
    }

    #[test]
    fn bitxor_self_yields_empty() {
        let b = Bitboard(0xDEAD_BEEF);
        assert_eq!(b ^ b, Bitboard::EMPTY);
    }

    #[test]
    fn bitxor_with_empty_is_identity() {
        let b = Bitboard(0xABCD);
        assert_eq!(b ^ Bitboard::EMPTY, b);
    }

    // ── From conversions ─────────────────────────────────────────────────────

    #[test]
    fn from_u64_roundtrips() {
        let val: u64 = 0x0123_4567_89AB_CDEF;
        assert_eq!(Bitboard::from(val).0, val);
    }

    #[test]
    fn from_square_sets_exactly_one_bit() {
        for i in 0u8..64 {
            let sq = Square(i);
            let bb = Bitboard::from(sq);
            assert_eq!(bb.0.count_ones(), 1);
            assert_eq!(bb.0, 1_u64 << i);
        }
    }

    // ── Derived traits ───────────────────────────────────────────────────────

    #[test]
    fn eq_and_ne() {
        assert_eq!(Bitboard(7), Bitboard(7));
        assert_ne!(Bitboard(7), Bitboard(8));
    }

    #[test]
    fn copy_is_independent() {
        let a = Bitboard(0xFF);
        let b = a; // Copy
        assert_eq!(a, b);
    }

    #[test]
    fn debug_contains_inner_value() {
        let s = format!("{:?}", Bitboard(255));
        assert!(s.contains("255"));
    }

    #[test]
    fn hash_equal_boards_have_equal_hashes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        Bitboard(42).hash(&mut h1);
        Bitboard(42).hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }
}
