use std::error::Error;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::color::Color;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

    pub fn as_char(&self) -> char {
        match self {
            File::A => 'A',
            File::B => 'B',
            File::C => 'C',
            File::D => 'D',
            File::E => 'E',
            File::F => 'F',
            File::G => 'G',
            File::H => 'H',
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            File::A => "A",
            File::B => "B",
            File::C => "C",
            File::D => "D",
            File::E => "E",
            File::F => "F",
            File::G => "G",
            File::H => "H",
        }
    }

    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            b'A' | b'a' => Some(File::A),
            b'B' | b'b' => Some(File::B),
            b'C' | b'c' => Some(File::C),
            b'D' | b'd' => Some(File::D),
            b'E' | b'e' => Some(File::E),
            b'F' | b'f' => Some(File::F),
            b'G' | b'g' => Some(File::G),
            b'H' | b'h' => Some(File::H),
            _ => None,
        }
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File={}", self.as_str())
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File={}", self.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseFileError(pub String);
impl Display for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "invalid file: `{}`", self.0)
    }
}
impl Error for ParseFileError {}
impl FromStr for File {
    type Err = ParseFileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.as_bytes();
        if p.len() != 1 {
            return Err(ParseFileError(s.to_string()));
        }

        Self::from_byte(p[0]).ok_or(ParseFileError(s.to_string()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
    pub fn as_char(&self) -> char {
        match self {
            Rank::First => '1',
            Rank::Second => '2',
            Rank::Third => '3',
            Rank::Fourth => '4',
            Rank::Fifth => '5',
            Rank::Sixth => '6',
            Rank::Seventh => '7',
            Rank::Eighth => '8',
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::First => "1",
            Rank::Second => "2",
            Rank::Third => "3",
            Rank::Fourth => "4",
            Rank::Fifth => "5",
            Rank::Sixth => "6",
            Rank::Seventh => "7",
            Rank::Eighth => "8",
        }
    }

    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            b'1' => Some(Rank::First),
            b'2' => Some(Rank::Second),
            b'3' => Some(Rank::Third),
            b'4' => Some(Rank::Fourth),
            b'5' => Some(Rank::Fifth),
            b'6' => Some(Rank::Sixth),
            b'7' => Some(Rank::Seventh),
            b'8' => Some(Rank::Eighth),
            _ => None,
        }
    }
}

impl Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rank={}", self.as_str())
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rank={}", self.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseRankError(pub String);
impl Display for ParseRankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "invalid rank: `{}`", self.0)
    }
}
impl Error for ParseRankError {}
impl FromStr for Rank {
    type Err = ParseRankError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.as_bytes();
        if p.len() != 1 {
            return Err(ParseRankError(s.to_string()));
        }

        Self::from_byte(p[0]).ok_or(ParseRankError(s.to_string()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Square(pub u8);
impl Square {
    pub const fn new(idx: u32) -> Square {
        assert!(idx < 64);
        Square(idx as u8)
    }
    pub const fn from_file_and_rank(file: File, rank: Rank) -> Self {
        Self::new((rank.as_u8() * 8 + file.as_u8()) as u32)
    }

    pub fn rank(&self) -> Rank {
        Rank::new((self.0 / 8) as u32)
    }

    pub fn file(&self) -> File {
        File::new((self.0 % 8) as u32)
    }

    pub fn ydist(self, other: Square) -> u8 {
        let (a, b) = (self.rank().as_u8(), other.rank().as_u8());

        a.wrapping_sub(b).min(b.wrapping_sub(a))
    }

    pub fn prev_rank(self, color: Color) -> Option<Square> {
        match color {
            Color::White => {
                if self.rank() == Rank::First {
                    None
                } else {
                    Some(Square::from_file_and_rank(
                        self.file(),
                        Rank::new((self.rank().as_u8() - 1) as u32),
                    ))
                }
            }
            Color::Black => {
                if self.rank() == Rank::Eighth {
                    None
                } else {
                    Some(Square::from_file_and_rank(
                        self.file(),
                        Rank::new((self.rank().as_u8() + 1) as u32),
                    ))
                }
            }
        }
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.file().as_str();
        let rank = self.rank().as_str();
        write!(f, "{}{}", file, rank)
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.file().as_str();
        let rank = self.rank().as_str();
        write!(f, "{}{}", file, rank)
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseSquareError(pub String);
impl Display for ParseSquareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "invalid square notation: `{}`", self.0)
    }
}
impl Error for ParseSquareError {}

impl FromStr for Square {
    type Err = ParseSquareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.trim().to_ascii_uppercase();
        let p = norm.as_bytes();
        match p {
            [b'A'..=b'H', b'1'..=b'8'] => {
                let rank = p[0] - b'A';
                let file = p[1] - b'1';
                Ok(Square(file * 8 + rank))
            }
            _ => Err(ParseSquareError(s.into())),
        }
    }
}

pub const A1: Square = Square(0);
pub const B1: Square = Square(1);
pub const C1: Square = Square(2);
pub const D1: Square = Square(3);
pub const E1: Square = Square(4);
pub const F1: Square = Square(5);
pub const G1: Square = Square(6);
pub const H1: Square = Square(7);
pub const A2: Square = Square(8);
pub const B2: Square = Square(9);
pub const C2: Square = Square(10);
pub const D2: Square = Square(11);
pub const E2: Square = Square(12);
pub const F2: Square = Square(13);
pub const G2: Square = Square(14);
pub const H2: Square = Square(15);
pub const A3: Square = Square(16);
pub const B3: Square = Square(17);
pub const C3: Square = Square(18);
pub const D3: Square = Square(19);
pub const E3: Square = Square(20);
pub const F3: Square = Square(21);
pub const G3: Square = Square(22);
pub const H3: Square = Square(23);
pub const A4: Square = Square(24);
pub const B4: Square = Square(25);
pub const C4: Square = Square(26);
pub const D4: Square = Square(27);
pub const E4: Square = Square(28);
pub const F4: Square = Square(29);
pub const G4: Square = Square(30);
pub const H4: Square = Square(31);
pub const A5: Square = Square(32);
pub const B5: Square = Square(33);
pub const C5: Square = Square(34);
pub const D5: Square = Square(35);
pub const E5: Square = Square(36);
pub const F5: Square = Square(37);
pub const G5: Square = Square(38);
pub const H5: Square = Square(39);
pub const A6: Square = Square(40);
pub const B6: Square = Square(41);
pub const C6: Square = Square(42);
pub const D6: Square = Square(43);
pub const E6: Square = Square(44);
pub const F6: Square = Square(45);
pub const G6: Square = Square(46);
pub const H6: Square = Square(47);
pub const A7: Square = Square(48);
pub const B7: Square = Square(49);
pub const C7: Square = Square(50);
pub const D7: Square = Square(51);
pub const E7: Square = Square(52);
pub const F7: Square = Square(53);
pub const G7: Square = Square(54);
pub const H7: Square = Square(55);
pub const A8: Square = Square(56);
pub const B8: Square = Square(57);
pub const C8: Square = Square(58);
pub const D8: Square = Square(59);
pub const E8: Square = Square(60);
pub const F8: Square = Square(61);
pub const G8: Square = Square(62);
pub const H8: Square = Square(63);
