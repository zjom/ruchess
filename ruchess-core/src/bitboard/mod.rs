mod attacks;
mod bitboard_ext;
mod magic;

pub use attacks::Attacks;

use crate::square::Square;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const ALL: Bitboard = Bitboard(u64::MAX);

    pub fn contains(self, other: impl Into<Bitboard>) -> bool {
        let mask = Bitboard(1u64) << other;
        (self & mask) != Bitboard::EMPTY
    }

    pub fn toggle(self, square: impl Into<Bitboard>) -> Bitboard {
        let mask = Bitboard(1u64) << square;
        self ^ mask
    }

    pub fn set(self, square: impl Into<Bitboard>) -> Bitboard {
        let mask = Bitboard(1u64) << square;
        self | mask
    }

    pub fn clear(self, square: impl Into<Bitboard>) -> Bitboard {
        let mask = Bitboard(1u64) << square;
        self & !mask
    }

    pub fn move_(self, from: impl Into<Bitboard>, to: impl Into<Bitboard>) -> Bitboard {
        self.clear(from).set(to)
    }

    pub fn intersect(self, other: impl Into<Bitboard>) -> Bitboard {
        self & other
    }

    pub fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: Fn(B, Bitboard) -> B,
    {
        let mut b = self.0;
        let mut result = init;
        while b != 0 {
            let lsb = b & b.wrapping_neg(); // isolate lowest set bit
            result = f(result, Bitboard(lsb));
            b &= b - 1; // clear lowest set bit
        }
        result
    }

    /// maps itself to Square
    /// bitboard must only contain 1 bit that is on
    pub fn as_square(self) -> Square {
        debug_assert_eq!(self.0.count_ones(), 1);
        Square(self.0.trailing_zeros() as u8)
    }

    pub fn as_grid(self) -> [[bool; 8]; 8] {
        let mut grid = [[false; 8]; 8];
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mask = 1u64 << (rank * 8 + file);
                grid[7 - rank][file] = (self & mask) != Bitboard::EMPTY
            }
        }
        grid
    }

    pub fn as_grid_str(&self) -> String {
        let mut buffer = String::with_capacity(136);
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mask = 1u64 << (rank * 8 + file);
                if (self.0 & mask) != 0 {
                    buffer.push_str("1 ");
                } else {
                    buffer.push_str(". ");
                }
            }
            buffer.push('\n');
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_and() {
        assert_eq!(Bitboard(2) & Bitboard(1), Bitboard(2 & 1));
    }

    #[test]
    fn set_and_clear_pos() {
        let sq = Square(0);
        assert_eq!(Bitboard::default().set(sq), Bitboard(1));
        assert_eq!(Bitboard(1).clear(sq), Bitboard::default());
    }
}
