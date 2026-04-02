pub const NUM_PIECES: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    pub const ALL: [Piece; NUM_PIECES] = [
        Piece::Pawn,
        Piece::Rook,
        Piece::Bishop,
        Piece::Knight,
        Piece::Queen,
        Piece::King,
    ];
}

impl std::fmt::Display for Piece {
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
