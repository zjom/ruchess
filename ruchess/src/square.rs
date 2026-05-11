#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    #[inline]
    pub const fn new(idx: u32) -> File {
        assert!(idx < 8);
        unsafe { File::new_unchecked(idx) }
    }

    /// # Safety
    ///
    /// Function must be called with an index < 8
    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> File {
        debug_assert!(index < 8);
        unsafe { std::mem::transmute(index as u8) }
    }

    pub const MASKS: [u64; 8] = [
        0x0101_0101_0101_0101, // File A
        0x0202_0202_0202_0202, // File B
        0x0404_0404_0404_0404, // File C
        0x0808_0808_0808_0808, // File D
        0x1010_1010_1010_1010, // File E
        0x2020_2020_2020_2020, // File F
        0x4040_4040_4040_4040, // File G
        0x8080_8080_8080_8080, // File H
    ];

    #[inline]
    pub const fn as_u8(self) -> u8 {
        // Safety: self is repr u8
        unsafe { std::mem::transmute(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Rank {
    First = 0,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}
impl Rank {
    #[inline]
    pub const fn new(idx: u32) -> Rank {
        assert!(idx < 8);
        unsafe { Rank::new_unchecked(idx) }
    }

    /// # Safety
    ///
    /// Function must be called with an index < 8
    #[inline]
    pub const unsafe fn new_unchecked(index: u32) -> Rank {
        debug_assert!(index < 8);
        unsafe { std::mem::transmute(index as u8) }
    }

    pub const MASKS: [u64; 8] = [
        0x0000_0000_0000_00FF, // Rank 1
        0x0000_0000_0000_FF00, // Rank 2
        0x0000_0000_00FF_0000, // Rank 3
        0x0000_0000_FF00_0000, // Rank 4
        0x0000_00FF_0000_0000, // Rank 5
        0x0000_FF00_0000_0000, // Rank 6
        0x00FF_0000_0000_0000, // Rank 7
        0xFF00_0000_0000_0000, // Rank 8
    ];

    #[inline]
    pub const fn as_u8(self) -> u8 {
        // Safety: self is repr u8
        unsafe { std::mem::transmute(self) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Square(pub u8);
impl Square {
    pub const fn new(idx: u32) -> Square {
        assert!(idx < 64);
        Square(idx as u8)
    }
}
//
// impl std::ops::Not for Square {
//     type Output = Self;
//     fn not(self) -> Self::Output {
//         Self(!self.0)
//     }
// }
