# Task: docs, renames, and the entry-path example (findings 17-20)

Session: `docs-and-example`. Repo: `/home/tony/programming/crystallographic-group`.
This is the fifth and last session of the five-session queue. It runs
after `seitz-invariants`. The `SpaceGroup` type from the
`space-group` session is available. Read `docs/ergonomics-review.md` for
the decision record.

## Goal

Document the public API, clean up the names, make the module layout
honest, and add one example that shows both entry paths.

## Work 1: finding 17, documentation

- Add doc comments to every public item. Start with the crate root.
- The crate-level doc states the 12-fold translation base
  (`SEITZ_TRANSLATE_BASE_NUMBER`). It is the most important fact for
  users of the matrix types.

## Work 2: finding 18, the renames

- `jones_faithful_repr` is internal jargon. Rename it to `formula()`.
- `pure_txt` and `text_format` overlap. Read the call sites, then merge
  or rename to one name. State which in
  `sessions/docs-and-example/notes.md`.
- `num_of_general_pos` reads better as `len`.
- `matrice` is a typo used in several places. Fix it.
- Update every call site. Breaking changes are approved (D1).

## Work 3: finding 19, honest module layout

- `utils` is a `pub mod` whose items are all `pub(crate)`
  (src/utils/mod.rs). Make the module private.
- Drop the crate-root `#![allow(dead_code)]` (src/lib.rs:1). It hides
  real dead code.
- `read_from_refs` (src/hall_symbols/mod.rs:360) is a `todo!()` in a
  test helper. Check its callers. Implement it or delete it. Record the
  choice in the notes.

## Work 4: finding 20, one example

- Add one file under `examples/`.
- It shows both entry paths: parse a Hall symbol string, and assemble a
  group from typed parts with the builder and `SpaceGroup`.
- Print the group number, the HM symbol, the crystal system, and the
  general positions for one group.

## Constraints

- `cargo clippy --all-targets`: zero warnings. This lints the example
  too.
- `cargo test --lib`: 16 of 16, including `test_all`.
- `cargo build --examples` succeeds.
- The renames do not change `test_all` output. The reference files stay
  the ground truth.

## Definition of done

- Every public item has a doc comment. The crate doc states the 12-fold
  base.
- No name in the finding-18 list remains.
- `utils` is not `pub`. The crate root has no `allow(dead_code)`.
  No `todo!()` remains.
- The example builds and runs.

## Commit

Commit your work when done. Suggested subject:
"Document the public API and add the entry-path example".

## Status contract

Write state to `sessions/docs-and-example/status`:

- `running`: at start.
- `blocked`: add a one-line reason after the word.
- `done`: only when the definition of done holds.
