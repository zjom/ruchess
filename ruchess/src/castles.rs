use crate::{
    bitboard::Bitboard,
    color::Color,
    mve::Move,
    role::Role,
    side::Side,
    square::{self, Square},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Castles(Bitboard);

impl Castles {
    /// All four castling rights available — the standard starting position.
    /// Bits set: A1, H1, A8, H8.
    const INIT: Castles = Castles(Bitboard(0x8100000000000081));
    const NONE: Castles = Castles(Bitboard::EMPTY);

    pub fn standard() -> Self {
        Self::INIT
    }

    pub fn new(
        white_king_side: bool,
        white_queen_side: bool,
        black_king_side: bool,
        black_queen_side: bool,
    ) -> Self {
        let mut bb = Bitboard(0);
        if white_king_side {
            bb |= Bitboard::from(square::H1);
        }
        if white_queen_side {
            bb |= Bitboard::from(square::A1);
        }
        if black_king_side {
            bb |= Bitboard::from(square::H8);
        }
        if black_queen_side {
            bb |= Bitboard::from(square::A8);
        }
        Castles(bb)
    }

    /// Construct from an arbitrary bitboard, masking off any bits that aren't
    /// one of the four valid castling squares.
    pub fn from_bitboard(bb: Bitboard) -> Self {
        Castles(bb & Self::INIT.0)
    }

    pub fn is_empty(self) -> bool {
        self == Self::NONE
    }

    pub fn contains(self, square: Square) -> bool {
        (self.0 & Bitboard::from(square)) != Bitboard(0)
    }

    /// Does `color` have any castling rights left?
    pub fn can(self, color: Color) -> bool {
        (self.0 & color.back_rank()) != Bitboard(0)
    }

    /// Does `color` have rights on this specific side?
    pub fn can_side(self, color: Color, side: Side) -> bool {
        self.contains(color.castle_square(side))
    }

    pub fn white_king_side(self) -> bool {
        self.contains(square::H1)
    }
    pub fn white_queen_side(self) -> bool {
        self.contains(square::A1)
    }
    pub fn black_king_side(self) -> bool {
        self.contains(square::H8)
    }
    pub fn black_queen_side(self) -> bool {
        self.contains(square::A8)
    }

    /// Remove every castling right belonging to `color` (e.g. when its king moves).
    pub fn without(self, color: Color) -> Self {
        Castles(self.0 & !Bitboard::from(color.back_rank()))
    }

    /// Remove a single castling right.
    pub fn without_side(self, color: Color, side: Side) -> Self {
        Castles(self.0 & !Bitboard::from(color.castle_square(side)))
    }

    pub fn add(self, color: Color, side: Side) -> Self {
        Castles(self.0 | Bitboard::from(color.castle_square(side)))
    }

    pub fn update(self, m: &Move) -> Self {
        if m.piece.role == Role::King || m.castle.is_some() {
            self.without(m.piece.color)
        } else {
            self
        }
    }

    /// Replace `color`'s rights wholesale.
    pub fn set(self, color: Color, king_side: bool, queen_side: bool) -> Self {
        let stripped = self.without(color).0;
        let ks = if king_side {
            Bitboard::from(color.castle_square(Side::King))
        } else {
            Bitboard(0)
        };
        let qs = if queen_side {
            Bitboard::from(color.castle_square(Side::Queen))
        } else {
            Bitboard(0)
        };
        Castles(stripped | ks | qs)
    }

    pub fn to_array(self) -> [bool; 4] {
        [
            self.white_king_side(),
            self.white_queen_side(),
            self.black_king_side(),
            self.black_queen_side(),
        ]
    }

    /// FEN castling letter → the rook square it represents (X-FEN not supported).
    pub fn char_to_square(c: char) -> Option<Square> {
        match c {
            'K' => Some(square::H1),
            'Q' => Some(square::A1),
            'k' => Some(square::H8),
            'q' => Some(square::A8),
            _ => None,
        }
    }
}

impl From<Castles> for Bitboard {
    fn from(c: Castles) -> Bitboard {
        c.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::piece::Piece;

    #[test]
    fn standard_has_all_four_rights() {
        let c = Castles::standard();
        assert!(c.white_king_side());
        assert!(c.white_queen_side());
        assert!(c.black_king_side());
        assert!(c.black_queen_side());
        assert_eq!(c.to_array(), [true, true, true, true]);
    }

    #[test]
    fn init_bitboard_matches_four_corners() {
        let expected = Bitboard::from(square::A1)
            | Bitboard::from(square::H1)
            | Bitboard::from(square::A8)
            | Bitboard::from(square::H8);
        assert_eq!(Castles::INIT.0, expected);
    }

    #[test]
    fn none_is_empty() {
        assert!(Castles::NONE.is_empty());
        assert!(!Castles::standard().is_empty());
    }

    #[test]
    fn new_individual_flags() {
        let c = Castles::new(true, false, false, true);
        assert!(c.white_king_side());
        assert!(!c.white_queen_side());
        assert!(!c.black_king_side());
        assert!(c.black_queen_side());
    }

    #[test]
    fn new_all_false_is_empty() {
        assert!(Castles::new(false, false, false, false).is_empty());
    }

    #[test]
    fn from_bitboard_strips_non_castling_bits() {
        let bb = Bitboard::from(square::E4) | Bitboard::from(square::H1);
        let c = Castles::from_bitboard(bb);
        assert!(c.white_king_side());
        assert!(!c.white_queen_side());
        assert!(!c.black_king_side());
        assert!(!c.black_queen_side());
        assert_eq!(Bitboard::from(c), Bitboard::from(square::H1));
    }

    #[test]
    fn contains_only_set_squares() {
        let c = Castles::new(true, false, true, false);
        assert!(c.contains(square::H1));
        assert!(!c.contains(square::A1));
        assert!(c.contains(square::H8));
        assert!(!c.contains(square::A8));
    }

    #[test]
    fn can_color_checks_either_side() {
        assert!(Castles::standard().can(Color::White));
        assert!(Castles::standard().can(Color::Black));

        let only_white = Castles::new(true, true, false, false);
        assert!(only_white.can(Color::White));
        assert!(!only_white.can(Color::Black));
    }

    #[test]
    fn can_side_matches_square() {
        let c = Castles::new(true, false, false, true);
        assert!(c.can_side(Color::White, Side::King));
        assert!(!c.can_side(Color::White, Side::Queen));
        assert!(!c.can_side(Color::Black, Side::King));
        assert!(c.can_side(Color::Black, Side::Queen));
    }

    #[test]
    fn without_removes_only_one_color() {
        let stripped = Castles::standard().without(Color::White);
        assert!(!stripped.white_king_side());
        assert!(!stripped.white_queen_side());
        assert!(stripped.black_king_side());
        assert!(stripped.black_queen_side());
    }

    #[test]
    fn without_side_removes_single_right() {
        let c = Castles::standard().without_side(Color::Black, Side::Queen);
        assert!(c.white_king_side());
        assert!(c.white_queen_side());
        assert!(c.black_king_side());
        assert!(!c.black_queen_side());
    }

    #[test]
    fn add_sets_a_single_right() {
        let c = Castles::NONE.add(Color::White, Side::King);
        assert!(c.white_king_side());
        assert!(!c.white_queen_side());
        assert!(!c.black_king_side());
        assert!(!c.black_queen_side());
    }

    #[test]
    fn add_is_idempotent() {
        let once = Castles::standard().add(Color::White, Side::King);
        let twice = once.add(Color::White, Side::King);
        assert_eq!(once, twice);
    }

    #[test]
    fn set_replaces_color_rights() {
        let c = Castles::standard().set(Color::White, false, true);
        assert!(!c.white_king_side());
        assert!(c.white_queen_side());
        assert!(c.black_king_side());
        assert!(c.black_queen_side());
    }

    #[test]
    fn set_to_false_strips_color() {
        let c = Castles::standard().set(Color::Black, false, false);
        assert!(c.can(Color::White));
        assert!(!c.can(Color::Black));
    }

    #[test]
    fn to_array_matches_individual_getters() {
        let c = Castles::new(true, false, true, false);
        assert_eq!(
            c.to_array(),
            [
                c.white_king_side(),
                c.white_queen_side(),
                c.black_king_side(),
                c.black_queen_side(),
            ]
        );
    }

    #[test]
    fn char_to_square_known_chars() {
        assert_eq!(Castles::char_to_square('K'), Some(square::H1));
        assert_eq!(Castles::char_to_square('Q'), Some(square::A1));
        assert_eq!(Castles::char_to_square('k'), Some(square::H8));
        assert_eq!(Castles::char_to_square('q'), Some(square::A8));
    }

    #[test]
    fn char_to_square_unknown_chars() {
        assert_eq!(Castles::char_to_square('-'), None);
        assert_eq!(Castles::char_to_square('a'), None);
        assert_eq!(Castles::char_to_square('1'), None);
        assert_eq!(Castles::char_to_square(' '), None);
    }

    #[test]
    fn into_bitboard_preserves_bits() {
        let bb: Bitboard = Castles::standard().into();
        assert_eq!(bb, Castles::INIT.0);
    }

    fn quiet(role: Role, color: Color) -> Move {
        Move::quiet(
            Piece { role, color },
            square::E1,
            square::E1,
            Board::EMPTY,
            Board::EMPTY,
        )
    }

    #[test]
    fn update_king_move_clears_color() {
        let c = Castles::standard().update(&quiet(Role::King, Color::White));
        assert!(!c.can(Color::White));
        assert!(c.can(Color::Black));
    }

    #[test]
    fn update_castle_clears_color() {
        let m = Move::castle(
            Color::Black,
            Side::King,
            square::E1,
            square::E1,
            Board::EMPTY,
            Board::EMPTY,
        );
        let c = Castles::standard().update(&m);
        assert!(c.can(Color::White));
        assert!(!c.can(Color::Black));
    }

    #[test]
    fn update_non_king_quiet_move_is_noop() {
        let before = Castles::standard();
        let after = before.update(&quiet(Role::Rook, Color::White));
        assert_eq!(before, after);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    fn castles() -> impl Strategy<Value = Castles> {
        any::<u64>().prop_map(|bits| Castles::from_bitboard(Bitboard(bits)))
    }

    fn color() -> impl Strategy<Value = Color> {
        prop_oneof![Just(Color::White), Just(Color::Black)]
    }

    fn side() -> impl Strategy<Value = Side> {
        prop_oneof![Just(Side::King), Just(Side::Queen)]
    }

    const ALL: [(Color, Side); 4] = [
        (Color::White, Side::King),
        (Color::White, Side::Queen),
        (Color::Black, Side::King),
        (Color::Black, Side::Queen),
    ];

    proptest! {
        #[test]
        fn new_to_array_roundtrip(a: bool, b: bool, c: bool, d: bool) {
            prop_assert_eq!(Castles::new(a, b, c, d).to_array(), [a, b, c, d]);
        }

        #[test]
        fn from_bitboard_is_subset_of_init(bb in any::<u64>()) {
            let c = Castles::from_bitboard(Bitboard(bb));
            prop_assert_eq!(Bitboard::from(c).0 & !Castles::INIT.0.0, 0);
        }

        #[test]
        fn from_bitboard_keeps_valid_bits(bb in any::<u64>()) {
            let c = Castles::from_bitboard(Bitboard(bb));
            prop_assert_eq!(Bitboard::from(c).0, bb & Castles::INIT.0.0);
        }

        #[test]
        fn without_clears_color(c in castles(), col in color()) {
            prop_assert!(!c.without(col).can(col));
        }

        #[test]
        fn without_preserves_opponent(c in castles(), col in color()) {
            let other = col.opponent();
            prop_assert_eq!(c.can(other), c.without(col).can(other));
            prop_assert_eq!(
                c.can_side(other, Side::King),
                c.without(col).can_side(other, Side::King)
            );
            prop_assert_eq!(
                c.can_side(other, Side::Queen),
                c.without(col).can_side(other, Side::Queen)
            );
        }

        #[test]
        fn without_side_clears_only_that_side(
            c in castles(),
            col in color(),
            s in side(),
        ) {
            let after = c.without_side(col, s);
            prop_assert!(!after.can_side(col, s));

            let other_side = if s == Side::King { Side::Queen } else { Side::King };
            prop_assert_eq!(c.can_side(col, other_side), after.can_side(col, other_side));

            let other = col.opponent();
            prop_assert_eq!(c.can_side(other, Side::King), after.can_side(other, Side::King));
            prop_assert_eq!(c.can_side(other, Side::Queen), after.can_side(other, Side::Queen));
        }

        #[test]
        fn add_sets_that_side(c in castles(), col in color(), s in side()) {
            prop_assert!(c.add(col, s).can_side(col, s));
        }

        #[test]
        fn add_only_touches_one_right(c in castles(), col in color(), s in side()) {
            let after = c.add(col, s);
            for &(col2, s2) in &ALL {
                if col2 == col && s2 == s {
                    continue;
                }
                prop_assert_eq!(c.can_side(col2, s2), after.can_side(col2, s2));
            }
        }

        #[test]
        fn without_side_then_add_equals_add(c in castles(), col in color(), s in side()) {
            prop_assert_eq!(c.without_side(col, s).add(col, s), c.add(col, s));
        }

        #[test]
        fn add_then_without_side_equals_without_side(
            c in castles(),
            col in color(),
            s in side(),
        ) {
            prop_assert_eq!(c.add(col, s).without_side(col, s), c.without_side(col, s));
        }

        #[test]
        fn set_overwrites_color(c in castles(), col in color(), ks: bool, qs: bool) {
            let after = c.set(col, ks, qs);
            prop_assert_eq!(after.can_side(col, Side::King), ks);
            prop_assert_eq!(after.can_side(col, Side::Queen), qs);
        }

        #[test]
        fn set_preserves_other_color(c in castles(), col in color(), ks: bool, qs: bool) {
            let other = col.opponent();
            let after = c.set(col, ks, qs);
            prop_assert_eq!(c.can_side(other, Side::King), after.can_side(other, Side::King));
            prop_assert_eq!(c.can_side(other, Side::Queen), after.can_side(other, Side::Queen));
        }

        #[test]
        fn set_built_from_none_matches_new(a: bool, b: bool, c: bool, d: bool) {
            let built = Castles::NONE
                .set(Color::White, a, b)
                .set(Color::Black, c, d);
            prop_assert_eq!(built, Castles::new(a, b, c, d));
        }

        #[test]
        fn contains_matches_can_side(c in castles(), col in color(), s in side()) {
            prop_assert_eq!(c.contains(col.castle_square(s)), c.can_side(col, s));
        }

        #[test]
        fn is_empty_iff_no_color_can(c in castles()) {
            prop_assert_eq!(c.is_empty(), !c.can(Color::White) && !c.can(Color::Black));
        }

        #[test]
        fn can_iff_either_side(c in castles(), col in color()) {
            prop_assert_eq!(
                c.can(col),
                c.can_side(col, Side::King) || c.can_side(col, Side::Queen)
            );
        }

        #[test]
        fn char_to_square_round_trips(col in color(), s in side()) {
            let sq = col.castle_square(s);
            let ch = match (col, s) {
                (Color::White, Side::King) => 'K',
                (Color::White, Side::Queen) => 'Q',
                (Color::Black, Side::King) => 'k',
                (Color::Black, Side::Queen) => 'q',
            };
            prop_assert_eq!(Castles::char_to_square(ch), Some(sq));
        }
    }
}
