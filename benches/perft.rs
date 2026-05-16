//! criterion perft benchmark — runs [perft](https://www.chessprogramming.org/Perft)
//! on the six standard test positions and reports nodes/sec via
//! [`Throughput::Elements`].
//!
//! Usage:
//!     cargo bench --bench perft
//!     cargo bench --bench perft -- start         # only the start position
//!     cargo bench --bench perft -- kiwipete/4    # one specific position+depth

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ruchess::{fen, position::Position};

struct TestPosition {
    name: &'static str,
    fen: &'static str,
    expected_nodes: &'static [u64],
    bench_depth: u32,
}

const POSITIONS: &[TestPosition] = &[
    TestPosition {
        name: "start",
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        expected_nodes: &[20, 400, 8_902, 197_281, 4_865_609, 119_060_324],
        bench_depth: 5,
    },
    TestPosition {
        name: "kiwipete",
        fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        expected_nodes: &[48, 2_039, 97_862, 4_085_603, 193_690_690],
        bench_depth: 4,
    },
    TestPosition {
        name: "pos3",
        fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        expected_nodes: &[14, 191, 2_812, 43_238, 674_624, 11_030_083],
        bench_depth: 6,
    },
    TestPosition {
        name: "pos4",
        fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        expected_nodes: &[6, 264, 9_467, 422_333, 15_833_292],
        bench_depth: 5,
    },
    TestPosition {
        name: "pos5",
        fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        expected_nodes: &[44, 1_486, 62_379, 2_103_487, 89_941_194],
        bench_depth: 4,
    },
    TestPosition {
        name: "pos6",
        fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        expected_nodes: &[46, 2_079, 89_890, 3_894_594, 164_075_551],
        bench_depth: 4,
    },
];

fn perft(pos: &mut Position, depth: u32) -> u64 {
    let moves = pos.valid_moves();

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut total: u64 = 0;
    for m in &moves {
        let undo = pos.make(m);
        total += perft(pos, depth - 1);
        pos.unmake(undo);
    }
    total
}

fn bench_perft(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft");

    for pd in POSITIONS {
        let depth = pd.bench_depth;
        let pos = fen::parse(pd.fen)
            .expect("hardcoded FEN should always parse")
            .without_repetition();
        let nodes = pd.expected_nodes[(depth - 1) as usize];

        group.throughput(Throughput::Elements(nodes));
        group.bench_with_input(BenchmarkId::new(pd.name, depth), &pos, |b, p| {
            b.iter_batched_ref(
                || p.clone(),
                |pos| perft(pos, depth),
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_perft);
criterion_main!(benches);
