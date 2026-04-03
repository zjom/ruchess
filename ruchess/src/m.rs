use std::fmt::Display;
use std::str::FromStr;

use crate::error::ParseSquareError;
use crate::Square;

pub struct Move {
    pub from: Square,
    pub to: Square,
}

impl FromStr for Move {
    type Err = ParseSquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() < 4 {
            return Err(ParseSquareError(s.into()));
        }
        let from = s[..2].parse::<Square>()?;
        let to = s[2..4].parse::<Square>()?;
        Ok(Move { from, to })
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}
