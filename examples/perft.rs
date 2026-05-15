//! perft benchmark — runs [perft](https://www.chessprogramming.org/Perft) on
//! the six standard test positions and reports node counts and nodes/sec.
//!
//! Usage:
//!     cargo run --release --example perft           # default depths
//!     cargo run --release --example perft -- 5      # cap max depth at 5
//!     cargo run --release --example perft -- start  # only the start position

use std::time::Instant;

use ruchess::{fen, mve::Move, position::Position};

/// (name, FEN, expected node counts for depths 1..=N)
const POSITIONS: &[(&str, &str, &[u64])] = &[
    (
        "start",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &[20, 400, 8_902, 197_281, 4_865_609, 119_060_324],
    ),
    (
        "kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        &[48, 2_039, 97_862, 4_085_603, 193_690_690],
    ),
    (
        "pos3",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        &[14, 191, 2_812, 43_238, 674_624, 11_030_083],
    ),
    (
        "pos4",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        &[6, 264, 9_467, 422_333, 15_833_292],
    ),
    (
        "pos5",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        &[44, 1_486, 62_379, 2_103_487, 89_941_194],
    ),
    (
        "pos6",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        &[46, 2_079, 89_890, 3_894_594, 164_075_551],
    ),
];

fn perft(pos: &mut Position, depth: u32) -> u64 {
    if depth == 1 {
        return pos.valid_moves().count() as u64;
    }
    // Realistic max moves per position is ~218; 256 is a safe upper bound and
    // avoids reallocation under push.
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
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (max_depth, only): (Option<u32>, Option<&str>) =
        args.iter().fold((None, None), |(d, n), a| match a.parse() {
            Ok(parsed) => (Some(parsed), n),
            Err(_) => (d, Some(a.as_str())),
        });
    let only = only.map(|s| s.to_string());

    println!(
        "{:<10} {:>6} {:>15} {:>12} {:>12} ok",
        "position", "depth", "nodes", "ms", "Mn/s"
    );
    println!("{}", "-".repeat(72));

    let mut grand_total: u64 = 0;
    let mut grand_ms: u128 = 0;
    let mut mismatches: Vec<(String, u32, u64, u64)> = Vec::new();

    for (name, fen_str, expected) in POSITIONS {
        if let Some(o) = &only
            && o != name
        {
            continue;
        }
        let pos = fen::parse(fen_str)
            .expect("FEN should parse")
            .without_repetition();
        let depths = expected.len() as u32;
        let cap = max_depth.unwrap_or(depths).min(depths);
        for d in 1..=cap {
            let want = expected[(d - 1) as usize];
            let start = Instant::now();
            let mut working = pos.clone();
            let got = perft(&mut working, d);
            let elapsed = start.elapsed();
            let ms = elapsed.as_millis().max(1);
            let mnps = (got as f64) / (elapsed.as_secs_f64().max(1e-9) * 1e6);
            let ok = if got == want { "✓" } else { "✗" };
            println!(
                "{:<10} {:>6} {:>15} {:>12} {:>12.2} {}{}",
                name,
                d,
                got,
                ms,
                mnps,
                ok,
                if got == want {
                    String::new()
                } else {
                    format!(" (expected {want})")
                }
            );
            grand_total += got;
            grand_ms += elapsed.as_millis();
            if got != want {
                mismatches.push((name.to_string(), d, got, want));
            }
        }
    }

    println!("{}", "-".repeat(72));
    let total_s = (grand_ms as f64) / 1000.0;
    let mnps = if total_s > 0.0 {
        (grand_total as f64) / (total_s * 1e6)
    } else {
        0.0
    };
    println!(
        "total: {} nodes in {:.2}s — {:.2} Mn/s",
        grand_total, total_s, mnps
    );

    if !mismatches.is_empty() {
        println!("\nmismatches (move-generation discrepancies vs canonical):");
        for (name, d, got, want) in &mismatches {
            let diff = *got as i128 - *want as i128;
            println!("  {name:<10} depth {d}: got {got}, expected {want} (diff {diff:+})");
        }
        std::process::exit(1);
    }
}
