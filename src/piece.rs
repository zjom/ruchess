#[derive(Debug)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
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
