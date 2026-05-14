# ruchess

rust chess

[docs](https://docs.rs/ruchess)

## features

- highly compact bitboard representations
- zobrist hashing
- magic attack tables
- move generation & validation
- outcome evaluation (checkmate, stalemate, repetition, insufficient material)
- fen parsing
- immutable
- hella [docs](https://docs.rs/ruchess)
- really really really fast (relatively)

i only support standard chess. it is unlikely i will support other variants.

probably check out [shakmaty](https://github.com/niklasf/shakmaty) if you want more features

## quickstart

> [!CAUTION]
> this is library is still at version `0.0.X`
>
> it is very likely that there will be breaking api changes until version `0.1.0`
>
> if you use this library, make sure you pin the version in cargo.toml

```sh
cargo add ruchess
```

or in your `cargo.toml`

```toml
[dependencies]
ruchess = "0.0.4"
```

```rust
use ruchess::game::Game;
use ruchess::square;

// Play 1.e4 from the standard starting position.
let game = Game::new();
let after_e4 = game.mve(square::E2, square::E4).unwrap();
assert!(after_e4.position().board().is_occupied(square::E4));
```

see: [docs](https://docs.rs/ruchess) for more details

see: [examples](https://github.com/zjom/ruchess/tree/main/examples) or [ruchess tui](https://github.com/zjom/ruchess/tree/main/tui) for example applications

## dependencies

just [lazy_static](https://docs.rs/lazy_static/latest/lazy_static/)

i could probably get rid of it with some refactoring but i am lazy

## roadmap

- [ ] benchmarks
  - will move to mutable api if benchmarks are very bad
- [ ] parsing
  - [x] fen
  - [ ] pgn
  - [ ] san
  - [ ] uci
- [ ] real uci support
- [ ] finalise public api

## benchmarks

simple and benchmarking using [perft](https://www.chessprogramming.org/Perft) to get an estimation of performance

results in [benchmark_results/](benchmark_results)

code in [examples/perft.rs](examples/perft.rs)

benchmarks were ran on m1 macbook pro

at depth=4 and depth=5, we are at 7ms and 177ms respectively.

for reference, [shakmaty](https://github.com/niklasf/shakmaty) and [jordanbray/chess](https://github.com/jordanbray/chess) are at 0.8 - 1ms for depth=4 and 18.6 - 24.1ms for depth=5.
we are currently an order of magnitude slower.

it is worth noting that the shakmaty and jordanbray benchmarks weren't run on my machine but still (sort of) indicative.

## license

[unlicense](https://unlicense.org/)
