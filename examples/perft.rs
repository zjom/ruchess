//! perft benchmark — runs [perft](https://www.chessprogramming.org/Perft) on
//! the six standard test positions and reports node counts and nodes/sec.
//!
//! Usage:
//!     cargo run --release --example perft           # default depths
//!     cargo run --release --example perft -- 5      # cap max depth at 5
//!     cargo run --release --example perft -- start  # only the start position

use ruchess::{fen, mve::Move, position::Position};
use std::time::{Duration, Instant};

struct TestPosition<'a> {
    name: &'a str,
    fen: &'a str,
    expected_nodes: &'a [u64],
}

const POSITIONS: &[TestPosition] = &[
    TestPosition {
        name: "start",
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        expected_nodes: &[20, 400, 8_902, 197_281, 4_865_609, 119_060_324],
    },
    TestPosition {
        name: "kiwipete",
        fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        expected_nodes: &[48, 2_039, 97_862, 4_085_603, 193_690_690],
    },
    TestPosition {
        name: "pos3",
        fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        expected_nodes: &[14, 191, 2_812, 43_238, 674_624, 11_030_083],
    },
    TestPosition {
        name: "pos4",
        fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        expected_nodes: &[6, 264, 9_467, 422_333, 15_833_292],
    },
    TestPosition {
        name: "pos5",
        fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        expected_nodes: &[44, 1_486, 62_379, 2_103_487, 89_941_194],
    },
    TestPosition {
        name: "pos6",
        fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        expected_nodes: &[46, 2_079, 89_890, 3_894_594, 164_075_551],
    },
];

#[derive(Default)]
struct RunConfig {
    max_depth: Option<u32>,
    target_position: Option<String>,
}

impl RunConfig {
    fn from_args() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let mut config = RunConfig::default();

        for arg in args {
            if let Ok(depth) = arg.parse::<u32>() {
                config.max_depth = Some(depth);
            } else {
                config.target_position = Some(arg);
            }
        }
        config
    }
}

struct Mismatch {
    name: String,
    depth: u32,
    got: u64,
    expected: u64,
}

fn perft(pos: &mut Position, depth: u32) -> u64 {
    if depth == 1 {
        return pos.valid_moves().count() as u64;
    }

    // Realistic max moves per position is ~218; 256 is a safe upper bound
    let mut moves: Vec<Move> = Vec::with_capacity(256);
    moves.extend(pos.valid_moves());

    let mut total: u64 = 0;
    for m in &moves {
        let undo = pos.make(m);
        total += perft(pos, depth - 1);
        pos.unmake(undo);
    }
    total
}

fn main() {
    let config = RunConfig::from_args();

    println!(
        "{:<10} {:>6} {:>15} {:>12} {:>12} ok",
        "position", "depth", "nodes", "ms", "Mn/s"
    );
    println!("{}", "-".repeat(72));

    let mut grand_total: u64 = 0;
    let mut total_duration = Duration::ZERO;
    let mut mismatches: Vec<Mismatch> = Vec::new();

    for pos_data in POSITIONS {
        if let Some(target) = &config.target_position
            && target != pos_data.name
        {
            continue;
        }

        let pos = fen::parse(pos_data.fen)
            .expect("Hardcoded FEN should always parse")
            .without_repetition();

        let available_depths = pos_data.expected_nodes.len() as u32;
        let cap = config
            .max_depth
            .unwrap_or(available_depths)
            .min(available_depths);

        for depth in 1..=cap {
            let expected_nodes = pos_data.expected_nodes[(depth - 1) as usize];

            let start = Instant::now();
            let mut working_pos = pos.clone();
            let actual_nodes = perft(&mut working_pos, depth);
            let elapsed = start.elapsed();

            grand_total += actual_nodes;
            total_duration += elapsed;

            let ms = elapsed.as_millis().max(1);
            let mnps = (actual_nodes as f64) / (elapsed.as_secs_f64().max(1e-9) * 1e6);
            let is_match = actual_nodes == expected_nodes;

            let status_mark = if is_match { "✓" } else { "✗" };
            let error_msg = if is_match {
                String::new()
            } else {
                format!(" (expected {})", expected_nodes)
            };

            println!(
                "{:<10} {:>6} {:>15} {:>12} {:>12.2} {}{}",
                pos_data.name, depth, actual_nodes, ms, mnps, status_mark, error_msg
            );

            if !is_match {
                mismatches.push(Mismatch {
                    name: pos_data.name.to_string(),
                    depth,
                    got: actual_nodes,
                    expected: expected_nodes,
                });
            }
        }
    }

    print_summary(grand_total, total_duration, &mismatches);
}

fn print_summary(total_nodes: u64, total_duration: Duration, mismatches: &[Mismatch]) {
    println!("{}", "-".repeat(72));

    let total_s = total_duration.as_secs_f64();
    let mnps = if total_s > 0.0 {
        (total_nodes as f64) / (total_s * 1e6)
    } else {
        0.0
    };

    println!(
        "total: {} nodes in {:.2}s — {:.2} Mn/s",
        total_nodes, total_s, mnps
    );

    if !mismatches.is_empty() {
        println!("\nmismatches (move-generation discrepancies vs canonical):");
        for m in mismatches {
            let diff = m.got as i128 - m.expected as i128;
            println!(
                "  {:<10} depth {}: got {}, expected {} (diff {:+})",
                m.name, m.depth, m.got, m.expected, diff
            );
        }
        std::process::exit(1);
    }
}
