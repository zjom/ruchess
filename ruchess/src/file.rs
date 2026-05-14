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

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File={}", self.as_str())
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File={}", self.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseFileError(pub String);
impl std::fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "invalid file: `{}`", self.0)
    }
}
impl std::error::Error for ParseFileError {}
impl std::str::FromStr for File {
    type Err = ParseFileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.as_bytes();
        if p.len() != 1 {
            return Err(ParseFileError(s.to_string()));
        }

        Self::from_byte(p[0]).ok_or(ParseFileError(s.to_string()))
    }
}
