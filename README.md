# ruchess

rust chess

[docs](https://docs.rs/ruchess)


## quickstart

> [!CAUTION]
> this is library is still at version 0.0.X
> it is very likely that there will be breaking api changes until version 0.1.0 release
> if you use this library, make sure you pin the version in cargo.toml


```sh
cargo add ruchess
```

or in your `cargo.toml`

```toml
[dependencies]
ruchess = "0.0.3"
```


```rust
use ruchess::game::Game;
use ruchess::square;
                                                                                
// Play 1.e4 from the standard starting position.
let game = Game::new();
let after_e4 = game.mve(square::E2, square::E4).unwrap();
assert!(after_e4.position().board().is_occupied(square::E4));
```

see [docs](https://docs.rs/ruchess) for more library/module details

see [examples](https://github.com/zjom/ruchess/tree/main/examples) or [ruchess tui](https://github.com/zjom/ruchess/tree/main/tui) for example applications


## why

i like chess

i currently only support standard chess. it is unlikely i will support other variants.

probably check out [shakmaty](https://github.com/niklasf/shakmaty) if you want more features

## dependencies

- [lazy_static](https://docs.rs/lazy_static/latest/lazy_static/)


## roadmap

- [ ] benchmarks
    - will move to mutable api if benchmarks are very bad
- [ ] fen parsing
- [ ] real uci support
