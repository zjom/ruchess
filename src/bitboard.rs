use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use crate::square::Square;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn contains(&self, square: &Square) -> bool {
        let mask = 1u64 << square.0;
        (self.0 & mask) != 0
    }

    pub fn toggle(&self, square: &Square) -> Bitboard {
        let mask = 1u64 << square.0;
        Bitboard(self.0 ^ mask)
    }

    pub fn set(&self, square: &Square) -> Bitboard {
        let mask = 1u64 << square.0;
        Bitboard(self.0 | mask)
    }

    pub fn clear(&self, square: &Square) -> Bitboard {
        let mask: u64 = 1u64 << square.0;
        Bitboard(self.0 & !mask)
    }

    pub fn move_(&self, from: &Square, to: &Square) -> Bitboard {
        self.clear(from).set(to)
    }

    pub fn as_grid(&self) -> [[bool; 8]; 8] {
        let mut grid = [[false; 8]; 8];
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mask = 1u64 << (rank * 8 + file);
                grid[rank - 8][file] = (self.0 & mask) != 0
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

impl Deref for Bitboard {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

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

pub const EMPTY: Bitboard = Bitboard(0);
pub const ALL: Bitboard = Bitboard(u64::MAX);

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
        assert_eq!(Bitboard::default().set(&sq), Bitboard(1));
        assert_eq!(Bitboard(1).clear(&sq), Bitboard::default());
    }
}
