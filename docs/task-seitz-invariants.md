# Task: SeitzMatrix invariants (findings 13, 14, 15, 16)

Session: `seitz-invariants`. Repo: `/home/tony/programming/crystallographic-group`.
This is the fourth session of the five-session queue. It runs after
`space-group`. Read `docs/ergonomics-review.md` for the decision record.

## Goal

Make the `SeitzMatrix` invariants hold. `PartialEq`, `Hash`, `Display`,
and the `Add` impls must agree on one representation.

## Context

Translation parts use a 12-fold base. The constant is
`SEITZ_TRANSLATE_BASE_NUMBER` (12). A helper for positive mod is in
`src/utils/mod.rs`. Pick one representation and hold it: positive
residues modulo 12.

## Work 1: finding 13, the hash invariant

`PartialEq` treats translation parts equal modulo 12, but `Hash` hashes
the raw matrix (src/hall_symbols/matrix_symbol/matrices/seitz_mat_impl.rs:16).
The invariant "equal values hash equal" is broken. A
`HashSet<SeitzMatrix>` will hold duplicates that compare equal. Normalize
the translation part before hashing, using the same positive-residue
normalization that `PartialEq` uses.

## Work 2: finding 14, the Display panic

`Display` calls `eigenvector()`, and that method unwraps a search result
(seitz_mat_impl.rs:105, 355). Any `println!("{m}")` can panic. Make
`eigenvector()` return `Option`. Give `Display` a safe fallback for the
`None` case. State the fallback in a doc comment.

## Work 3: finding 15, the wrong Add

`Add` for `SeitzMatrix + SeitzMatrix` adds the rotation parts too
(seitz_mat_impl.rs:308). That is not a group operation. Keep only the
translation addition. Document what the operator means after your change.
Audit the call sites before you change semantics. Update them to the
correct operation if any rely on the old behavior.

## Work 4: finding 16, mixed representations

`SeitzMatrix + Vector3` leaves negative translation values. Other paths
normalize to positive. Normalize this path to the same positive-residue
representation.

## Constraints

- `cargo clippy --lib`: zero warnings.
- `cargo test --lib`: 16 of 16, including `test_all`.
- `test_all` behavior does not change. The reference files are the
  ground truth for the print path.
- Add one test that proves the hash invariant. It takes two matrices
  that are equal modulo 12 in the translation part. It checks that they
  hash equal and that a `HashSet` round-trip holds no duplicate.

## Definition of done

- `eigenvector()` returns `Option`. `Display` never panics.
- `Add` for two `SeitzMatrix` values does not add rotation parts.
- `SeitzMatrix + Vector3` normalizes to positive residues.
- The new hash-invariant test passes.

## Commit

Commit your work when done. Suggested subject:
"Fix the SeitzMatrix hash, Display, and Add invariants".

## Status contract

Write state to `sessions/seitz-invariants/status`:

- `running`: at start.
- `blocked`: add a one-line reason after the word.
- `done`: only when the definition of done holds.
