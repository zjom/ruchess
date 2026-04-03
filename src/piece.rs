use crate::{Bitboard, color::Color};

pub const NUM_ROLES: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Role {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

impl Role {
    pub const ALL: [Role; NUM_ROLES] = [
        Role::Pawn,
        Role::Rook,
        Role::Bishop,
        Role::Knight,
        Role::Queen,
        Role::King,
    ];

    pub fn of(self, color: Color) -> Piece {
        Piece(self, color)
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pawn => "P",
            Self::Rook => "R",
            Self::Knight => "N",
            Self::Bishop => "B",
            Self::Queen => "Q",
            Self::King => "K",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ByRole<T>
where
    T: Copy,
{
    pub pawn: T,
    pub rook: T,
    pub knight: T,
    pub bishop: T,
    pub queen: T,
    pub king: T,
}

impl<T> ByRole<T>
where
    T: Copy,
{
    pub fn get(&self, r: Role) -> &T {
        match r {
            Role::Pawn => &self.pawn,
            Role::Rook => &self.rook,
            Role::Bishop => &self.bishop,
            Role::Knight => &self.knight,
            Role::Queen => &self.queen,
            Role::King => &self.king,
        }
    }
    pub fn update<F>(&mut self, r: Role, f: F) -> &Self
    where
        F: FnOnce(&T) -> T,
    {
        match r {
            Role::Pawn => self.pawn = f(&self.pawn),
            Role::Rook => self.rook = f(&self.rook),
            Role::Bishop => self.bishop = f(&self.bishop),

            Role::Knight => self.knight = f(&self.knight),
            Role::Queen => self.queen = f(&self.queen),
            Role::King => self.king = f(&self.king),
        };
        self
    }

    pub fn find<F>(&self, mut predicate: F) -> Option<Role>
    where
        F: FnMut(&T) -> bool,
    {
        if predicate(&self.pawn) {
            Some(Role::Pawn)
        } else if predicate(&self.rook) {
            Some(Role::Rook)
        } else if predicate(&self.knight) {
            Some(Role::Knight)
        } else if predicate(&self.bishop) {
            Some(Role::Bishop)
        } else if predicate(&self.queen) {
            Some(Role::Queen)
        } else if predicate(&self.king) {
            Some(Role::King)
        } else {
            None
        }
    }

    pub fn find_or_king<F>(&self, mut predicate: F) -> Role
    where
        F: FnMut(&T) -> bool,
    {
        if predicate(&self.pawn) {
            Role::Pawn
        } else if predicate(&self.rook) {
            Role::Rook
        } else if predicate(&self.knight) {
            Role::Knight
        } else if predicate(&self.bishop) {
            Role::Bishop
        } else if predicate(&self.queen) {
            Role::Queen
        } else {
            Role::King
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece(pub Role, pub Color);

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Piece(Role::Pawn, Color::White) => "♙",
            Piece(Role::Knight, Color::White) => "♘",
            Piece(Role::Bishop, Color::White) => "♗",
            Piece(Role::Rook, Color::White) => "♖",
            Piece(Role::Queen, Color::White) => "♕",
            Piece(Role::King, Color::White) => "♔",
            Piece(Role::Pawn, Color::Black) => "♟",
            Piece(Role::Knight, Color::Black) => "♞",
            Piece(Role::Bishop, Color::Black) => "♝",
            Piece(Role::Rook, Color::Black) => "♜",
            Piece(Role::Queen, Color::Black) => "♛",
            Piece(Role::King, Color::Black) => "♚",
        };
        write!(f, "{symbol}")
    }
}
