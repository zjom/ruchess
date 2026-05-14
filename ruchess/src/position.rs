use crate::{
    attacks::ATTACKS,
    bitboard::Bitboard,
    board::Board,
    castles::Castles,
    color::Color,
    history::History,
    mve::Move,
    piece::Piece,
    role::{PromotableRole, Role},
    side::Side,
    square::{self, Square},
    uci::Uci,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    board: Board,
    history: History,
    color: Color,
}

impl Position {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            history: History::new(),
            color: Color::White,
        }
    }

    pub fn with_board(self, board: Board) -> Self {
        Self { board, ..self }
    }

    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn change_color(self) -> Self {
        let color = self.color;
        self.with_color(color)
    }
    pub fn with_history(self, history: History) -> Self {
        Self { history, ..self }
    }
    pub fn with_castles(self, castles: Castles) -> Self {
        Self {
            history: self.history.with_castles(castles),
            ..self
        }
    }
    pub fn update_history<F>(self, f: F) -> Self
    where
        F: FnOnce(&History) -> History,
    {
        Self {
            history: f(&self.history),
            ..self
        }
    }

    pub fn mve(self, orig: Square, dest: Square) -> Option<Self> {
        let mve = self
            .valid_moves()
            .find(|m| m.orig == orig && m.dest == dest)?;

        let history = self.history.clone().update(&self, &mve);
        Some(Self {
            board: mve.after,
            history,
            color: self.color.opponent(),
        })
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn is_check(&self) -> bool {
        self.board.is_check(self.color)
    }

    pub fn enpassant_square(&self) -> Option<Square> {
        self.history
            .last_move
            .and_then(|lm| potential_enpassant_sq(lm, self.board, self.color))
    }

    pub fn valid_moves(&self) -> impl Iterator<Item = Move> {
        self.pawn_moves()
            .chain(self.enpassant_moves())
            .chain(self.king_moves())
            .chain(self.knight_moves())
            .chain(self.bishop_moves())
            .chain(self.rook_moves())
            .chain(self.queen_moves())
    }
    pub fn has_moves(&self) -> bool {
        self.valid_moves().any(|_| true)
    }

    pub fn valid_moves_at(&self, orig: Square) -> impl Iterator<Item = Move> {
        self.valid_moves().filter(move |m| m.orig == orig)
    }

    pub fn pawn_moves(&self) -> impl Iterator<Item = Move> {
        let pawns = self.board.bypiece(Piece {
            role: Role::Pawn,
            color: self.color,
        });
        let captures = pawns
            .flat_map(|from| {
                (ATTACKS.pawn_attacks(self.color, from) & self.board.bycolor(self.color))
                    .into_iter()
                    .map(move |to| (from, to))
            })
            .flat_map(|(from, to)| self.gen_pawn_moves(from, to, true));

        let singles = !self.board.occupied()
            & (match self.color {
                Color::White => (self.board.white() & pawns) << 8,
                Color::Black => (self.board.black() & pawns) >> 8,
            });

        let single_moves = singles.flat_map(|to| {
            let from = Square(match self.color {
                Color::White => to.0 - 8,
                Color::Black => to.0 + 8,
            });
            self.gen_pawn_moves(from, to, false)
        });

        let doubles = !self.board.occupied()
            & (match self.color {
                Color::White => singles << 8,
                Color::Black => singles >> 8,
            })
            & self.color.fourth_rank();
        let double_moves = doubles.flat_map(|to| {
            let from = Square(match self.color {
                Color::White => to.0 - 16,
                Color::Black => to.0 + 16,
            });
            self.gen_pawn_moves(from, to, false)
        });

        captures.chain(single_moves).chain(double_moves)
    }

    pub fn enpassant_moves(&self) -> impl Iterator<Item = Move> {
        self.history
            .last_move
            .and_then(|last_move| {
                let target = potential_enpassant_sq(last_move, self.board, self.color)?;
                let our_pawns = self.board.bypiece(Piece {
                    role: Role::Pawn,
                    color: self.color,
                });
                Some(
                    (ATTACKS.pawn_attacks(self.color.opponent(), target) & our_pawns)
                        .into_iter()
                        .filter_map(move |from| self.enpassant(from, target)),
                )
            })
            .into_iter()
            .flatten()
    }

    pub fn king_moves(&self) -> impl Iterator<Item = Move> {
        let orig = self.board.king(self.color);
        let moves = ATTACKS.king_attacks(orig).filter_map(move |dest| {
            (!self.board.is_attacked(dest, self.color.opponent())).then_some(self.normal(
                orig,
                dest,
                Role::King,
            )?)
        });
        moves.chain(self.castling_moves().into_iter().flatten())
    }

    pub fn knight_moves(&self) -> impl Iterator<Item = Move> {
        let knights = self.board.bypiece(Piece {
            role: Role::Knight,
            color: self.color,
        });
        knights
            .flat_map(|from| ATTACKS.knight_attacks(from).map(move |to| (from, to)))
            .filter_map(|(from, to)| self.normal(from, to, Role::Knight))
    }

    pub fn bishop_moves(&self) -> impl Iterator<Item = Move> {
        let bishops = self.board.bypiece(Piece {
            role: Role::Bishop,
            color: self.color,
        });
        bishops
            .flat_map(|from| {
                ATTACKS
                    .bishop_attacks(from, self.board.occupied())
                    .map(move |to| (from, to))
            })
            .filter_map(|(from, to)| self.normal(from, to, Role::Bishop))
    }
    pub fn rook_moves(&self) -> impl Iterator<Item = Move> {
        let rooks = self.board.bypiece(Piece {
            role: Role::Rook,
            color: self.color,
        });
        rooks
            .flat_map(|from| {
                ATTACKS
                    .rook_attacks(from, self.board.occupied())
                    .map(move |to| (from, to))
            })
            .filter_map(|(from, to)| self.normal(from, to, Role::Rook))
    }

    pub fn queen_moves(&self) -> impl Iterator<Item = Move> {
        let queens = self.board.bypiece(Piece {
            role: Role::Queen,
            color: self.color,
        });
        queens
            .flat_map(|from| {
                let bishops = ATTACKS
                    .bishop_attacks(from, self.board.occupied())
                    .map(move |to| (from, to));
                let rooks = ATTACKS
                    .rook_attacks(from, self.board.occupied())
                    .map(move |to| (from, to));
                bishops.chain(rooks)
            })
            .filter_map(|(from, to)| self.normal(from, to, Role::Queen))
    }

    fn castling_moves(&self) -> Option<impl Iterator<Item = Move>> {
        if self.board.is_check(self.color) {
            return None;
        }
        Some(
            [Side::King, Side::Queen]
                .into_iter()
                .filter_map(|side| self.castle(side)),
        )
    }

    fn castle(&self, side: Side) -> Option<Move> {
        if !self.history.castles.can_side(self.color, side) {
            return None;
        }

        let king_from = self.board.king(self.color);
        let rook_from = self.color.castle_square(side);
        if !self.history.unmoved_rooks.contains(rook_from) {
            return None;
        }
        let (king_to, rook_to, between, king_path) = castle_squares(self.color, side);

        if (self.board.occupied() & between).is_non_empty() {
            return None;
        }
        if king_path
            .into_iter()
            .any(|sq| self.board.is_attacked(sq, self.color))
        {
            return None;
        }

        let after = self
            .board
            .mve(king_from, king_to)?
            .mve(rook_from, rook_to)?;

        Some(Move::castle(
            self.color, side, king_from, king_to, self.board, after,
        ))
    }

    fn normal(&self, orig: Square, dest: Square, role: Role) -> Option<Move> {
        let piece = Piece {
            role,
            color: self.color,
        };
        if self.board.is_occupied(dest) {
            let after = self.board.capture(orig, dest, None)?;
            Some(Move::capture(piece, orig, dest, dest, self.board, after))
        } else {
            let after = self.board.mve(orig, dest)?;
            Some(Move::quiet(piece, orig, dest, self.board, after))
        }
    }

    /// Builds an enpassant [`Move`] from `orig` to `dest`.
    pub fn enpassant(&self, orig: Square, dest: Square) -> Option<Move> {
        let captured = Square::from_file_and_rank(dest.file(), orig.rank());
        let after = self.board.capture(orig, dest, Some(captured))?;
        Some(Move::enpassant(
            self.color, orig, dest, captured, self.board, after,
        ))
    }

    fn gen_pawn_moves(
        &self,
        from: Square,
        to: Square,
        is_capture: bool,
    ) -> impl Iterator<Item = Move> + '_ {
        let is_promotion = from.rank() == self.color.seventh_rank();

        // Up to 4 promotion moves — empty if not a promoting rank.
        let promotions = is_promotion
            .then_some(PromotableRole::ROLES)
            .into_iter()
            .flatten()
            .filter_map(move |r| {
                let (after, captured) = if is_capture {
                    (self.board.capture(from, to, None)?, Some(to))
                } else {
                    (self.board.mve(from, to)?, None)
                };
                Some(Move::promotion(
                    self.color, from, to, r, captured, self.board, after,
                ))
            });

        // 0 or 1 normal pawn moves — empty if it IS a promoting rank.
        let normal = (!is_promotion)
            .then(|| self.normal(from, to, Role::Pawn))
            .flatten()
            .into_iter();

        promotions.chain(normal)
    }
}

/// For `color` castling on `side`, returns
/// `(king_destination, rook_destination, must_be_empty, king_path_must_be_safe)`.
///
/// `must_be_empty` covers every square strictly between king and rook;
/// `king_path_must_be_safe` covers the squares the king transits to and
/// lands on (the king's starting square is already covered by the caller's
/// in-check test).
fn castle_squares(color: Color, side: Side) -> (Square, Square, Bitboard, Bitboard) {
    match (color, side) {
        (Color::White, Side::King) => (
            square::G1,
            square::F1,
            Bitboard::from(square::F1) | Bitboard::from(square::G1),
            Bitboard::from(square::F1) | Bitboard::from(square::G1),
        ),
        (Color::White, Side::Queen) => (
            square::C1,
            square::D1,
            Bitboard::from(square::B1) | Bitboard::from(square::C1) | Bitboard::from(square::D1),
            Bitboard::from(square::C1) | Bitboard::from(square::D1),
        ),
        (Color::Black, Side::King) => (
            square::G8,
            square::F8,
            Bitboard::from(square::F8) | Bitboard::from(square::G8),
            Bitboard::from(square::F8) | Bitboard::from(square::G8),
        ),
        (Color::Black, Side::Queen) => (
            square::C8,
            square::D8,
            Bitboard::from(square::B8) | Bitboard::from(square::C8) | Bitboard::from(square::D8),
            Bitboard::from(square::C8) | Bitboard::from(square::D8),
        ),
    }
}
fn potential_enpassant_sq(last_move: Uci, board: Board, color: Color) -> Option<Square> {
    board.piece_at(last_move.dest).and_then(|piece| {
        if piece.color != color
            && piece.role == Role::Pawn
            && last_move.orig.ydist(last_move.dest) == 2
        {
            last_move.dest.prev_rank(color)
        } else {
            None
        }
    })
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unmoved_rooks::UnmovedRooks;

    fn pc(role: Role, color: Color) -> Piece {
        Piece { role, color }
    }

    /// Position with the given board, otherwise pristine (standard castles,
    /// unmoved_rooks derived from `b`, white to move).
    fn pos_from(b: Board) -> Position {
        let history = History {
            unmoved_rooks: UnmovedRooks::from_board(b),
            ..History::new()
        };
        Position::new().with_board(b).with_history(history)
    }

    // ── Starting position sanity ─────────────────────────────────────────

    #[test]
    fn starting_position_has_20_moves() {
        assert_eq!(Position::new().valid_moves().count(), 20);
    }

    #[test]
    fn starting_position_pawn_moves_count() {
        // 8 pawns × (1 single push + 1 double push) = 16
        assert_eq!(Position::new().pawn_moves().count(), 16);
    }

    #[test]
    fn starting_position_knight_moves_count() {
        // 2 knights × 2 destinations each = 4
        assert_eq!(Position::new().knight_moves().count(), 4);
    }

    #[test]
    fn starting_position_no_castling() {
        let castles: Vec<_> = Position::new()
            .valid_moves()
            .filter(|m| m.castle.is_some())
            .collect();
        assert!(
            castles.is_empty(),
            "no castles legal from start, got {castles:?}"
        );
    }

    #[test]
    fn starting_position_no_enpassant() {
        assert_eq!(Position::new().enpassant_square(), None);
        assert_eq!(Position::new().enpassant_moves().count(), 0);
    }

    #[test]
    fn starting_position_no_promotion() {
        let proms: Vec<_> = Position::new()
            .valid_moves()
            .filter(|m| m.promotion.is_some())
            .collect();
        assert!(proms.is_empty());
    }

    #[test]
    fn starting_position_not_in_check() {
        assert!(!Position::new().is_check());
    }

    #[test]
    fn starting_position_has_moves() {
        assert!(Position::new().has_moves());
    }

    // ── mve / color flip / history ───────────────────────────────────────

    #[test]
    fn mve_flips_color() {
        let after = Position::new().mve(square::E2, square::E4).unwrap();
        assert_eq!(after.color(), Color::Black);
    }

    #[test]
    fn mve_applies_move_to_board() {
        let after = Position::new().mve(square::E2, square::E4).unwrap();
        assert!(!after.board().is_occupied(square::E2));
        assert_eq!(
            after.board().piece_at(square::E4),
            Some(pc(Role::Pawn, Color::White))
        );
    }

    #[test]
    fn mve_invalid_returns_none() {
        // E2 → E5 is not a legal pawn move (only 1 or 2 squares forward).
        assert!(Position::new().mve(square::E2, square::E5).is_none());
    }

    #[test]
    fn mve_from_empty_square_returns_none() {
        assert!(Position::new().mve(square::E4, square::E5).is_none());
    }

    #[test]
    fn mve_updates_history_last_move() {
        let after = Position::new().mve(square::E2, square::E4).unwrap();
        let lm = after.history().last_move.expect("last_move set after mve");
        assert_eq!(lm.orig, square::E2);
        assert_eq!(lm.dest, square::E4);
    }

    #[test]
    fn change_color_actually_flips() {
        let p = Position::new();
        assert_eq!(p.color(), Color::White);
        // change_color should produce a position whose color is the opponent.
        // The current implementation in position.rs:40-43 is a no-op — this
        // test surfaces the bug.
        let q = p.change_color();
        assert_eq!(q.color(), Color::Black);
    }

    // ── Pawn pushes ──────────────────────────────────────────────────────

    #[test]
    fn pawn_double_push_emits_two_destinations() {
        let moves: Vec<_> = Position::new().valid_moves_at(square::E2).collect();
        assert_eq!(moves.len(), 2);
        let dests: Vec<_> = moves.iter().map(|m| m.dest).collect();
        assert!(dests.contains(&square::E3));
        assert!(dests.contains(&square::E4));
    }

    #[test]
    fn pawn_blocked_cannot_push() {
        // White pawn on E2 with a black knight directly in front on E3.
        let b = Board::EMPTY
            .set(square::E1, pc(Role::King, Color::White))
            .set(square::E8, pc(Role::King, Color::Black))
            .set(square::E2, pc(Role::Pawn, Color::White))
            .set(square::E3, pc(Role::Knight, Color::Black));
        let p = pos_from(b);
        let pushes: Vec<_> = p
            .valid_moves_at(square::E2)
            .filter(|m| m.capture.is_none())
            .collect();
        assert!(
            pushes.is_empty(),
            "pawn blocked on E3 cannot push to E3 or E4, got {pushes:?}"
        );
    }

    // ── En passant ───────────────────────────────────────────────────────

    #[test]
    fn enpassant_offered_after_opposing_double_push() {
        // White pawn on E5, black just played D7→D5.
        let b = Board::EMPTY
            .set(square::E1, pc(Role::King, Color::White))
            .set(square::E8, pc(Role::King, Color::Black))
            .set(square::E5, pc(Role::Pawn, Color::White))
            .set(square::D5, pc(Role::Pawn, Color::Black));
        let history = History {
            last_move: Some(crate::uci::Uci {
                orig: square::D7,
                dest: square::D5,
                promotion: None,
            }),
            unmoved_rooks: UnmovedRooks::from_board(b),
            ..History::new()
        };
        let p = Position::new().with_board(b).with_history(history);
        assert_eq!(
            p.enpassant_square(),
            Some(square::D6),
            "en passant target should be D6 (the square the black pawn passed)"
        );
        let eps: Vec<_> = p.enpassant_moves().collect();
        assert_eq!(eps.len(), 1, "exactly one en-passant move available");
        let m = eps[0];
        assert_eq!(m.orig, square::E5);
        assert_eq!(m.dest, square::D6);
        assert_eq!(m.capture, Some(square::D5));
        assert!(m.enpassant.is_some());
    }

    #[test]
    fn enpassant_not_offered_after_single_push() {
        // White pawn on E5, black played D6→D5 (single push, not double).
        let b = Board::EMPTY
            .set(square::E1, pc(Role::King, Color::White))
            .set(square::E8, pc(Role::King, Color::Black))
            .set(square::E5, pc(Role::Pawn, Color::White))
            .set(square::D5, pc(Role::Pawn, Color::Black));
        let history = History {
            last_move: Some(crate::uci::Uci {
                orig: square::D6,
                dest: square::D5,
                promotion: None,
            }),
            unmoved_rooks: UnmovedRooks::from_board(b),
            ..History::new()
        };
        let p = Position::new().with_board(b).with_history(history);
        assert_eq!(p.enpassant_square(), None);
        assert_eq!(p.enpassant_moves().count(), 0);
    }

    // ── Promotion ────────────────────────────────────────────────────────

    #[test]
    fn pawn_on_seventh_promotes_to_four_pieces() {
        let b = Board::EMPTY
            .set(square::E1, pc(Role::King, Color::White))
            .set(square::H8, pc(Role::King, Color::Black))
            .set(square::A7, pc(Role::Pawn, Color::White));
        let p = pos_from(b);
        let moves: Vec<_> = p.valid_moves_at(square::A7).collect();
        assert_eq!(moves.len(), 4, "4 promotion choices; got {moves:?}");
        let promos: Vec<_> = moves.iter().filter_map(|m| m.promotion).collect();
        assert!(promos.contains(&PromotableRole::Queen));
        assert!(promos.contains(&PromotableRole::Rook));
        assert!(promos.contains(&PromotableRole::Bishop));
        assert!(promos.contains(&PromotableRole::Knight));
        for m in &moves {
            assert_eq!(m.dest, square::A8);
            assert_eq!(m.capture, None);
        }
    }

    #[test]
    fn pawn_pre_promotion_push_has_no_promotion_field() {
        let b = Board::EMPTY
            .set(square::E1, pc(Role::King, Color::White))
            .set(square::E8, pc(Role::King, Color::Black))
            .set(square::A6, pc(Role::Pawn, Color::White));
        let p = pos_from(b);
        let moves: Vec<_> = p.valid_moves_at(square::A6).collect();
        assert!(
            moves.iter().all(|m| m.promotion.is_none()),
            "no promotion on rank-6 push, got {moves:?}"
        );
    }

    // ── Castling ─────────────────────────────────────────────────────────

    fn castling_board() -> Board {
        Board::EMPTY
            .set(square::E1, pc(Role::King, Color::White))
            .set(square::A1, pc(Role::Rook, Color::White))
            .set(square::H1, pc(Role::Rook, Color::White))
            .set(square::E8, pc(Role::King, Color::Black))
            .set(square::A8, pc(Role::Rook, Color::Black))
            .set(square::H8, pc(Role::Rook, Color::Black))
    }

    #[test]
    fn castle_kingside_white_available() {
        let p = pos_from(castling_board());
        let castles: Vec<_> = p
            .valid_moves()
            .filter(|m| m.castle == Some(Side::King))
            .collect();
        assert_eq!(castles.len(), 1);
        let m = castles[0];
        assert_eq!(m.orig, square::E1);
        assert_eq!(m.dest, square::G1);
    }

    #[test]
    fn castle_queenside_white_available() {
        let p = pos_from(castling_board());
        let castles: Vec<_> = p
            .valid_moves()
            .filter(|m| m.castle == Some(Side::Queen))
            .collect();
        assert_eq!(castles.len(), 1);
        assert_eq!(castles[0].orig, square::E1);
        assert_eq!(castles[0].dest, square::C1);
    }

    #[test]
    fn cannot_castle_kingside_through_check() {
        // Black rook on F8 attacks F1 (a square the king transits).
        let b = castling_board().set(square::F8, pc(Role::Rook, Color::Black));
        let p = pos_from(b);
        let king_castle = p.valid_moves().find(|m| m.castle == Some(Side::King));
        assert!(
            king_castle.is_none(),
            "F-file rook prevents kingside castle"
        );
    }

    #[test]
    fn cannot_castle_while_in_check() {
        // Black queen on E4 attacks E1 (white king's square) along the E-file.
        let b = castling_board().set(square::E4, pc(Role::Queen, Color::Black));
        let p = pos_from(b);
        assert!(
            p.is_check(),
            "white king on E1 is in check from black Q on E4"
        );
        let any_castle = p.valid_moves().find(|m| m.castle.is_some());
        assert!(
            any_castle.is_none(),
            "no castles allowed while in check, got {any_castle:?}"
        );
    }

    #[test]
    fn cannot_castle_kingside_when_blocked() {
        let b = castling_board().set(square::F1, pc(Role::Bishop, Color::White));
        let p = pos_from(b);
        let king_castle = p.valid_moves().find(|m| m.castle == Some(Side::King));
        assert!(king_castle.is_none(), "bishop on F1 blocks kingside castle");
    }

    #[test]
    fn cannot_castle_without_rights() {
        let p = pos_from(castling_board()).with_castles(Castles::new(false, false, false, false));
        let any_castle = p.valid_moves().find(|m| m.castle.is_some());
        assert!(any_castle.is_none());
    }

    #[test]
    fn cannot_castle_with_moved_rooks() {
        let history = History {
            unmoved_rooks: UnmovedRooks::from_board(Board::EMPTY),
            ..History::new()
        };
        let p = Position::new()
            .with_board(castling_board())
            .with_history(history);
        let any_castle = p.valid_moves().find(|m| m.castle.is_some());
        assert!(any_castle.is_none(), "no castle when rooks have moved");
    }

    // ── is_check ─────────────────────────────────────────────────────────

    #[test]
    fn is_check_detects_back_rank_rook() {
        let b = Board::EMPTY
            .set(square::E1, pc(Role::King, Color::White))
            .set(square::E8, pc(Role::Rook, Color::Black))
            .set(square::A8, pc(Role::King, Color::Black));
        let p = pos_from(b);
        assert!(p.is_check());
    }

    #[test]
    fn is_check_false_when_blocked() {
        let b = Board::EMPTY
            .set(square::E1, pc(Role::King, Color::White))
            .set(square::E4, pc(Role::Pawn, Color::White))
            .set(square::E8, pc(Role::Rook, Color::Black))
            .set(square::A8, pc(Role::King, Color::Black));
        let p = pos_from(b);
        assert!(!p.is_check(), "rook attack blocked by own pawn on E4");
    }

    // ── valid_moves_at consistency (small concrete check) ────────────────

    #[test]
    fn valid_moves_at_matches_filter_on_start() {
        let p = Position::new();
        for i in 0u8..64 {
            let s = Square(i);
            let direct = p.valid_moves_at(s).count();
            let filtered = p.valid_moves().filter(|m| m.orig == s).count();
            assert_eq!(direct, filtered, "mismatch at square {s}");
        }
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::unmoved_rooks::UnmovedRooks;
    use proptest::prelude::*;

    // ── Strategies ───────────────────────────────────────────────────────

    fn sq() -> impl Strategy<Value = Square> {
        (0u8..64).prop_map(Square)
    }

    fn color() -> impl Strategy<Value = Color> {
        prop_oneof![Just(Color::White), Just(Color::Black)]
    }

    fn non_king_role() -> impl Strategy<Value = Role> {
        prop_oneof![
            Just(Role::Pawn),
            Just(Role::Knight),
            Just(Role::Bishop),
            Just(Role::Rook),
            Just(Role::Queen),
        ]
    }

    fn non_king_piece() -> impl Strategy<Value = Piece> {
        (non_king_role(), color()).prop_map(|(role, color)| Piece { role, color })
    }

    /// Builds a position with kings on `wk`/`bk`, the listed non-king pieces sprinkled
    /// on top (king squares and illegal pawn ranks skipped), and no castling rights.
    fn build_position(wk: Square, bk: Square, ops: Vec<(Square, Piece)>, c: Color) -> Position {
        let mut b = Board::EMPTY
            .set(
                wk,
                Piece {
                    role: Role::King,
                    color: Color::White,
                },
            )
            .set(
                bk,
                Piece {
                    role: Role::King,
                    color: Color::Black,
                },
            );
        for (s, p) in ops {
            if s == wk || s == bk {
                continue;
            }
            if p.role == Role::Pawn {
                let r = s.rank().as_u8();
                if r == 0 || r == 7 {
                    continue;
                }
            }
            b = b.set(s, p);
        }
        let history = History {
            castles: Castles::new(false, false, false, false),
            unmoved_rooks: UnmovedRooks::from_board(b),
            ..History::new()
        };
        Position::new()
            .with_board(b)
            .with_color(c)
            .with_history(history)
    }

    /// Random "legal-enough" position: exactly one king per color, kings non-adjacent,
    /// side-not-to-move not in check.
    fn random_position() -> impl Strategy<Value = Position> {
        (
            sq(),
            sq(),
            prop::collection::vec((sq(), non_king_piece()), 0..12),
            color(),
        )
            .prop_filter("kings distinct", |(wk, bk, _, _)| wk != bk)
            .prop_filter("kings non-adjacent", |(wk, bk, _, _)| {
                let dx = (wk.file().as_u8() as i32 - bk.file().as_u8() as i32).abs();
                let dy = (wk.rank().as_u8() as i32 - bk.rank().as_u8() as i32).abs();
                dx > 1 || dy > 1
            })
            .prop_map(|(wk, bk, ops, c)| build_position(wk, bk, ops, c))
            .prop_filter("side-not-to-move not in check", |p| {
                !p.board().is_check(p.color().opponent())
            })
    }

    fn piece_count(b: &Board, role: Role, color: Color) -> u32 {
        b.bypiece(Piece { role, color }).0.count_ones()
    }

    proptest! {
        // 1. Every generated move is for a piece of the side to move.
        #[test]
        fn move_color_matches_side_to_move(p in random_position()) {
            for m in p.valid_moves() {
                prop_assert_eq!(m.piece.color, p.color());
            }
        }

        // 2. No null moves.
        #[test]
        fn move_orig_not_equal_dest(p in random_position()) {
            for m in p.valid_moves() {
                prop_assert_ne!(m.orig, m.dest);
            }
        }

        // 3. The origin square holds the piece claimed by the move.
        #[test]
        fn move_origin_holds_claimed_piece(p in random_position()) {
            for m in p.valid_moves() {
                prop_assert_eq!(p.board().piece_at(m.orig), Some(m.piece));
            }
        }

        // 4. After-board has exactly one king of each color (king cannot be captured).
        #[test]
        fn after_board_keeps_both_kings(p in random_position()) {
            for m in p.valid_moves() {
                let white_kings = piece_count(&m.after, Role::King, Color::White);
                let black_kings = piece_count(&m.after, Role::King, Color::Black);
                prop_assert_eq!(white_kings, 1);
                prop_assert_eq!(black_kings, 1);
            }
        }

        // 5. After-board satisfies bitboard invariants.
        #[test]
        fn after_board_invariants(p in random_position()) {
            for m in p.valid_moves() {
                let b = m.after;
                let white = b.bycolor(Color::White);
                let black = b.bycolor(Color::Black);
                prop_assert_eq!(b.occupied(), white | black);
                prop_assert_eq!(white & black, crate::bitboard::Bitboard::EMPTY);
            }
        }

        // 6. Move generation is deterministic.
        #[test]
        fn deterministic(p in random_position()) {
            let a: Vec<Move> = p.valid_moves().collect();
            let b: Vec<Move> = p.valid_moves().collect();
            prop_assert_eq!(a, b);
        }

        // 7. valid_moves equals the disjoint union of per-piece-type generators.
        #[test]
        fn partition_matches_per_piece_generators(p in random_position()) {
            let total = p.valid_moves().count();
            let sum = p.pawn_moves().count()
                + p.enpassant_moves().count()
                + p.king_moves().count()
                + p.knight_moves().count()
                + p.bishop_moves().count()
                + p.rook_moves().count()
                + p.queen_moves().count();
            prop_assert_eq!(total, sum);
        }

        // 8. valid_moves_at(s) is equivalent to filtering valid_moves() by orig==s.
        #[test]
        fn valid_moves_at_filter_consistent(p in random_position()) {
            for i in 0u8..64 {
                let s = Square(i);
                let direct = p.valid_moves_at(s).count();
                let filtered = p.valid_moves().filter(|m| m.orig == s).count();
                prop_assert_eq!(direct, filtered);
            }
        }

        // 9. has_moves agrees with valid_moves().next().is_some().
        #[test]
        fn has_moves_iff_any(p in random_position()) {
            prop_assert_eq!(p.has_moves(), p.valid_moves().next().is_some());
        }

        // 10. mve() with a listed move succeeds and flips color.
        #[test]
        fn mve_with_listed_move_works(p in random_position()) {
            if let Some(m) = p.valid_moves().next() {
                let next = p.clone().mve(m.orig, m.dest).expect("listed move must succeed");
                prop_assert_eq!(next.color(), p.color().opponent());
                if m.promotion.is_none() {
                    prop_assert_eq!(*next.board(), m.after);
                }
            }
        }

        // 11. Promotion moves only land on the opponent's back rank.
        #[test]
        fn promotion_only_on_opponent_back_rank(p in random_position()) {
            let opp_back = p.color().opponent().back_rank();
            for m in p.valid_moves() {
                if m.promotion.is_some() {
                    prop_assert_eq!(m.dest.rank(), opp_back);
                    prop_assert_eq!(m.piece.role, Role::Pawn);
                }
            }
        }

        // 12. valid_moves() does not mutate the position.
        #[test]
        fn valid_moves_does_not_mutate(p in random_position()) {
            let before = p.clone();
            let _: Vec<Move> = p.valid_moves().collect();
            prop_assert_eq!(p, before);
        }
    }
}
