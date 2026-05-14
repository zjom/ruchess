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

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rank={}", self.as_str())
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rank={}", self.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseRankError(pub String);
impl std::fmt::Display for ParseRankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "invalid rank: `{}`", self.0)
    }
}
impl std::error::Error for ParseRankError {}

impl std::str::FromStr for Rank {
    type Err = ParseRankError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.as_bytes();
        if p.len() != 1 {
            return Err(ParseRankError(s.to_string()));
        }

        Self::from_byte(p[0]).ok_or(ParseRankError(s.to_string()))
    }
}
