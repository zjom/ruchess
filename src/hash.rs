//! # Zobrist Hashing
//!
//! Position fingerprints used for repetition detection and (eventually)
//! transposition tables. Each [`Position`] gets a 24-bit [`Hash`], computed
//! by XOR-folding precomputed random values for every (color, role, square),
//! the side to move, castling rights, and the en-passant file.
//!
//! [`PositionHash`] stacks several `Hash` values into a packed byte trail —
//! used by [`History`](crate::history::History) to test for threefold and
//! fivefold repetition without retaining whole positions.

use crate::{
    color::{ByColor, Color},
    piece::Piece,
    position::Position,
    role::{ByRole, Role},
};

/// Zobrist hash of a [`Position`], truncated to 24 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash(u32);

impl Hash {
    /// Number of bytes a single `Hash` occupies inside a [`PositionHash`].
    pub const SIZE: usize = 3;

    /// Wraps an arbitrary 32-bit value, keeping only its top 24 bits.
    pub fn new(value: u32) -> Self {
        Self(value >> 8)
    }

    /// Computes the Zobrist hash of `position`, truncated to 24 bits.
    pub fn from_position(position: &Position) -> Self {
        Self(hash_position(position) >> 8)
    }

    /// Returns the underlying 24-bit value (in the low bits of a `u32`).
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Concatenated 3-byte Zobrist hashes, ordered from most-recent to oldest,
/// or a `Disabled` sentinel that suppresses repetition tracking entirely.
///
/// Each `Hash` contributes [`Hash::SIZE`] bytes. Entries `[0..3)` correspond
/// to the current position; entries at offsets `Hash::SIZE * 2`, `Hash::SIZE * 4`,
/// ... correspond to earlier positions with the same side to move.
///
/// The `Disabled` variant skips the hash computation and `Vec` traffic on
/// every move — used by perft and search loops that never query repetition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionHash {
    /// Repetition tracking is off. `combine` is a no-op, `is_repetition`
    /// always returns false (for `times > 1`).
    Disabled,
    /// A growing byte trail of 3-byte hashes.
    Trail(Vec<u8>),
}

impl PositionHash {
    /// Wraps an existing byte vector. The caller is responsible for ensuring
    /// the length is a multiple of [`Hash::SIZE`] if repetition detection is
    /// to behave correctly.
    pub fn new(value: Vec<u8>) -> Self {
        Self::Trail(value)
    }

    /// Returns an empty trail with no entries.
    pub fn empty() -> Self {
        Self::Trail(Vec::new())
    }

    /// Returns a `Disabled` trail — repetition tracking is permanently off
    /// for any chain of histories descending from this value.
    pub fn disabled() -> Self {
        Self::Disabled
    }

    /// Builds a single-entry trail containing the three high bytes of `h`,
    /// in big-endian order.
    pub fn from_hash(h: Hash) -> Self {
        Self::Trail(vec![(h.0 >> 16) as u8, (h.0 >> 8) as u8, h.0 as u8])
    }

    /// Returns the raw byte buffer. Empty when `Disabled`.
    pub fn value(&self) -> &[u8] {
        match self {
            Self::Disabled => &[],
            Self::Trail(v) => v,
        }
    }

    /// Returns `true` if no positions have been recorded (including `Disabled`).
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Disabled => true,
            Self::Trail(v) => v.is_empty(),
        }
    }

    /// Returns `true` if repetition tracking is turned off on this trail.
    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Prepends `self`'s entries with `other`'s, producing a trail that
    /// keeps `self` (most recent) at the front.
    ///
    /// If either side is `Disabled`, the result is `Disabled` — repetition
    /// tracking, once off, stays off for descendants.
    #[must_use]
    pub fn combine(self, other: &PositionHash) -> Self {
        match (self, other) {
            (Self::Disabled, _) | (_, Self::Disabled) => Self::Disabled,
            (Self::Trail(mut v), Self::Trail(o)) => {
                v.extend_from_slice(o);
                Self::Trail(v)
            }
        }
    }

    /// Whether the leading hash has appeared at least `times` times in total
    /// (i.e. `times - 1` additional times among the trailing entries with the
    /// same side to move).
    pub fn is_repetition(&self, times: usize) -> bool {
        if times <= 1 {
            return true;
        }
        let data = match self {
            Self::Disabled => return false,
            Self::Trail(v) => v,
        };
        let len = data.len();
        if len <= (times - 1) * 4 * Hash::SIZE {
            return false;
        }
        let x = data[0];
        let y = data[1];
        let z = data[2];
        let mut i = Hash::SIZE * 2;
        let mut count = 0usize;
        while i + Hash::SIZE <= len && count < times - 1 {
            if x == data[i] && y == data[i + 1] && z == data[i + 2] {
                count += 1;
            }
            i += Hash::SIZE * 2;
        }
        count == times - 1
    }
}

/// Computes the full 32-bit Zobrist hash for a position by XOR-folding the
/// per-(color, role, square) masks, the side-to-move mask, the four castling
/// masks, and the en-passant file mask.
fn hash_position(position: &Position) -> u32 {
    let mut h: u32 = 0;

    for color in [Color::White, Color::Black] {
        let color_sub = ACTOR_MASKS.get(color);
        for role in [
            Role::Pawn,
            Role::Rook,
            Role::Knight,
            Role::Bishop,
            Role::Queen,
            Role::King,
        ] {
            let sub = color_sub.get(role);
            for sq in position.board().bypiece(Piece { role, color }) {
                h ^= sub[sq.0 as usize];
            }
        }
    }

    if position.color() == Color::White {
        h ^= WHITE_TURN_MASK;
    }

    let castles = position.history().castles;
    if castles.white_king_side() {
        h ^= CASTLING_MASKS.white[0];
    }
    if castles.white_queen_side() {
        h ^= CASTLING_MASKS.white[1];
    }
    if castles.black_king_side() {
        h ^= CASTLING_MASKS.black[0];
    }
    if castles.black_queen_side() {
        h ^= CASTLING_MASKS.black[1];
    }

    if let Some(sq) = position.enpassant_square() {
        h ^= EN_PASSANT_MASKS[sq.file().as_u8() as usize];
    }

    h
}

const ACTOR_MASKS: ByColor<ByRole<[u32; 64]>> = ByColor {
    black: ByRole {
        pawn: [
            0x9d39_247e, 0x2af7_3980, 0x44db_0150, 0x9c15_f73e, 0x7583_4465, 0x3290_ac3a,
            0x0fbb_ad1f, 0xe83a_908f, 0x0d7e_765d, 0x1a08_3822, 0x9605_d5f0, 0xd021_ff5c,
            0x40bd_f15d, 0x0113_5514, 0x5db4_8320, 0x239f_8b2d, 0x05d1_a1ae, 0x679f_848f,
            0x7449_bbff, 0x7d11_cdb1, 0x82c7_709e, 0xf321_8f1c, 0x3314_78f3, 0x4bb3_8de5,
            0xaa64_9c6e, 0x8dbd_98a3, 0x87d2_074b, 0x19f3_c751, 0xb4ab_30f0, 0x7b05_00ac,
            0xc945_2ca8, 0x24aa_6c51, 0x4c9f_3442, 0x14a6_8fd7, 0xa71b_9b83, 0x0348_8b95,
            0x637b_2b34, 0x09d1_bc9a, 0x3575_6683, 0x735e_2b97, 0x1872_7070, 0x1fcb_acd2,
            0xd310_a7c2, 0xbf98_3fe0, 0x9f74_d14f, 0x51eb_dc4a, 0x5c82_c505, 0xfcf7_fe8a,
            0x3253_a729, 0x8c74_c368, 0xb9bc_6c87, 0x7ef4_8f2b, 0x11d5_05d4, 0x6568_fca9,
            0x4de0_b0f4, 0x96d6_9346, 0x42e2_40cb, 0x6d2b_dcda, 0x4288_0b02, 0x5f0f_4a58,
            0x39f8_90f5, 0x93c5_b5f4, 0x63dc_359d, 0xec16_ca8a,
        ],
        knight: [
            0x5643_6c9f, 0xefac_4b70, 0xbb21_5798, 0x45f2_0042, 0x930f_80f4, 0xff67_12ff,
            0xae62_3fd6, 0xdd2c_5bc8, 0x7eed_120d, 0x22fe_5454, 0xc918_00e9, 0x808b_d68e,
            0xdec4_6814, 0x1bed_e3a3, 0x4353_9603, 0xaa96_9b5c, 0xa878_32d3, 0x6594_2c7b,
            0xded2_d633, 0x21f0_8570, 0xb415_938d, 0x91b8_59e5, 0x10cf_f333, 0x28ae_d140,
            0xc5cc_1d89, 0x5648_f680, 0x2d25_5069, 0x9bc5_a38e, 0xef2f_0543, 0xaf20_42f5,
            0x4804_12ba, 0xaef3_af4a, 0x19af_e59a, 0x5259_3803, 0xf4f0_76e6, 0x1137_9625,
            0xbce5_d224, 0x9da4_243d, 0x066f_70b3, 0x4dc4_de18, 0x5103_9ab7, 0xc07a_3f80,
            0xb46e_e9c5, 0xb381_9a42, 0x21a0_0793, 0x2df1_6f76, 0x763c_4a13, 0xf793_c467,
            0xd728_8e01, 0xde33_6a2a, 0x0bf6_92b3, 0x2c60_4a7a, 0x4850_e73e, 0xcfc4_47f1,
            0xb05c_a3f5, 0x9ae1_82c8, 0xa4fc_4bd4, 0xe755_178d, 0x69b9_7db1, 0xf9b5_b7c4,
            0xfc6a_82d6, 0x9c68_4cb6, 0x8ec9_7d29, 0x6703_df9d,
        ],
        bishop: [
            0x7f9b_6af1, 0x5862_7e1a, 0x2cd1_6e2a, 0xd363_eff5, 0x0ce2_a38c, 0x1a80_4aad,
            0x907f_3042, 0x501f_65ed, 0x3762_4ae5, 0x957b_af61, 0x3a6c_2793, 0xd495_0353,
            0x088e_0495, 0xf943_aee7, 0x6c3b_8e3e, 0x364f_6ffa, 0xd60f_6dce, 0x5696_3b0d,
            0x16f5_0edf, 0xef19_5591, 0x5656_01c0, 0xecb5_3939, 0xbac7_a9a1, 0xb344_c470,
            0x65d3_4954, 0xb4b8_1b3f, 0xb422_0611, 0x0715_8240, 0x7a13_f18b, 0xbc40_97b1,
            0x59b9_7885, 0x9917_0a5d, 0x6f42_3357, 0x3259_28ee, 0xd0e4_3662, 0x565c_31f7,
            0x30f5_6114, 0xd873_db39, 0x7bd9_4e1d, 0xc7d9_f168, 0x947a_e053, 0xc8c9_3882,
            0x3a9b_f55b, 0xd9a1_1fbb, 0x0fd2_2063, 0xb3f2_56d8, 0xb030_31a8, 0x35dd_37d5,
            0xe9f6_082b, 0xebfa_fa33, 0x9255_abb5, 0xb9ab_4ce5, 0x6935_01d6, 0xc62c_58f9,
            0xcd45_4f8f, 0xbbe8_3f4e, 0xdc84_2b7e, 0xba89_142e, 0xa3bc_941d, 0xe9f6_760e,
            0x09c7_e552, 0x852f_5493, 0x8107_fccf, 0x0989_54d5,
        ],
        rook: [
            0xda3a_361b, 0xdcdd_7d20, 0x3683_3336, 0xce68_341f, 0xab90_9016, 0x4395_4b32,
            0xb438_c2b6, 0x10dc_d78e, 0xdbc2_7ab5, 0x9b3c_db65, 0xb67b_7896, 0xbfce_d1b0,
            0xa911_9b60, 0x1fff_7ac8, 0xac12_fb17, 0xaf08_da91, 0x1b0c_ab93, 0xb559_eb1d,
            0xc37b_45b3, 0xc3a9_dc22, 0xf3b8_b667, 0x9fc4_77de, 0x6737_8d8e, 0x6dd8_56d9,
            0xa319_ce15, 0x0739_7375, 0x8a8e_849e, 0xe192_5c71, 0x74c0_4bf1, 0x4dda_4815,
            0x9d26_6d6a, 0x7440_fb81, 0x1332_8503, 0xd6bf_7bae, 0x4838_d65f, 0x1e15_2328,
            0x8f84_19a3, 0x72c8_834a, 0xd7a0_23a7, 0x94eb_c8ab, 0x9fc1_0d0f, 0xde68_a235,
            0xa44c_fe79, 0x9d1d_84fc, 0x51d2_b1ab, 0x2fd7_e4b9, 0x65ca_5b96, 0xdd69_a0d8,
            0x604d_51b2, 0x73aa_8a56, 0x1a8c_1e99, 0xaac4_0a27, 0x764d_beae, 0x1e99_b96e,
            0x2c5e_9deb, 0x3a93_8fee, 0x26e6_db8f, 0x4693_56c5, 0xc876_3c5b, 0x3f6c_6af8,
            0x7f7c_c394, 0x9bfb_227e, 0x8903_9d79, 0x8fe8_8b57,
        ],
        queen: [
            0x001f_837c, 0x1877_b51e, 0xa285_3b80, 0x993e_1de7, 0xb359_8080, 0x252f_59cf,
            0xd23c_8e17, 0x1bda_0492, 0x21e0_bd50, 0x3b09_7ada, 0x8d14_dedb, 0xf95c_ffa2,
            0x3871_7007, 0xca67_2b91, 0x64c8_e531, 0x2412_60ed, 0x106c_09b9, 0x7fba_1954,
            0x7884_d9bc, 0x0647_dfed, 0x6357_3ff0, 0x4fc8_e956, 0x1db9_56e4, 0xb8d9_1274,
            0xa2eb_ee47, 0xd9f1_f30c, 0xefed_53d7, 0x2e6d_02c3, 0xa9aa_4d20, 0xb64b_e8d8,
            0x70cb_6af7, 0x98f0_76a4, 0xbf84_4708, 0x94c3_251f, 0x3e00_3e61, 0xb925_a6cd,
            0x61bd_d130, 0xbf8d_5108, 0x240a_b57a, 0xfc87_614b, 0xef02_cdd0, 0xa108_2c04,
            0x8215_e577, 0xd39b_b9c3, 0x2738_2596, 0x61cf_4f94, 0x1b6b_aca2, 0x758f_450c,
            0x959f_587d, 0xb063_e962, 0x60e8_ed72, 0x7b64_9785, 0xfd08_0d23, 0x8c90_fd9b,
            0x106f_72fe, 0x7976_033a, 0xa4ec_0132, 0x733e_a705, 0xb4d8_f77b, 0x9e21_f4f9,
            0x9d76_5e41, 0xd30c_088b, 0x5d94_337f, 0x1a4e_4822,
        ],
        king: [
            0x230e_343d, 0x43ed_7f5a, 0x3a88_a0fb, 0x2187_4b8b, 0x1bde_a12e, 0x53c0_65c6,
            0xe34a_1d25, 0xd6b0_4d3b, 0x5e90_277e, 0x2c04_6f22, 0xb10b_b459, 0x3fa9_ddfb,
            0x0e09_b88e, 0x10e8_b35a, 0x9eed_eca8, 0xd4c7_18bc, 0x8153_6d60, 0x91b5_34f8,
            0xec81_77f8, 0x190e_714f, 0xb592_bf39, 0x89c3_50c8, 0xac04_2e70, 0xb49b_52e5,
            0xfb15_2fe3, 0x3e66_6e6f, 0x3b54_4ebe, 0xe805_a1e2, 0x24b3_3c9d, 0xe747_3342,
            0x0a80_4d18, 0x57e3_306d, 0x4ae7_d6a3, 0x2d8d_5432, 0xd1e6_49de, 0x8a32_8a1c,
            0x07a3_aec7, 0x8454_7ddc, 0x990a_98fd, 0x1a4f_f126, 0xf6f7_fd14, 0x30c0_5b1b,
            0x8d26_36b8, 0x46c9_feb5, 0xccec_0a73, 0x4e9d_2827, 0x19eb_b029, 0x4659_d2b7,
            0x963e_f2c9, 0x74f8_5198, 0x5a0f_544d, 0x0372_7073, 0xc7f6_aa2d, 0x3527_87ba,
            0x9853_eab6, 0xabbd_cdd7, 0xcf05_daf5, 0x49ca_d48c, 0x7a4c_10ec, 0xd9e9_2aa2,
            0x13ae_978d, 0x7304_99af, 0x4e4b_705b, 0xff57_7222,
        ],
    },
    white: ByRole {
        pawn: [
            0x5355_f900, 0x07fb_9f85, 0x5093_417a, 0x7bcb_c38d, 0x19fc_8a76, 0x637a_7780,
            0x8249_a47a, 0x79ad_6955, 0x14ac_baf4, 0xf145_b6be, 0xdabf_2ac8, 0x24c3_c94d,
            0xbb6e_2924, 0x0ce2_6c0b, 0xa49c_d132, 0xe99d_662a, 0x27e6_ad78, 0x8535_f040,
            0x54b3_f4fa, 0x72b1_2c32, 0xee95_4d3c, 0x9a85_ac90, 0x70ac_4cd9, 0xf9b8_9d3e,
            0x87b3_e2b2, 0xa366_e5b8, 0xae4a_9346, 0x1920_c04d, 0x87bf_02c6, 0x0922_37ac,
            0xff07_f64e, 0x8de8_dca9, 0x9c16_3326, 0xb3f2_2c3d, 0x390e_5fb4, 0x5bfe_a5b4,
            0x1e10_3291, 0x9a74_acb9, 0x4f80_f7a0, 0x6304_d09a, 0x2171_e646, 0x5b9b_63eb,
            0x506a_acf4, 0x1881_afc9, 0x6503_0804, 0xdfd3_9533, 0xef92_7dbc, 0x7b32_f7d1,
            0xb9fd_7620, 0x05a7_e8a5, 0xb588_9c6e, 0x4a75_0a09, 0xcf46_4cec, 0xf538_639c,
            0x3c79_a0ff, 0xede6_c87f, 0x799e_81f0, 0x8653_6b8c, 0x97d7_374c, 0xa246_637c,
            0x043f_cae6, 0x920e_4495, 0x70eb_093b, 0x73a1_9219,
        ],
        knight: [
            0xc547_f57e, 0x78e3_7644, 0xfe9a_44e9, 0x08bd_35cc, 0x9315_e5eb, 0x9406_1b87,
            0xdf1d_9f9d, 0x3bba_57b6, 0xd2b7_adee, 0xf7a2_55d8, 0xd7f4_f244, 0xd95b_e88c,
            0x336f_52f8, 0xa740_49da, 0xa2f6_1bb6, 0x4f2a_5cb0, 0x87d3_80bd, 0x16b9_f7e0,
            0x7ba2_484c, 0xf3a6_78ca, 0x39b0_bf7d, 0xfcaf_55c1, 0x18fc_f680, 0x4c05_63b8,
            0x40e0_8793, 0x8cff_a941, 0x68ca_3905, 0x7a1e_e967, 0x9d1d_60e5, 0x3810_e399,
            0x3209_5b6d, 0x35ca_b621, 0xa90b_2449, 0x77a2_25a0, 0x513e_5e63, 0x4361_c0ca,
            0xd941_aca4, 0x528f_7c86, 0x52ab_92be, 0x9d1d_fa2e, 0x722f_f175, 0x1d12_60a5,
            0x7a24_9a57, 0x0420_8fe9, 0x5a11_0c60, 0x0cd9_a497, 0x56fd_23c8, 0x284c_847b,
            0x04fe_abfb, 0x742e_1e65, 0x9a96_32e6, 0x881b_82a1, 0x506e_6744, 0xb018_3db5,
            0x0ed9_b915, 0x5e11_e86d, 0xf678_647e, 0x1b85_d488, 0xdab9_fe65, 0x0d15_1d86,
            0xa865_a54e, 0x93c4_2566, 0x99e7_afea, 0x48cb_ff08,
        ],
        bishop: [
            0x23b7_0edb, 0xc330_de42, 0x4715_ed43, 0xa8d7_e4da, 0x0572_b974, 0xb57d_2e98,
            0xe8d9_ecbe, 0x2fe4_b171, 0x1131_7ba8, 0x7fbf_21ec, 0x1725_cabf, 0x964e_915c,
            0x3e2b_8bcb, 0xbe74_44e3, 0xf85b_2b4f, 0x4935_3fea, 0x1dd0_1aaf, 0x1fca_8a92,
            0xfc7c_95d8, 0x18a6_a990, 0xcccb_7005, 0x3bdb_b92c, 0xaa70_b5b4, 0xe94c_39a5,
            0xb7a0_b174, 0xd4db_a847, 0x2e18_bc1a, 0x2de0_966d, 0xb9c1_1d5b, 0x6497_2d68,
            0x9462_8d38, 0xdbc0_d2b6, 0xd273_3c43, 0x7e75_d99d, 0x6ced_1983, 0x97fc_aacb,
            0x7b77_497b, 0x8547_eddf, 0x7999_9cdf, 0xcffe_1939, 0x8296_26e3, 0x92fa_e242,
            0x63e2_2c14, 0xc678_b6d8, 0x5873_8888, 0x0981_dcd2, 0x9f65_789a, 0x9ff3_8fed,
            0xe479_ee5b, 0xe7f2_8ecd, 0x56c0_74a5, 0x5544_f7d7, 0x7b3f_0195, 0x1215_3635,
            0x7f51_26db, 0x7a76_956c, 0x3d57_74a1, 0x8a1b_0838, 0x7b4a_38e3, 0x9501_1364,
            0x4da8_979a, 0x3bc3_6e07, 0x5d0a_12f2, 0x7f9d_1a2e,
        ],
        rook: [
            0xa09e_8c8c, 0xfa7e_3939, 0xd6b6_d0ec, 0xdfea_21ea, 0xb67c_1fa4, 0xca1e_3785,
            0x1cfc_8bed, 0xd18d_8549, 0x4ed0_fe7e, 0xe4db_f063, 0x1761_f93a, 0x5389_8e4c,
            0x734d_e818, 0x2680_b122, 0x298a_f231, 0x7983_eed3, 0x66c1_a2a1, 0x9e17_e496,
            0xedb4_54e7, 0x50b7_04ca, 0x4cc3_17fb, 0x66b4_835d, 0x219b_97e2, 0x261e_4e4c,
            0x1fe2_cca7, 0xd750_4dfa, 0xb957_1fa0, 0x1ddc_0325, 0xcf3f_4688, 0xf4f5_d05c,
            0x38b6_525c, 0x36f6_0e2b, 0xeb35_9380, 0x9c4c_d625, 0xaf0c_317d, 0x258e_5a80,
            0x8b88_9d62, 0xf4d1_4597, 0xd434_7f66, 0xe699_ed85, 0x2472_f620, 0xc2a1_e7b5,
            0xab4f_6451, 0x6376_7572, 0xa59e_0bd1, 0x116d_0016, 0x2cf9_c8ca, 0x0b09_0a75,
            0xabee_ddb2, 0x58ef_c10b, 0xc6e5_7a78, 0x2eab_8ca6, 0x14a1_9564, 0x7c08_28dd,
            0xd74b_be77, 0x8044_56af, 0xebe9_ea2a, 0x0321_9a39, 0x4978_7fef, 0xa1e9_300c,
            0x5b45_e522, 0xb49c_3b39, 0xd449_0ad5, 0x12a8_f216,
        ],
        queen: [
            0x6ffe_73e8, 0xddf9_57bc, 0x64d0_e29e, 0x08dd_9bdf, 0x087e_79e5, 0xe328_e230,
            0x1c25_59e3, 0x720b_f5f2, 0xb077_4d26, 0x443f_64ec, 0x4112_cf68, 0xd813_f2fa,
            0x660d_3257, 0x59ac_2c78, 0xe846_9638, 0x93b6_33ab, 0xc0c0_f5a6, 0xcaf2_1ecd,
            0x5727_7707, 0x506c_11b9, 0xd83c_c268, 0x4a29_c646, 0xed2d_f212, 0xb563_5c95,
            0x22af_003a, 0x52e7_6259, 0x9aeb_a33a, 0x944f_6de0, 0x6c47_bec8, 0x6ad0_47c4,
            0xa5b1_cfdb, 0x7c45_d833, 0x5092_ef95, 0x9338_e69c, 0x455a_4b4c, 0x6b02_e631,
            0x6b17_b224, 0xd1e0_ccd2, 0xde0c_89a5, 0x5006_5e53, 0x9c11_69fa, 0x78ed_efd6,
            0x6dc9_3d95, 0xee97_f453, 0x32ab_0edb, 0x3a68_53c7, 0x3186_5ced, 0x67fe_f95d,
            0x1f2b_1d1f, 0xb69e_38a8, 0xaa91_19ff, 0xf43c_7328, 0xfb4a_3d79, 0x3550_c232,
            0x371f_77e7, 0x6bfa_9aae, 0xcd04_f3ff, 0xe327_3522, 0x9f91_508b, 0x049a_7f41,
            0xfcb6_be43, 0x08de_8a1c, 0x8f98_87e6, 0xb5b4_071d,
        ],
        king: [
            0x55b6_344c, 0xb862_225b, 0xcac0_9afb, 0xdaf8_e982, 0xb5fd_fc5d, 0x310c_b380,
            0xe87f_bb46, 0x2102_ae46, 0xf854_9e1a, 0x07a6_9afd, 0xc4c1_18bf, 0xf9f4_892e,
            0x1af3_dbe2, 0xf5b4_b0b0, 0x962a_ceef, 0x046e_3eca, 0xf05d_1296, 0x9647_81ce,
            0x9c2e_d440, 0x522e_23f3, 0x177e_00f9, 0x2bc6_0a63, 0x222b_bfae, 0x4862_89dd,
            0x7dc7_785b, 0x8af3_8731, 0x1fab_64ea, 0xe4d9_4293, 0x9da0_58c6, 0x24c0_e332,
            0x2330_03b5, 0xd586_bd01, 0x5e56_3788, 0x7eba_726d, 0x0a56_a5f0, 0xd794_76a8,
            0x9e4c_1269, 0x17ef_ee45, 0x1d95_b0a5, 0x93cb_e0b6, 0x65fa_4f22, 0xd5f9_e858,
            0xc2b5_a03f, 0x5930_0222, 0xce2f_8642, 0x7ca9_723f, 0x2785_3383, 0xc61b_b3a1,
            0x150f_361d, 0x9f6a_419d, 0x64a5_3dc9, 0x142d_e49f, 0x0c33_5248, 0x0a9c_32d5,
            0xe6c4_2178, 0x71f1_ce24, 0xf1bc_c3d2, 0xe728_e8c8, 0x96fb_f83a, 0x81a1_549f,
            0x5fa7_867c, 0x5698_6e2e, 0x917f_1dd5, 0xd20d_8c88,
        ],
    },
};

const WHITE_TURN_MASK: u32 = 0xf8d6_26aa;

const CASTLING_MASKS: ByColor<[u32; 2]> = ByColor {
    black: [0xa57e_6339, 0x1ef6_e6db],
    white: [0x31d7_1dce, 0xf165_b587],
};

const EN_PASSANT_MASKS: [u32; 8] = [
    0x70cc_73d9, 0xe21a_6b35, 0x003a_93d8, 0x1c99_ded3, 0xcf31_45de, 0xd0e4_427a, 0x77c6_21cc,
    0x67a3_4dac,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_drops_low_byte() {
        assert_eq!(Hash::new(0xAABB_CCDD).value(), 0x00AA_BBCC);
    }

    #[test]
    fn hash_fits_in_24_bits() {
        let h = Hash::from_position(&Position::new());
        assert_eq!(h.value() & 0xFF00_0000, 0);
    }

    #[test]
    fn position_hash_from_hash_is_three_bytes() {
        // Hash::new shifts right by 8: 0x11_22_33_44 → stored as 0x00_11_22_33.
        // PositionHash::from_hash emits the bottom three bytes high-to-low.
        let bytes = PositionHash::from_hash(Hash::new(0x11_22_33_44));
        assert_eq!(bytes.value(), &[0x11, 0x22, 0x33]);
    }

    #[test]
    fn position_hash_empty_is_empty() {
        assert!(PositionHash::empty().is_empty());
    }

    #[test]
    fn combine_concatenates_bytes() {
        let a = PositionHash::new(vec![1, 2, 3]);
        let b = PositionHash::new(vec![4, 5, 6]);
        assert_eq!(a.combine(&b).value(), &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn is_repetition_trivial_for_zero_or_one() {
        let p = PositionHash::empty();
        assert!(p.is_repetition(0));
        assert!(p.is_repetition(1));
    }

    #[test]
    fn is_repetition_false_when_too_short() {
        let p = PositionHash::new(vec![1, 2, 3]);
        assert!(!p.is_repetition(2));
    }

    #[test]
    fn is_repetition_detects_threefold() {
        // Leading hash [1,2,3] then alternating same-side hashes; we want 3
        // total occurrences, i.e. 2 additional matches at offsets 6 and 12.
        // The length gate requires len > (times-1)*4*SIZE = 24 bytes for times=3.
        let mut data = vec![
            1, 2, 3, // current (side X)
            9, 9, 9, // other side
            1, 2, 3, // match #1 (side X, offset 6)
            8, 8, 8, // other side
            1, 2, 3, // match #2 (side X, offset 12)
            7, 7, 7, // other side
        ];
        data.extend(std::iter::repeat_n(0u8, 12));
        let p = PositionHash::new(data);
        assert!(p.is_repetition(3));
        assert!(!p.is_repetition(4));
    }

    #[test]
    fn starting_position_hash_is_stable() {
        let h1 = Hash::from_position(&Position::new());
        let h2 = Hash::from_position(&Position::new());
        assert_eq!(h1, h2);
    }
}
