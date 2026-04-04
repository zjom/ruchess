use std::fmt::Display;

pub const NUM_COLORS: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub const ALL: [Color; 2] = [Color::White, Color::Black];
    pub fn opponent(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::White => "white",
            Self::Black => "black",
        };
        write!(f, "{}", s)
    }
}

impl std::ops::Not for Color {
    type Output = Color;
    fn not(self) -> Self::Output {
        self.opponent()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ByColor<T>
where
    T: Copy,
{
    pub black: T,
    pub white: T,
}

impl<T> ByColor<T>
where
    T: Copy,
{
    pub fn get(&self, c: Color) -> &T {
        match c {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

    pub fn find<F>(&self, mut predicate: F) -> Option<Color>
    where
        F: FnMut(&T) -> bool,
    {
        if predicate(&self.white) {
            Some(Color::White)
        } else if predicate(&self.black) {
            Some(Color::Black)
        } else {
            None
        }
    }

    pub fn update<F>(&mut self, c: Color, f: F)
    where
        F: FnOnce(&T) -> T,
    {
        match c {
            Color::White => self.white = f(&self.white),
            Color::Black => self.black = f(&self.black),
        }
    }
}
