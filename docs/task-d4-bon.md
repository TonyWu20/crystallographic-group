# Task: D4, replace `MatrixSymbolBuilder` with the `bon` crate

Session: `bon-builder`. Repo: `/home/tony/programming/crystallographic-group`.
Decision record: `docs/ergonomics-review.md`, section "D4: builder design with
bon". This session runs after `findings-pass`. That pass made
`MatrixSymbolError` a lifetime-free, owned-error type. Build on that state.

## Goal

Replace the hand-written `MatrixSymbolBuilder` with `bon`. Keep the test
suite green. `cargo clippy --lib` must end at zero warnings.

## Dependency

Add `bon = "3.10"` to `Cargo.toml`.

## Builder replacement

`src/hall_symbols/matrix_symbol/builder.rs` becomes a generated builder:

- `#[derive(Builder)]` on `MatrixSymbol`, with:
  - `#[builder(default)]` on `minus_sign`, `nfold_sub`, `nfold_diag`,
    `rotation_axis`.
  - Omitting `nfold_body` is a compile error. This replaces the runtime
    `IncompleteFields` error.
  - `translation_symbols: Option<Vec<TranslationSymbol>>` stays skippable and
    defaults to `None`.
- Drop the hand-written `MatrixSymbolBuilder` struct and its `set_*` methods.
- `MatrixSymbol::new_builder()` is the public entry. Keep that name if bon
  supports it, or rename to `builder()` and update the call sites in
  `src/hall_symbols/matrix_symbol/mod.rs` and the parser. Breaking changes
  are approved (D1).

## Cross-field validation

The `new` function builder is fallible and runs the cross-field rules that
`get_rotation_matrix` already encodes. For example, `NFold::N2` plus
`NFoldDiag::Asterisk` is invalid. `NFold::N6` only allows sub-numbers
`None, N1, N2, N4, N5`. Return `Result<MatrixSymbol, MatrixSymbolError>`.

Document which rules are enforced. Keep the rule set consistent with
`get_rotation_matrix`. Do not duplicate that table. Prefer calling into the
existing matrix logic, or extracting one shared rule function.

## Out of scope

- The two `HallParseError` nits. They stay open.
- Findings 5, 8, 9, 10, 12, 13, 14, 15, 16.
- The parser framework.

## Definition of done

- `cargo clippy --lib`: zero warnings.
- `cargo test --lib`: 16 of 16, including `test_all`.
- Omitting `nfold_body` at the builder is a compile error, not a runtime
  error. Prove it with a short note, or a `trybuild`-free manual check, and
  record what you checked.

## Status contract

Write state to `sessions/bon-builder/status`:

- `running`: at start.
- `blocked`: add a one-line reason after the word.
- `done`: only when the definition of done holds.
