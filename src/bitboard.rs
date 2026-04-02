use std::fmt::{Debug, Display};

use crate::square::Square;

#[derive(Clone, Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn new() -> Self {
        Bitboard(0)
    }

    pub fn contains(&self, square: &Square) -> bool {
        let mask = 1u64 << square.0;
        (self.0 & mask) != EMPTY.0
    }

    pub fn toggle_pos(&self, square: &Square) -> Bitboard {
        let mask = 1u64 << square.0;
        Bitboard(self.0 ^ mask)
    }

    pub fn set_pos(&self, square: &Square) -> Bitboard {
        let mask = 1u64 << square.0;
        Bitboard(self.0 | mask)
    }

    pub fn clear_pos(&self, square: &Square) -> Bitboard {
        let mask: u64 = 1u64 << square.0;
        Bitboard(self.0 & !mask)
    }

    pub fn move_(&self, from: &Square, to: &Square) -> Bitboard {
        self.clear_pos(from).set_pos(to)
    }

    pub fn as_grid_str(&self) -> String {
        let mut buffer = String::with_capacity(136);

        for rank in (0..8).rev() {
            for file in 0..8 {
                let position = rank * 8 + file;
                let mask = 1u64 << position;

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
impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
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
// // E4, D4, E5, D5
// pub const CENTER: Bitboard = Bitboard(0x1818000000);
//
// pub const FIRST_RANK: Bitboard = Bitboard(0xff);
// pub const LAST_RANK: Bitboard = Bitboard(0xff << 56);
//
// // all light squares
// pub const LIGHT_SQUARES: Bitboard = Bitboard(0x55aa55aa55aa55aa);
// // all dark squares
// pub const DARK_SQUARES: Bitboard = Bitboard(0xaa55aa55aa55aa55);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_bitwise_and_correct() {
        assert_eq!(Bitboard(2).0 & Bitboard(1).0, Bitboard(2 & 1).0)
    }
}
