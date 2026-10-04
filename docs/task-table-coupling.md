# Task: table coupling (findings 9, 10, 11)

Session: `table-coupling`. Repo: `/home/tony/programming/crystallographic-group`.
This is the second session of the five-session queue. It runs after
`parser-hardening`. Read `docs/ergonomics-review.md` for the decision
record.

## Goal

Remove the index hazard in the space group database. No hand-maintained
parallel index. No `String` allocation from static data. No `unwrap()` on
table lookups. The 230 reference files and `test_all` must behave exactly
as before. The table content does not change.

## Current shape

- `SpaceGroupHallSymbol` is a hand-written 530-variant enum
  (src/database/mod.rs:14). Each method indexes the parallel tables with
  `*self as usize` (around src/database/mod.rs:548). Insert one variant in
  the wrong place and every index is wrong.
- The same methods allocate `String` from `&'static str` data and call
  `.unwrap()` on `Option`.
- `LookUpSpaceGroup` (src/database/space_group_table.rs:9) is implemented
  for the raw array shape `[[&str; 530]; 3]`. No user can implement it.
- The table statics are named `FULL_SPACE_GROUP_SYMBOLS` (530 entries) and
  `DEFAULT_SPACE_GROUP_SYMBOLS` (230 entries). The names say nothing about
  the content difference.

## Work 1: finding 9, remove the index hazard

Keep the strings as the source of truth. Two designs are on the table:

- Option A: a typed newtype, for example `SpaceGroupNumber(u16)`, that
  indexes the tables directly with bounds checks. The 530-variant enum is
  dropped or kept as a thin wrapper over the typed value.
- Option B: code-generate the enum from the table in a `build.rs` step.

Pick the lower-risk option and document the choice in
`sessions/table-coupling/notes.md`. After your work, no call site may do
`*self as usize` against a hand-maintained variant order. If you keep the
enum, add a test that proves every variant index matches its table
position. If you drop it, update every call site, including the
`From<SpaceGroupHallSymbol>` impl. That impl stays compilable. The next
session in the queue reworks it.

## Work 2: finding 10, no allocation, no unwrap

- Return `&'static str` for symbol lookups, or a typed
  `SpaceGroupNumber(u16)` for numbers. Define the newtype if option A or B
  leaves numbers as strings.
- Remove the `String` allocation from static data and the `unwrap()` calls
  on `Option` in the lookup methods (src/database/mod.rs:548 area). The
  row layout of the tables is fixed. Index it without unwraps.

## Work 3: finding 11, the trait and the table names

- `LookUpSpaceGroup` is closer to an implementation detail than a trait.
  Seal it, make it private, or replace it with concrete methods on an owned
  table type. Pick one and say which in the notes.
- Rename `DEFAULT_` and `FULL_` so the names say what the content
  difference is. Read the two tables first. Do not guess the meaning.
  Document the result in a doc comment on the new names.

## Constraints

- `cargo test --lib` must stay at 16 of 16, including `test_all` against
  `refs/`.
- `test_all` behavior does not change. If a renamed or reworked lookup
  changes what it prints, fix the call site, not the reference files.
- Breaking API changes are approved (D1).

## Definition of done

- `cargo clippy --lib`: zero warnings.
- `cargo test --lib`: 16 of 16, including `test_all`.
- No `*self as usize` index remains against a hand-maintained variant
  order.
- No `String` allocation or `unwrap()` in the table lookup path.
- The design choice and the new table names are documented in
  `sessions/table-coupling/notes.md`.

## Commit

Commit your work when done. Suggested subject:
"Type the space group database lookups and drop the index hazard".

## Status contract

Write state to `sessions/table-coupling/status`:

- `running`: at start.
- `blocked`: add a one-line reason after the word.
- `done`: only when the definition of done holds.
