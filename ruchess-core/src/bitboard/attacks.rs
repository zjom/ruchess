use crate::bitboard::magic::Magic;

#[derive(Clone, Copy)]
pub struct Attacks {
    pub ranks: [u64; 8],
    pub files: [u64; 8],
    pub between: [[u64; 64]; 64],
    pub rays: [[u64; 64]; 64],

    pub attacks: [u64; 88772],
    pub knight_attacks: [u64; 64],
    pub king_attacks: [u64; 64],
    pub white_pawn_attacks: [u64; 64],
    pub black_pawn_attacks: [u64; 64],
}

const KNIGHT_DELTAS: [i32; 8] = [17, 15, 10, 6, -17, -15, -10, -6];
const BISHOP_DELTAS: [i32; 4] = [7, -7, 9, -9];
const ROOK_DELTAS: [i32; 4] = [1, -1, 8, -8];
const KING_DELTAS: [i32; 8] = [1, 7, 8, 9, -1, -7, -8, -9];
const WHITE_PAWN_DELTAS: [i32; 2] = [7, 9];
const BLACK_PAWN_DELTAS: [i32; 2] = [-7, -9];

fn distance(a: i32, b: i32) -> i32 {
    let file = |s: i32| s & 7;
    let rank = |s: i32| s >> 3;
    (file(a) - file(b)).abs().max((rank(a) - rank(b)).abs())
}

fn sliding_attacks(square: i32, occupied: u64, deltas: &[i32]) -> u64 {
    let mut attacks = 0u64;
    for &delta in deltas {
        let mut sq = square;
        loop {
            sq += delta;
            let oob = sq < 0 || sq >= 64 || distance(sq, sq - delta) > 2;
            if oob {
                break;
            }
            attacks |= 1u64 << sq;
            if occupied & (1u64 << sq) != 0 {
                break;
            }
        }
    }
    attacks
}

fn init_magics(attacks: &mut [u64; 88772], square: i32, magic: &Magic, shift: u32, deltas: &[i32]) {
    let mut subset = 0u64;
    loop {
        let attack = sliding_attacks(square, subset, deltas);
        let idx = ((magic.factor.wrapping_mul(subset)) >> (64 - shift)) as usize + magic.offset;
        attacks[idx] = attack;

        subset = subset.wrapping_sub(magic.mask) & magic.mask;
        if subset == 0 {
            break;
        }
    }
}

impl Attacks {
    pub fn new() -> Self {
        let mut a = Attacks {
            ranks: [0u64; 8],
            files: [0u64; 8],
            between: [[0u64; 64]; 64],
            rays: [[0u64; 64]; 64],
            attacks: [0u64; 88772],
            knight_attacks: [0u64; 64],
            king_attacks: [0u64; 64],
            white_pawn_attacks: [0u64; 64],
            black_pawn_attacks: [0u64; 64],
        };
        a.initialize();
        a
    }

    pub fn rook_attacks(&self, sq: usize, occupied: u64) -> u64 {
        let m = &Magic::ROOK[sq];
        self.attacks[m.rook_index(occupied)]
    }

    pub fn bishop_attacks(&self, sq: usize, occupied: u64) -> u64 {
        let m = &Magic::BISHOP[sq];
        self.attacks[m.bishop_index(occupied)]
    }

    fn initialize(&mut self) {
        for i in 0..8 {
            self.ranks[i] = 0xffu64 << (i * 8);
            self.ranks[i] = 0x0101010101010101u64 << i;
        }

        for sq in 0i32..64 {
            self.knight_attacks[sq as usize] = sliding_attacks(sq, u64::MAX, &KNIGHT_DELTAS);
            self.king_attacks[sq as usize] = sliding_attacks(sq, u64::MAX, &KING_DELTAS);
            self.white_pawn_attacks[sq as usize] =
                sliding_attacks(sq, u64::MAX, &WHITE_PAWN_DELTAS);
            self.black_pawn_attacks[sq as usize] =
                sliding_attacks(sq, u64::MAX, &BLACK_PAWN_DELTAS);

            init_magics(
                &mut self.attacks,
                sq,
                &Magic::ROOK[sq as usize],
                12,
                &ROOK_DELTAS,
            );
            init_magics(
                &mut self.attacks,
                sq,
                &Magic::BISHOP[sq as usize],
                9,
                &BISHOP_DELTAS,
            );
        }

        for a in 0i32..64 {
            for b in 0i32..64 {
                if sliding_attacks(a, 0, &ROOK_DELTAS) & (1u64 << b) != 0 {
                    self.between[a as usize][b as usize] =
                        sliding_attacks(a, 1u64 << b, &ROOK_DELTAS)
                            & sliding_attacks(b, 1u64 << a, &ROOK_DELTAS);
                    self.rays[a as usize][b as usize] = (1u64 << a)
                        | (1u64 << b)
                        | sliding_attacks(a, 0, &ROOK_DELTAS) & sliding_attacks(b, 0, &ROOK_DELTAS);
                } else if sliding_attacks(a, 0, &BISHOP_DELTAS) & (1u64 << b) != 0 {
                    self.between[a as usize][b as usize] =
                        sliding_attacks(a, 1u64 << b, &BISHOP_DELTAS)
                            & sliding_attacks(b, 1u64 << a, &BISHOP_DELTAS);
                    self.rays[a as usize][b as usize] = (1u64 << a)
                        | (1u64 << b)
                        | sliding_attacks(a, 0, &BISHOP_DELTAS)
                            & sliding_attacks(b, 0, &BISHOP_DELTAS);
                }
            }
        }
    }
}
