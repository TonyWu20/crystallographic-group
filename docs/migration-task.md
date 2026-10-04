# Task: migrate the Hall-symbol parser from winnow to chumsky + ariadne

Session: `chumsky-migration`. Repo: `/home/tony/programming/crystallographic-group`.
Decision record: `docs/ergonomics-review.md`, D3 = Path B.

## Goal

Replace the `winnow` parser with `chumsky` + `ariadne`. Keep all 16 lib tests
green at every step. `test_all` checks every space group against `refs/`.
Do not regress it.

## Dependency changes

- Remove `winnow` from `Cargo.toml`.
- Add `chumsky = "0.13"` and `ariadne = "0.6"` (with `features = ["auto-color"]`).
- Drop the `features = ["ascii", "parser"]` winnow line.

## Parser rewrites

The 4 parser files to rewrite with chumsky combinators:

- `src/hall_symbols/lattice_symbol/parser.rs`
- `src/hall_symbols/matrix_symbol/parser.rs`
- `src/hall_symbols/origin_shift/parser.rs`
- `src/hall_symbols/parser.rs`

Style reference: your `castep_cell_fmt` crate
(`/home/tony/programming/castep-cell-io/castep_cell_fmt/src/parser.rs`).
Use the dot-chain form `a().b().c().parse(input)`. Use
`extra::Err<Rich<'a, char>>` for spanned errors.

Grammar mapping:

- `LatticeSymbol`: optional `-`, then one of `PABCIRF`.
- `MatrixSymbol`: optional `-`, one of `12346`, optional axis or diagnostic
  char (`x y z " ' *`), optional sub-number digit, optional translation chars
  (`abcnuvwd`).
- Repeated symbols: `matrix_symbol().repeated()`. It stops at the origin shift.
- `OriginShift`: `(` then up to 3 integers, then `)`.

Keep `restore_information_in_matrix_symbols` (src/hall_symbols/parser.rs) as
pure post-processing. It does not depend on the parser crate.

## Error handling

- Wrap the parse entry in a crate-owned error type. Do not leak `Rich` or
  `ariadne` into the public API yet. That boundary design is the re-inspection
  step for the main session.
- Provide one ariadne helper that renders a `Rich` error with byte-index spans.
- No panic on parse failure. Return the error.

## Out of scope

- Findings 1, 2, 3, 6 from the review. The main session re-inspects them after
  this migration.
- The `bon` builder (D4). The table module. `SeitzMatrix` invariants.

## Definition of done

- `cargo test --lib` passes 16 of 16, including `test_all`.
- `cargo build` adds no new warnings.
- `winnow` is fully removed. No `winnow::` reference remains.
- The suite is the guard. Do not weaken a test to make it pass.

## Status contract

Write state to `sessions/chumsky-migration/status` as you go:

- `running`: at start.
- `blocked`: append a one-line reason after the word.
- `done`: only when the definition of done holds.
