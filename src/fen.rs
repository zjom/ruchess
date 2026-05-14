//! # FEN parsing
//!
//! [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
//! is a one-line ASCII description of a chess position. [`parse`] turns a FEN
//! string into a [`Position`].
//!
//! A FEN record has six fields, separated by whitespace:
//!
//! 1. **Piece placement**: 8 ranks from rank 8 down to rank 1, separated by
//!    `/`. Pieces use the standard FEN convention — uppercase for White
//!    (`KQRBNP`), lowercase for Black (`kqrbnp`). Digits `1`–`8` count empty
//!    squares.
//! 2. **Active color**: `w` or `b`.
//! 3. **Castling rights**: any subset of `KQkq`, or `-` for none.
//! 4. **En passant target square**: the square *behind* the pawn that just
//!    pushed two ranks, or `-`. The parser reconstructs the implied prior move
//!    so [`Position::enpassant_square`] reports the same target.
//! 5. **Halfmove clock**: half-moves since the last pawn move or capture.
//! 6. **Fullmove number**: number of full moves in game;
//!
//! Fields 5 and 6 are optional; missing values default to `0` and `1`.
//!
//! ```
//! # use ruchess::fen;
//! # use ruchess::position::Position;
//! let start = fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
//! assert_eq!(start, Position::new());
//! ```

use std::{error::Error, fmt::Display};

use crate::{
    board::Board,
    castles::Castles,
    color::{Color, ParseColorError},
    file::File,
    halfmoveclock::HalfMoveClock,
    hash::PositionHash,
    history::History,
    piece::Piece,
    ply::Ply,
    position::Position,
    rank::Rank,
    role::Role,
    square::Square,
    uci::Uci,
    unmoved_rooks::UnmovedRooks,
};

/// Parses a FEN string into a [`Position`].
///
/// See the [module documentation](self) for the accepted grammar.
///
/// # Example
/// ```
/// # use ruchess::fen;
/// # use ruchess::color::Color;
/// let p = fen::parse("8/8/8/8/8/8/4K3/k7 b - - 0 1").unwrap();
/// assert_eq!(p.color(), Color::Black);
/// ```
pub fn parse(s: &str) -> Result<Position, ParseFenError> {
    let mut parts = s.split_ascii_whitespace();
    let board_s = parts.next().ok_or(ParseFenError::MissingField("board"))?;
    let color_s = parts
        .next()
        .ok_or(ParseFenError::MissingField("active color"))?;
    let castles_s = parts
        .next()
        .ok_or(ParseFenError::MissingField("castling rights"))?;
    let ep_s = parts
        .next()
        .ok_or(ParseFenError::MissingField("en passant"))?;
    let halfmove_s = parts.next().unwrap_or("0");
    let fullmove_s = parts.next().unwrap_or("1");

    let board = parse_board(board_s)?;
    let color: Color = color_s.parse()?;
    let castles = parse_castles(castles_s)?;
    let last_move = parse_enpassant(ep_s, color)?;
    let half_move_clock = parse_halfmove(halfmove_s)?;
    let ply = fullmove_s
        .parse()
        .map_err(|_| ParseFenError::InvalidFullmoveNumber)
        .map(|full_moves| Ply::from_full_moves(full_moves, color))?;

    let history = History {
        last_move,
        castles,
        unmoved_rooks: UnmovedRooks::from_board(board),
        half_move_clock,
        position_hashes: PositionHash::empty(),
    };

    Ok(Position::new()
        .with_board(board)
        .with_color(color)
        .with_history(history)
        .with_ply(ply))
}

fn parse_board(s: &str) -> Result<Board, ParseBoardError> {
    let ranks: Vec<&str> = s.split('/').collect();
    if ranks.len() != 8 {
        return Err(ParseBoardError::InvalidRankCount);
    }
    let mut board = Board::EMPTY;
    for (i, rank_str) in ranks.iter().enumerate() {
        // FEN lists ranks top-down: ranks[0] is rank 8, ranks[7] is rank 1.
        let rank = Rank::new((7 - i) as u32);
        let mut file_idx: u8 = 0;
        for c in rank_str.chars() {
            if let Some(skip) = c.to_digit(10) {
                if !(1..=8).contains(&skip) {
                    return Err(ParseBoardError::InvalidChar(c));
                }
                file_idx += skip as u8;
                if file_idx > 8 {
                    return Err(ParseBoardError::RankOverflow);
                }
            } else {
                if file_idx >= 8 {
                    return Err(ParseBoardError::RankOverflow);
                }
                let piece = piece_from_fen_char(c).ok_or(ParseBoardError::InvalidChar(c))?;
                board = board.set(
                    Square::from_file_and_rank(File::new(file_idx as u32), rank),
                    piece,
                );
                file_idx += 1;
            }
        }
        if file_idx != 8 {
            return Err(ParseBoardError::InvalidRankLength);
        }
    }
    Ok(board)
}

fn piece_from_fen_char(c: char) -> Option<Piece> {
    let color = if c.is_ascii_uppercase() {
        Color::White
    } else {
        Color::Black
    };
    let role = match c.to_ascii_lowercase() {
        'p' => Role::Pawn,
        'n' => Role::Knight,
        'b' => Role::Bishop,
        'r' => Role::Rook,
        'q' => Role::Queen,
        'k' => Role::King,
        _ => return None,
    };
    Some(Piece { role, color })
}

fn parse_castles(s: &str) -> Result<Castles, ParseFenError> {
    if s == "-" {
        return Ok(Castles::NONE);
    }
    let (mut wk, mut wq, mut bk, mut bq) = (false, false, false, false);
    for c in s.chars() {
        match c {
            'K' => wk = true,
            'Q' => wq = true,
            'k' => bk = true,
            'q' => bq = true,
            _ => return Err(ParseFenError::InvalidCastlingRights),
        }
    }
    Ok(Castles::new(wk, wq, bk, bq))
}

fn parse_enpassant(s: &str, color: Color) -> Result<Option<Uci>, ParseFenError> {
    if s == "-" {
        return Ok(None);
    }
    let target: Square = s.parse().map_err(|_| ParseFenError::InvalidEnPassant)?;
    // The target sits between the pusher's origin and destination. With White
    // to move, Black just double-pushed (e.g. e7→e5, target e6); with Black to
    // move, White just double-pushed (e.g. e2→e4, target e3).
    let (orig_rank, dest_rank) = match (color, target.rank()) {
        (Color::White, Rank::Sixth) => (Rank::Seventh, Rank::Fifth),
        (Color::Black, Rank::Third) => (Rank::Second, Rank::Fourth),
        _ => return Err(ParseFenError::InvalidEnPassant),
    };
    let file = target.file();
    Ok(Some(Uci {
        orig: Square::from_file_and_rank(file, orig_rank),
        dest: Square::from_file_and_rank(file, dest_rank),
        promotion: None,
    }))
}

fn parse_halfmove(s: &str) -> Result<HalfMoveClock, ParseFenError> {
    let n: u8 = s.parse().map_err(|_| ParseFenError::InvalidHalfmoveClock)?;
    let mut clock = HalfMoveClock::new();
    for _ in 0..n {
        clock = clock.incr();
    }
    Ok(clock)
}

/// Error returned by [`parse`] when a FEN string is malformed.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseFenError {
    /// A required FEN field was absent.
    MissingField(&'static str),
    /// The board (first) field failed to parse.
    ParseBoardError(ParseBoardError),
    /// The active-color field was not `w` or `b`.
    InvalidActiveColor,
    /// The castling-rights field contained an unrecognized character.
    InvalidCastlingRights,
    /// The en-passant field was neither `-` nor a square on rank 3 (Black to
    /// move) or rank 6 (White to move).
    InvalidEnPassant,
    /// The halfmove clock was not a non-negative integer fitting in `u8`.
    InvalidHalfmoveClock,
    /// The fullmove number was not a non-negative integer fitting in `u32`.
    InvalidFullmoveNumber,
}

impl Display for ParseFenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "missing FEN field: {field}"),
            Self::ParseBoardError(e) => write!(f, "invalid board: {e}"),
            Self::InvalidActiveColor => write!(f, "invalid active color (expected `w` or `b`)"),
            Self::InvalidCastlingRights => write!(f, "invalid castling rights"),
            Self::InvalidEnPassant => write!(f, "invalid en passant target"),
            Self::InvalidHalfmoveClock => write!(f, "invalid halfmove clock"),
            Self::InvalidFullmoveNumber => write!(f, "invalid fullmove number"),
        }
    }
}
impl Error for ParseFenError {}

/// Error returned when parsing the board (piece-placement) field of a FEN.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParseBoardError {
    /// The placement field did not have exactly 8 `/`-separated ranks.
    InvalidRankCount,
    /// A rank's piece/skip sequence did not cover all 8 files.
    InvalidRankLength,
    /// A rank extended past the H file via too many pieces or oversized skips.
    RankOverflow,
    /// A character in a rank was neither a piece letter nor a `1`–`8` skip.
    InvalidChar(char),
}

impl Display for ParseBoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRankCount => write!(f, "expected 8 ranks separated by `/`"),
            Self::InvalidRankLength => write!(f, "rank did not cover 8 files"),
            Self::RankOverflow => write!(f, "rank extends past the H file"),
            Self::InvalidChar(c) => write!(f, "invalid character `{c}` in board"),
        }
    }
}
impl Error for ParseBoardError {}
impl From<ParseBoardError> for ParseFenError {
    fn from(value: ParseBoardError) -> Self {
        Self::ParseBoardError(value)
    }
}

impl From<ParseColorError> for ParseFenError {
    fn from(_: ParseColorError) -> Self {
        ParseFenError::InvalidActiveColor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn starting_position_matches_default() {
        assert_eq!(parse(START).unwrap(), Position::new());
    }

    #[test]
    fn pieces_use_standard_fen_case() {
        let p = parse(START).unwrap();
        let b = p.board();
        assert_eq!(
            b.piece_at(square::A1),
            Some(Piece {
                role: Role::Rook,
                color: Color::White
            })
        );
        assert_eq!(
            b.piece_at(square::E8),
            Some(Piece {
                role: Role::King,
                color: Color::Black
            })
        );
        assert_eq!(
            b.piece_at(square::E2),
            Some(Piece {
                role: Role::Pawn,
                color: Color::White
            })
        );
    }

    #[test]
    fn active_color_black() {
        let p = parse("4k3/8/8/8/8/8/8/4K3 b - - 0 1").unwrap();
        assert_eq!(p.color(), Color::Black);
    }

    #[test]
    fn castling_dash_is_empty() {
        let p = parse("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        assert!(p.history().castles.is_empty());
    }

    #[test]
    fn castling_partial_rights() {
        let p = parse("r3k2r/8/8/8/8/8/8/R3K2R w Kq - 0 1").unwrap();
        let c = p.history().castles;
        assert!(c.white_king_side());
        assert!(!c.white_queen_side());
        assert!(!c.black_king_side());
        assert!(c.black_queen_side());
    }

    #[test]
    fn enpassant_target_white_to_move() {
        // Black just played d7→d5; target is d6.
        let p = parse("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1").unwrap();
        assert_eq!(p.enpassant_square(), Some(square::D6));
        // And the legal en-passant capture is actually emitted.
        assert_eq!(p.enpassant_moves().count(), 1);
    }

    #[test]
    fn enpassant_target_black_to_move() {
        // White just played e2→e4; target is e3.
        let p = parse("4k3/8/8/8/3pP3/8/8/4K3 b - e3 0 1").unwrap();
        assert_eq!(p.enpassant_square(), Some(square::E3));
        assert_eq!(p.enpassant_moves().count(), 1);
    }

    #[test]
    fn halfmove_clock_parsed() {
        let p = parse("4k3/8/8/8/8/8/8/4K3 w - - 42 1").unwrap();
        assert_eq!(p.history().half_moves(), 42);
    }

    #[test]
    fn halfmove_and_fullmove_optional() {
        let p = parse("4k3/8/8/8/8/8/8/4K3 w - -").unwrap();
        assert_eq!(p.history().half_moves(), 0);
    }

    #[test]
    fn complex_position_round_trips_piece_placement() {
        // Kiwipete — exercises a busy middlegame with full castling rights.
        let kiwipete = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let p = parse(kiwipete).unwrap();
        assert_eq!(
            p.board().piece_at(square::E5),
            Some(Piece {
                role: Role::Knight,
                color: Color::White
            })
        );
        assert_eq!(
            p.board().piece_at(square::F3),
            Some(Piece {
                role: Role::Queen,
                color: Color::White
            })
        );
        assert_eq!(p.board().occupied().0.count_ones(), 32);
    }

    // ── Error paths ──────────────────────────────────────────────────────

    #[test]
    fn missing_color_field() {
        assert_eq!(
            parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            Err(ParseFenError::MissingField("active color"))
        );
    }

    #[test]
    fn invalid_color() {
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4K3 x - - 0 1"),
            Err(ParseFenError::InvalidActiveColor)
        );
    }

    #[test]
    fn invalid_piece_char() {
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4X3 w - - 0 1"),
            Err(ParseFenError::ParseBoardError(
                ParseBoardError::InvalidChar('X')
            ))
        );
    }

    #[test]
    fn rank_count_wrong() {
        assert_eq!(
            parse("4k3/8/8/8/8/8/4K3 w - - 0 1"),
            Err(ParseFenError::ParseBoardError(
                ParseBoardError::InvalidRankCount
            ))
        );
    }

    #[test]
    fn rank_length_wrong() {
        // Rank only sums to 7 files.
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4K2 w - - 0 1"),
            Err(ParseFenError::ParseBoardError(
                ParseBoardError::InvalidRankLength
            ))
        );
    }

    #[test]
    fn rank_overflow() {
        // 4K3 + extra piece = 9 squares.
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4K3P w - - 0 1"),
            Err(ParseFenError::ParseBoardError(
                ParseBoardError::RankOverflow
            ))
        );
    }

    #[test]
    fn invalid_castling() {
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4K3 w KZ - 0 1"),
            Err(ParseFenError::InvalidCastlingRights)
        );
    }

    #[test]
    fn invalid_enpassant_square() {
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4K3 w - zz 0 1"),
            Err(ParseFenError::InvalidEnPassant)
        );
    }

    #[test]
    fn invalid_enpassant_wrong_rank() {
        // White to move but target is on rank 3 (would imply White just moved).
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4K3 w - e3 0 1"),
            Err(ParseFenError::InvalidEnPassant)
        );
    }

    #[test]
    fn invalid_halfmove() {
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4K3 w - - x 1"),
            Err(ParseFenError::InvalidHalfmoveClock)
        );
    }

    #[test]
    fn invalid_fullmove() {
        assert_eq!(
            parse("4k3/8/8/8/8/8/8/4K3 w - - 0 x"),
            Err(ParseFenError::InvalidFullmoveNumber)
        );
    }
}
