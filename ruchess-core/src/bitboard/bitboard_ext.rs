use crate::Bitboard;
use std::fmt::{Debug, Display};

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_grid_str())
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

impl From<Bitboard> for u64 {
    fn from(bb: Bitboard) -> Self {
        bb.0
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl<T: Into<Bitboard>> std::ops::BitAnd<T> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: T) -> Bitboard {
        Bitboard(self.0 & rhs.into().0)
    }
}

impl<T: Into<Bitboard>> std::ops::BitOr<T> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: T) -> Bitboard {
        Bitboard(self.0 | rhs.into().0)
    }
}

impl<T: Into<Bitboard>> std::ops::BitXor<T> for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: T) -> Self::Output {
        Bitboard(self.0 ^ rhs.into().0)
    }
}

impl<T: Into<Bitboard>> std::ops::Shl<T> for Bitboard {
    type Output = Bitboard;
    fn shl(self, rhs: T) -> Self::Output {
        Bitboard(self.0 << rhs.into().0)
    }
}
impl<T: Into<Bitboard>> std::ops::Shr<T> for Bitboard {
    type Output = Bitboard;
    fn shr(self, rhs: T) -> Self::Output {
        Bitboard(self.0 >> rhs.into().0)
    }
}
