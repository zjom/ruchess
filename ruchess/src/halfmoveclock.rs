/// The halfmove clock specifies a decimal number of half moves with respect to the 50 move draw rule.
/// It is reset to zero after a capture or a pawn move and incremented otherwise.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct HalfMoveClock(u8);
impl HalfMoveClock {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn incr(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl From<HalfMoveClock> for u8 {
    fn from(value: HalfMoveClock) -> Self {
        value.get()
    }
}
