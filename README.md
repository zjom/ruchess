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

- [arrayvec](https://docs.rs/arrayvec/latest/arrayvec/index.html) to allocate generated moves on stack

## roadmap

- [x] benchmarks
  - will move to mutable api if benchmarks are very bad (update: working on performance)
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

we generate roughly 212 Mn/s

for reference, [shakmaty](https://github.com/niklasf/shakmaty) generates roughly 223 Mn/s

we are currently about 5% slower.

## license

[unlicense](https://unlicense.org/)
