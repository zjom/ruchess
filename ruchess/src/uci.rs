use std::{error::Error, fmt::Display, str::FromStr};

use crate::{
    mve::Move,
    role::{ParseRoleError, PromotableRole},
    square::{ParseSquareError, Square},
};

#[derive(Debug, Clone, Copy)]
pub struct Uci {
    pub orig: Square,
    pub dest: Square,
    pub promotion: Option<PromotableRole>,
}

impl Uci {
    pub fn from_move(m: Move) -> Self {
        Self {
            orig: m.orig,
            dest: m.dest,
            promotion: m.promotion,
        }
    }

    pub fn uci(&self) -> String {
        match self.promotion {
            Some(r) => format!("{}{}{}", self.orig, self.dest, r.as_ascii()),
            None => format!("{}{}", self.orig, self.dest,),
        }
    }
}

impl From<Move> for Uci {
    fn from(value: Move) -> Self {
        Self::from_move(value)
    }
}

impl std::fmt::Display for Uci {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.uci())
    }
}

impl FromStr for Uci {
    type Err = UciFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            4 | 5 => {
                let orig = Square::from_str(&s[0..2])?;
                let dest = Square::from_str(&s[2..4])?;
                let promotion = match s.len() {
                    5 => Some(PromotableRole::from_str(&s[4..5])?),
                    _ => None,
                };

                Ok(Self {
                    orig,
                    dest,
                    promotion,
                })
            }
            _ => Err(UciFormatError::InvalidLength(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum UciFormatError {
    InvalidLength(String),
    ParseSquareError(ParseSquareError),
    ParsePromotionError(ParseRoleError),
}

impl Display for UciFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength(input) => writeln!(f, "invalid uci format: {}", input),
            Self::ParseSquareError(error) => writeln!(f, "invalid uci format: {}", error),
            Self::ParsePromotionError(error) => writeln!(f, "invalid uci format: {}", error),
        }
    }
}
impl Error for UciFormatError {}

impl From<ParseSquareError> for UciFormatError {
    fn from(value: ParseSquareError) -> Self {
        Self::ParseSquareError(value)
    }
}
impl From<ParseRoleError> for UciFormatError {
    fn from(value: ParseRoleError) -> Self {
        Self::ParsePromotionError(value)
    }
}
