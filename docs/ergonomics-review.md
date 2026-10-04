# Ergonomics review: crystallographic-group

Status: draft. Decisions are recorded in the table below as they are made.

## Recorded decisions

| ID | Decision | Status |
|----|----------|--------|
| D1 | Breaking API changes are acceptable. Target version 0.4.0. | Approved |
| D2 | Commit `refs/*.txt` so `test_all` passes out of the box. | Done (230 files, 16/16 tests pass) |
| D3 | Parser framework: `chumsky` 0.13 + `ariadne` 0.6 (Path B). | Decided |
| D4 | Replace the hand-written `MatrixSymbolBuilder` with the `bon` crate. | Done (session `bon-builder`, design in the D4 section) |
| D5 | Monitoring of delegated long tasks: event-driven, no poll loop. Standard form: `nohup bash -c 'rushi run task "task" && rushi run main "done" || rushi run main "failed"'`. The back message uses the start form, no `--no-run`: the main session is stopped when the task ends, and only the start form wakes it. `--no-run` appends to a live loop only. | Decided, corrected |
| D6 | Execution plan: run the findings pass and D4 as two headless sessions, chained in that order. Both share files, so they must not run in parallel. | Decided |
| D7 | Finding 4: delete the `From<&str>` and `From<&char>` impls for `NFoldSub`. They map unknown input to `None` silently. The parser maps digits directly. | Decided |
| D8 | Finding 7: promote `restore_information_in_matrix_symbols` to a public, documented function. Re-export it at the `hall_symbols` root. | Decided |

## Baseline (verified 2026-10-04)

- `cargo build` passes with 4 warnings. All 4 come from the lifetime in `MatrixSymbolError`.
- `cargo test --lib`: 16 of 16 tests pass, including `test_all` against `refs/`.
- No `examples/`, no integration tests.

## Findings, priority 1: entry points

1. `HallSymbolNotation::try_from_str` returns `winnow::ModalResult` (src/hall_symbols/mod.rs:56). A user must depend on `winnow` just to name the error type, or call `.unwrap()`. Decide with D3: Path A keeps the leak unless we wrap the error. Path B handles rich errors inside the framework.
2. `MatrixSymbol::try_from_str` takes `&mut &str`, but `HallSymbolNotation::try_from_str` takes `&str`. Pick one public shape. `&str` is the better default.
3. `MatrixSymbolError<'a>` borrows `&'a MatrixSymbol` (src/hall_symbols/matrix_symbol/mod.rs:107). A borrowing error type is unusual. It is the source of all 4 build warnings. Store an owned copy instead.
4. `NFoldSub::from` maps any unknown input to `None` (src/hall_symbols/matrix_symbol/notations/mod.rs:51). This is silent data loss. Return a `Result` at the parse boundary.

## Findings, priority 1: the central type

5. The crate goal is to assemble a space group by picking elements. The user must instead parse a string, then call `general_positions()` on the parse result. Add one `SpaceGroup` type as the public center. It holds the lattice, the generator symbols, and the origin shift. It exposes the number, the HM symbol, the `CrystalSystem`, and the positions. Keep `HallSymbolNotation` as the parse form.
6. `MatrixSymbolBuilder::build()` only checks that `nfold_body` is present (src/hall_symbols/matrix_symbol/builder.rs:51). It never checks cross-field validity. A user can build an invalid symbol and learn of it later in `seitz_matrix()`. Move validation into the build step (see the D4 section).
7. `restore_information_in_matrix_symbols` (src/hall_symbols/parser.rs:25) is the rule set for implied axes. It is private to the parser path. A user who builds symbols by hand must rediscover these rules. Promote the rules to a public, documented function.
8. `impl From<SpaceGroupHallSymbol> for HallSymbolNotation` (src/hall_symbols/mod.rs:208) round-trips through a string and calls `.unwrap()`. A bad table entry becomes a panic. Build the notation from structured fields instead.

## Findings, priority 1: table coupling

9. `SpaceGroupHallSymbol` is a 530-variant hand-written enum (src/database/mod.rs:14). Each method indexes the parallel table with `*self as usize`. Insert one variant in the wrong place and every index is wrong. Keep the strings as the source of truth. Use a typed `SpaceGroupRef`, or a code-generated enum from a `build.rs` step.
10. The same methods allocate `String` from `&'static str` data and call `.unwrap()` on `Option` (src/database/mod.rs:548). Return `&'static str`, or better a typed `SpaceGroupNumber(u16)`.
11. `LookUpSpaceGroup` is implemented for the raw array shape `[[&str; 530]; 3]` (src/database/space_group_table.rs:9). No user can implement it. It is closer to an implementation detail than a trait. The `DEFAULT_` vs `FULL_` names (230 vs 530 entries) say nothing about the difference in content.
12. `CrystalSystem` exists but nothing connects to it. There is no way to ask which crystal system a group belongs to. Add `crystal_system()` to the reference type and a filter by system.

## Findings, priority 2: invariants in `SeitzMatrix`

13. `PartialEq` treats translation parts equal modulo 12, but `Hash` hashes the raw matrix (src/hall_symbols/matrix_symbol/matrices/seitz_mat_impl.rs:16). The hash invariant is broken. A `HashSet<SeitzMatrix>` will hold duplicates that compare equal. Normalize the translation part before hashing.
14. `Display` calls `eigenvector()`, and that method `unwrap()`s a search result (seitz_mat_impl.rs:105, 355). Any `println!("{m}")` can panic. Make `eigenvector()` return `Option` and give `Display` a safe fallback.
15. `Add` for `SeitzMatrix + SeitzMatrix` adds the rotation parts too (seitz_mat_impl.rs:308). That is not a group operation. Keep only the translation addition.
16. `SeitzMatrix + Vector3` leaves negative translation values. Other paths normalize to positive. Pick one representation and hold it.

## Findings, priority 2: documentation and examples

17. No public item has a doc comment. The most important fact for users is the 12-fold translation base (`SEITZ_TRANSLATE_BASE_NUMBER`). Put it in the crate-level doc.
18. Names need a pass. `jones_faithful_repr` is internal jargon. Suggest `formula()` as the public name. `pure_txt` and `text_format` overlap. `num_of_general_pos` reads better as `len`. `matrice` is a typo used in several places.
19. `utils` is a `pub mod` whose items are all `pub(crate)` (src/utils/mod.rs). Make the module private. Also drop the crate-root `#![allow(dead_code)]` (src/lib.rs:1). It hides real dead code such as the `todo!()` in `read_from_refs`.
20. Add one `examples/` that shows both entry paths: parse a Hall symbol, and assemble a group from typed parts. That example is the ergonomics test.

## D3: parser framework decision

Two paths. Both are checked against the current code (2026-10-04).

### Path A: winnow 0.7.6 to 1.0.4 (minimal, verified in the working tree)

- `Cargo.toml`: `winnow = { version = "1.0.4", features = ["ascii", "parser"] }`.
- One code edit: the 10-element `alt` tuple in `parse_sign_fold` no longer implements `Alt`. Nest it into two groups of five (src/hall_symbols/matrix_symbol/parser.rs:72).
- All 16 tests pass. No other change needed.

Limits: `ModalResult` still leaks `winnow` into the public API (finding 1). The hand-rolled `StrContext` error code in `matrix_symbol/parser.rs` stays.

### Path B: chumsky 0.13 + ariadne 0.6 (rewrite, style match)

Your `castep_cell_fmt` uses `chumsky 0.10.1` + `ariadne 0.6.0`. Latest stable is `chumsky 0.13.0`. The castep patterns work here:

- Dot-chained combinators: `choice((...)).padded_by(...).repeated().collect().parse(input)`.
- Rich errors: `extra::Err<Rich<'a, char>>`, `parse(input).into_result()` gives `Result<T, Vec<Rich<'a, char>>>`.
- Diagnostics: `ariadne` `Report`/`Label`/`Source` with byte-index spans (your `rich_error` helper).

Mapping of the Hall grammar:

- `LatticeSymbol`: optional `-`, then `one_of("PABCIRF")`.
- `MatrixSymbol`: optional `-`, one of `12346`, optional axis or diagnostic char (`xyz"'`), optional sub-number digit, optional translation chars (`abcnuvwd`).
- Repeated symbols: `matrix_symbol().repeated()`. It stops at the origin shift `( i i i )` without an error.
- `OriginShift`: `just('(')` then three integers.
- `restore_information_in_matrix_symbols` stays as pure post-processing. It does not depend on the parser crate.

Cost and risk:

- Rewrite of 4 parser files, about 330 lines. The grammar is linear and non-recursive, which fits chumsky well.
- `chumsky` is still 0.x. Your own ADR (castep-cell-io `docs/adr/0002-chumsky-parser.md`) records the same risk. Verify the rewrite on 0.13.0, not 0.10.1.
- `ariadne` adds spanned diagnostics that replace the hand-rolled `StrContext` messages.

### Recommendation

Decision D3: Path B. The user picked `chumsky` + `ariadne` (Path B). The reason is the dot-chain style preference. The grammar shape also fits chumsky. Path A (`winnow` 1.0.4) stays in the working tree as a verified fallback.

Work order:

- First, run the `chumsky` + `ariadne` migration. Delegate it to a new headless session.
- After the migration, re-inspect findings 1, 2, 3, 6. They are parser-coupled, so re-check them against the new framework.
- Then implement D4 and the remaining findings in a second pass.

## Migration brief (task for the headless session)

The headless session gets this task. It replaces `winnow` with `chumsky` + `ariadne` and keeps the suite green.

- `Cargo.toml`: drop `winnow`. Add `chumsky = "0.13"` and `ariadne = "0.6"`.
- Rewrite the 4 parser files with chumsky combinators. Use `extra::Err<Rich<char>>` for spans. Keep `restore_information_in_matrix_symbols` as pure post-processing.
- Wrap the public parse entry in a crate-owned error type. Do not leak `Rich` or `ariadne` into the public API yet. That is the re-inspection step.
- Definition of done: `cargo test --lib` passes 16 of 16, including `test_all` against `refs/`.
- The session writes its own progress to `sessions/<name>/`. The main session pings it with `rushi run`.

## D4: builder design with bon

`bon` 3.10.2 (MSRV 1.88). Two parts.

Field checks (turn `IncompleteFields` from a runtime error into a compile error):

```rust
use bon::Builder;

#[derive(Builder)]
struct MatrixSymbol {
    #[builder(default)]
    minus_sign: bool,
    nfold_body: NFold,
    #[builder(default)]
    nfold_sub: NFoldSub,
    #[builder(default)]
    nfold_diag: NFoldDiag,
    #[builder(default)]
    rotation_axis: RotationAxis,
    translation_symbols: Option<Vec<TranslationSymbol>>,
}
```

- Omitting `nfold_body` is a compile error.
- `translation_symbols` is `Option`. It stays `None` when not set. This matches the current builder semantics.

Cross-field validation (finding 6), as a function builder that returns `Result`:

```rust
#[bon::builder]
fn new(
    #[bon::builder(default)] minus_sign: bool,
    nfold_body: NFold,
    #[bon::builder(default)] nfold_sub: NFoldSub,
    #[bon::builder(default)] nfold_diag: NFoldDiag,
    #[bon::builder(default)] rotation_axis: RotationAxis,
    translation_symbols: Option<Vec<TranslationSymbol>>,
) -> Result<MatrixSymbol, MatrixSymbolError> {
    /* cross-field rules, e.g. NFold::N2 + NFoldDiag::Asterisk is invalid */
}
```

The generated `build()` returns the `Result`. Call it after the typestate checks pass.

## D3 migration outcome (session `chumsky-migration`, 2026-10-04)

Done. Status: `sessions/chumsky-migration/status`.

- `Cargo.toml`: `winnow` removed. Added `chumsky = "0.13"` (0.13.0) and
  `ariadne = "0.6"` (0.6.0) with `features = ["auto-color"]`.
- Rewrote the 4 parser files in dot-chain form with
  `extra::Err<Rich<'a, char>>` spans. `restore_information_in_matrix_symbols`
  is unchanged pure post-processing.
- New crate-owned error type `HallParseError<'a>`
  (`src/hall_symbols/errors.rs`, re-exported as
  `crate::hall_symbols::HallParseError`). It holds the `Rich` errors and the
  input privately. One ariadne helper renders a `Rich` error with byte-index
  spans (`IndexType::Byte`). `Rich` and ariadne types stay out of the public
  signatures.
- All `try_from_str` entries now take `&str` and return
  `Result<Self, HallParseError>`. The `&mut &str` stream shape is gone
  (finding 2 resolved: `&str` everywhere).
- Findings 1 and 3 re-inspected: the public API no longer names `winnow`.
  `HallParseError` is crate-owned. The 4 build warnings still come from
  `MatrixSymbolError<'a>` (finding 3, untouched by this task).

Behavior changes against winnow, for the re-inspection pass:

- `chumsky` `Parser::parse` enforces end-of-input. Trailing content after
  the origin shift is now a parse error. Winnow ignored it.
- A shift with 4 or more integers, or a `(` with no closing `)`, is now
  rejected. Winnow silently fell back to `(0 0 0)` there.
- A `(` holding 1 to 2 integers was already a hard error under winnow. It
  still is, now with one custom `Rich` message.
- `chumsky` 0.13 `validate` never fails. It only emits errors. Hard
  failures come from a `custom` parser that returns a `Rich` error.
- 0.13 API notes that shaped the code:
  - `repeated()` outputs `()`. Use `collect::<Vec<T>>()` for items.
  - `then` yields tuples. Use `ignore_then` to drop the right side.
  - There are no tuple parsers. Chain `then`.
  - `parse` appends `end()`.

Suite state: `cargo test --lib` 16 of 16, including `test_all`.
`cargo build` at the 4-warning baseline (all from `MatrixSymbolError`).
No `winnow::` reference remains. No test was weakened.

## Re-inspection of findings 1, 2, 3, 6 (main session, 2026-10-04)

Done after the migration, as agreed.

- Finding 1 (error leak): resolved. Both parse entries take `&str` and return
  `Result<_, HallParseError>`. `HallParseError` implements
  `std::error::Error`. `Rich` and ariadne stay out of the public signatures.
- Finding 2 (`&mut &str` vs `&str`): resolved. `&str` everywhere.
- Finding 3 (`MatrixSymbolError<'a>`): still open. It still borrows
  `&'a MatrixSymbol`. It is the source of all 4 build warnings. Fix: store an
  owned `MatrixSymbol` in the variant.
- Finding 6 (builder validation): still open. `MatrixSymbolBuilder` only
  checks that `nfold_body` is present. No cross-field validation. D4
  addresses it. The new chumsky parse path is safe on its own. It always sets
  `nfold_body`, so its `build().expect` cannot fail.

Two new nits from the migration:

- `HallParseError<'a>` borrows the input. It cannot outlive that input. That
  limits storing the error in a longer-lived struct.
- `HallParseError::print` writes to stdout. A library should return the
  rendered text, or write to stderr. The helper also uses `.expect`, a panic
  on an error path.

## Open items

- D3 is decided (Path B). The migration is done and verified. See the D3
  migration outcome section.
- `findings-pass` session: done. See the findings pass outcome section.
  Brief: `docs/task-findings-pass.md`.
- `bon-builder` session: done. See the D4 outcome section.
  Brief: `docs/task-d4-bon.md`.
- Each session pokes this session on exit, per D5. The pokes land in
  `sessions/<task>/notify.log`.
- Both sessions are done and verified in the main session: clippy at zero
  warnings, 16 of 16 tests, owned-value error, no rule-table duplication.
- Finding 4 status: the chumsky rewrite closed it at the parse boundary.
  The parser only accepts digits 12345, so the unknown-input path is
  unreachable there. D7 removes the remaining silent `From` impls.
- Queue: five headless sessions run in one serial chain, per D5 and D6.
  Each step is followed by a `cargo test --lib` and clippy gate. The
  final step pokes this session.

  1. `parser-hardening`: the two `HallParseError` nits, D7, D8. Brief:
     `docs/task-parser-hardening.md`.
  2. `table-coupling`: findings 9, 10, 11. Brief:
     `docs/task-table-coupling.md`.
  3. `space-group`: findings 5, 8, 12. Brief:
     `docs/task-space-group.md`.
  4. `seitz-invariants`: findings 13 to 16. Brief:
     `docs/task-seitz-invariants.md`.
  5. `docs-and-example`: findings 17 to 20. Brief:
     `docs/task-docs-and-example.md`.
- The order puts table coupling before `SpaceGroup`. The new center type
  exposes the typed reference values that the table work produces.

## Findings pass outcome

- Finding 3 is closed. `MatrixSymbolError` owns a `MatrixSymbol` and has
  no lifetime parameter. The `Invalid(self)` sites now clone.
- The `path_statements` and `unnecessary_unwrap` warnings are gone.
- The const/static pair is now two `pub static` items. I checked both
  directions. All-const fires `clippy::large_const_arrays` on the 530-entry
  `FULL_SPACE_GROUP_SYMBOLS`. The `static` pair keeps `cargo clippy --lib`
  silent.
- End state: zero `cargo clippy --lib` warnings, 16 of 16 tests pass,
  including `test_all`. D4 is not started. The `bon-builder` session takes
  it next.

## D4 outcome (session `bon-builder`, 2026-10-04)

Done. Status: `sessions/bon-builder/status`. Check record:
`sessions/bon-builder/notes.md`.

- `Cargo.toml`: added `bon = "3.10"`. The lock file resolved to 3.10.2.
  `Cargo.lock` is git-ignored, so the lock entry stays local.
- `MatrixSymbol` now carries `#[derive(bon::Builder)]`.
  `minus_sign`, `nfold_sub`, `nfold_diag`, and `rotation_axis` have
  `#[builder(default)]`. The typestate builder needs `nfold_body`.
  Omitting it is a compile error. `translation_symbols` stays skippable.
  An omitted value is `None`.
- The hand-written `MatrixSymbolBuilder` and its `set_*` methods are gone.
  `MatrixSymbol::new_builder()` keeps its name. It returns the generated
  typestate builder and wraps `MatrixSymbol::builder()`.
- New fallible entry: `MatrixSymbol::new()` returns the generated
  `NewBuilder`. Its `build()` returns
  `Result<MatrixSymbol, MatrixSymbolError>` and runs the cross-field
  rules through the existing `get_rotation_matrix` and `set_transform`.
  No rule table is duplicated. The rules are documented on
  `MatrixSymbol::from_parts` in `matrix_symbol/builder.rs`.
- `MatrixSymbolError::IncompleteFields` is removed. The positional form
  of the validated constructor is `MatrixSymbol::from_parts(...)`.
- Verification: omitting `nfold_body` failed to compile with E0277
  (recorded in the session notes). `NFold::N2` plus
  `NFoldDiag::Asterisk` builds to `Err`. `NFold::N6` plus
  `NFoldSub::N4` builds to `Ok`. `NFold::N6` plus `NFoldSub::N3`
  builds to `Err`.
- End state: `cargo build` clean, `cargo clippy --lib` zero warnings,
  `cargo test --lib` 16 of 16 including `test_all`. No test was added
  or weakened.

## Space group outcome (session `space-group`, 2026-10-05)

Done. Status: `sessions/space-group/status`. Notes:
`sessions/space-group/notes.md`.

- `SpaceGroup` is the public center. It is re-exported at the crate
  root. It holds the lattice, the generators, and the origin shift,
  plus its all-settings table row.
- Entry paths: `SpaceGroup::try_from_str` (parse plus table lookup,
  `SpaceGroupError::Parse` or `SpaceGroupError::NotInTable`),
  `SpaceGroup::new` (structured parts), and
  `From<HallSymbolNotation>`.
- Finding 5 is closed. `number()`, `hm_symbol()`, `crystal_system()`,
  and `general_positions()` return owned or `&'static` values.
- Finding 8 is closed. The panicking `From` impl is now a `TryFrom`
  with `HallParseError`. A bad table entry is a `Result`, not a
  panic.
- Finding 12 is closed. `CrystalSystem::of_number` maps the standard
  ITC ranges 1-2, 3-15, 16-74, 75-142, 143-167, 168-194, 195-230.
  `SpaceGroupTable::rows_of` filters by system.
  `SpaceGroupHallSymbol::crystal_system` exposes the system on the
  reference type.
- The range split was verified against the table content. Every row's
  lattice letter belongs to its system's allowed set.
- End state: `cargo clippy --lib` zero warnings, `cargo test --lib`
  22 of 22 including `test_all`. Six new tests sit on top of the 16
  baseline tests. No baseline test was weakened or removed.
