# humans

[![crates.io](https://img.shields.io/crates/v/humans.svg)](https://crates.io/crates/humans)
[![docs.rs](https://docs.rs/humans/badge.svg)](https://docs.rs/humans)

Human-facing primitives for the [userspace.party](https://userspace.party) ecosystem.

## Role

`humans` collects concepts at the boundary between computation and human perception. It is a `no_std` crate built on `ample`, with the current code organized around audible and visual domains.

The present surface includes:

- `audible` primitives, including music-related structures;
- `visual` primitives, including color-related structures;
- types intended to be reused by presentation layers such as `webspace`.

Keeping these concepts separate from the browser or operating-system layer allows human-interface semantics to remain reusable across targets.

## Use

```bash
cargo add humans
```

The crate uses `alloc` but does not require the Rust standard library.

## Ecosystem

- Ecosystem: https://userspace.party
- Crate homepage: https://userspace.party/humans
- API documentation: https://docs.rs/humans
- crates.io: https://crates.io/crates/humans
- Source: https://github.com/ze-gois/rust_humans
- Workspace hub: https://github.com/ze-gois/rust_userspace_hub

## Status

Experimental. Audible and visual primitives are being grown incrementally as concrete consumers require them.

## License

See [LICENSE](LICENSE).
