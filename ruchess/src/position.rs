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

#[derive(Debug, Clone)]
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
        F: FnOnce(History) -> History,
    {
        Self {
            history: f(self.history),
            ..self
        }
    }

    pub fn mve(&self, orig: Square, dest: Square) -> Option<Self> {
        let mve = self
            .valid_moves()
            .find(|m| m.orig == orig && m.dest == dest)?;

        todo!()
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::square::{A1, A2, A3, A4, B7, D5, E4, E5};
//
//     fn board_with(pieces: &[(Square, Piece)]) -> Board {
//         pieces
//             .iter()
//             .fold(Board::EMPTY, |b, (sq, p)| b.set(*sq, *p))
//     }
//
//     // ── Move::normal ─────────────────────────────────────────────────────────
//
//     #[test]
//     fn normal_move_onto_empty_square_succeeds() {
//         let m = Move::normal(Board::new(), A2, A3, false, Color::White, Role::Pawn)
//             .expect("a2-a3 should be legal from the starting position");
//
//         assert_eq!(m.orig, A2);
//         assert_eq!(m.dest, A3);
//         assert!(m.capture.is_none());
//         assert!(m.promotion.is_none());
//         assert!(m.castle.is_none());
//         assert!(m.enpassant.is_none());
//
//         // The piece left its origin and now lives on the destination.
//         assert!(!m.after.is_occupied(A2));
//         let landed = m.after.piece_at(A3).expect("pawn should be on a3");
//         assert_eq!(landed.role, Role::Pawn);
//         assert_eq!(landed.color, Color::White);
//     }
//
//     #[test]
//     fn normal_non_capture_onto_occupied_square_returns_none() {
//         // a1 holds the white rook; trying to slide it onto its own pawn on a2
//         // without `is_capture` must fail.
//         assert!(Move::normal(Board::new(), A1, A2, false, Color::White, Role::Rook).is_none());
//     }
//
//     #[test]
//     fn normal_from_empty_origin_returns_none() {
//         let b = Board::EMPTY;
//         assert!(Move::normal(b, A1, A2, false, Color::White, Role::Rook).is_none());
//     }
//
//     #[test]
//     fn normal_capture_records_target_square_and_replaces_piece() {
//         let b = board_with(&[
//             (
//                 E4,
//                 Piece {
//                     role: Role::Knight,
//                     color: Color::White,
//                 },
//             ),
//             (
//                 D5,
//                 Piece {
//                     role: Role::Pawn,
//                     color: Color::Black,
//                 },
//             ),
//         ]);
//
//         let m = Move::normal(b, E4, D5, true, Color::White, Role::Knight)
//             .expect("knight should be able to capture on d5");
//
//         assert_eq!(m.capture, Some(D5));
//         assert!(!m.after.is_occupied(E4));
//         let landed = m.after.piece_at(D5).expect("knight should be on d5");
//         assert_eq!(landed.role, Role::Knight);
//         assert_eq!(landed.color, Color::White);
//     }
//
//     // ── pawn_moves ───────────────────────────────────────────────────────────
//
//     #[test]
//     fn pawn_moves_from_starting_position_white() {
//         let moves = pawn_moves(Board::new(), Color::White);
//         // 8 single pushes + 8 double pushes, no captures available.
//         assert_eq!(moves.len(), 16);
//         assert!(moves.iter().all(|m| m.capture.is_none()));
//         assert!(moves.iter().all(|m| m.promotion.is_none()));
//     }
//
//     #[test]
//     fn pawn_moves_from_starting_position_black() {
//         let moves = pawn_moves(Board::new(), Color::Black);
//         assert_eq!(moves.len(), 16);
//         assert!(moves.iter().all(|m| m.capture.is_none()));
//     }
//
//     #[test]
//     fn pawn_double_push_blocked_when_intermediate_square_occupied() {
//         // A friendly piece on a3 blocks both a2-a3 and the a2-a4 double push.
//         let b = Board::new().set(
//             A3,
//             Piece {
//                 role: Role::Knight,
//                 color: Color::White,
//             },
//         );
//
//         let from_a2: Vec<_> = pawn_moves(b, Color::White)
//             .into_iter()
//             .filter(|m| m.orig == A2)
//             .collect();
//         assert!(from_a2.is_empty());
//     }
//
//     #[test]
//     fn pawn_double_push_blocked_when_target_square_occupied() {
//         // An enemy piece on a4 leaves the single push open but cancels the double.
//         let b = Board::new().set(
//             A4,
//             Piece {
//                 role: Role::Pawn,
//                 color: Color::Black,
//             },
//         );
//
//         let from_a2: Vec<_> = pawn_moves(b, Color::White)
//             .into_iter()
//             .filter(|m| m.orig == A2)
//             .collect();
//         assert_eq!(from_a2.len(), 1);
//         assert_eq!(from_a2[0].dest, A3);
//         assert!(from_a2[0].capture.is_none());
//     }
//
//     #[test]
//     fn pawn_captures_diagonally_onto_enemy_piece() {
//         // Lone white pawn on e4, lone black pawn on d5 — d5 must be flagged as a capture.
//         let b = board_with(&[
//             (
//                 E4,
//                 Piece {
//                     role: Role::Pawn,
//                     color: Color::White,
//                 },
//             ),
//             (
//                 D5,
//                 Piece {
//                     role: Role::Pawn,
//                     color: Color::Black,
//                 },
//             ),
//         ]);
//
//         let captures: Vec<_> = pawn_moves(b, Color::White)
//             .into_iter()
//             .filter(|m| m.capture.is_some())
//             .collect();
//
//         assert_eq!(captures.len(), 1);
//         assert_eq!(captures[0].orig, E4);
//         assert_eq!(captures[0].dest, D5);
//         assert_eq!(captures[0].capture, Some(D5));
//         // The pawn actually moved on the resulting board.
//         assert!(!captures[0].after.is_occupied(E4));
//         let landed = captures[0].after.piece_at(D5).unwrap();
//         assert_eq!(landed.role, Role::Pawn);
//         assert_eq!(landed.color, Color::White);
//     }
//
//     #[test]
//     fn pawn_does_not_capture_own_color() {
//         // Two adjacent white pawns: e4 and d5. e4 must not "capture" d5.
//         let b = board_with(&[
//             (
//                 E4,
//                 Piece {
//                     role: Role::Pawn,
//                     color: Color::White,
//                 },
//             ),
//             (
//                 D5,
//                 Piece {
//                     role: Role::Pawn,
//                     color: Color::White,
//                 },
//             ),
//         ]);
//
//         let captures: Vec<_> = pawn_moves(b, Color::White)
//             .into_iter()
//             .filter(|m| m.capture.is_some())
//             .collect();
//         assert!(captures.is_empty());
//     }
//
//     #[test]
//     fn lone_pawn_at_e4_has_only_a_single_push() {
//         let b = board_with(&[(
//             E4,
//             Piece {
//                 role: Role::Pawn,
//                 color: Color::White,
//             },
//         )]);
//
//         let moves = pawn_moves(b, Color::White);
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].orig, E4);
//         assert_eq!(moves[0].dest, E5);
//         assert!(moves[0].capture.is_none());
//     }
//
//     #[test]
//     fn black_pawn_attacks_two_white_targets_with_two_captures() {
//         // Black pawn on b7 with two white pieces diagonally in front: a6 and c6.
//         // Use a7 / c7 squares — but b7's black-pawn attacks land on a6 and c6.
//         use crate::square::{A6, C6};
//         let b = board_with(&[
//             (
//                 B7,
//                 Piece {
//                     role: Role::Pawn,
//                     color: Color::Black,
//                 },
//             ),
//             (
//                 A6,
//                 Piece {
//                     role: Role::Pawn,
//                     color: Color::White,
//                 },
//             ),
//             (
//                 C6,
//                 Piece {
//                     role: Role::Pawn,
//                     color: Color::White,
//                 },
//             ),
//         ]);
//
//         let captures: Vec<_> = pawn_moves(b, Color::Black)
//             .into_iter()
//             .filter(|m| m.capture.is_some())
//             .collect();
//         assert_eq!(captures.len(), 2);
//         let dests: Vec<_> = captures.iter().map(|m| m.dest).collect();
//         assert!(dests.contains(&A6));
//         assert!(dests.contains(&C6));
//     }
// }
