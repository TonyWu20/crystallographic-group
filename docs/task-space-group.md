# Task: the SpaceGroup center type (findings 5, 8, 12)

Session: `space-group`. Repo: `/home/tony/programming/crystallographic-group`.
This is the third session of the five-session queue. It runs after
`table-coupling`. Build on the typed lookup state that session left.
Read `docs/ergonomics-review.md` for the decision record.

## Goal

The crate goal is to assemble a space group by picking elements. Today the
user must parse a string and call `general_positions()` on the parse
result. Add one public center type: `SpaceGroup`. It holds the lattice,
the generator symbols, and the origin shift. It exposes the number, the
HM symbol, the `CrystalSystem`, and the positions. `HallSymbolNotation`
stays the parse form.

## The type

```rust
pub struct SpaceGroup {
    lattice: LatticeSymbol,
    generators: Vec<MatrixSymbol>,
    origin_shift: OriginShift,
}
```

- Put it where the public center belongs (a new module or the
  `hall_symbols` root). Re-export it at the crate level.
- Construction: from structured parts, and from a parsed
  `HallSymbolNotation`.
- String entry: `try_from_str(&str)` or `from_str` that parses the Hall
  symbol and looks the database for number, HM symbol, and crystal
  system. The parse error is `HallParseError`. A symbol that is not in
  the table is its own error case. Design that error. No `unwrap()` and
  no panic on a missing entry.

## Methods

- `number()` returns the typed space group number from the
  `table-coupling` session (not a `String`).
- `hm_symbol()` returns the Hermann-Maurer full symbol.
- `crystal_system()` returns `CrystalSystem`
  (src/database/crystal_system.rs). This closes finding 12. See the
  system mapping below.
- `general_positions()` delegates to the existing
  `HallSymbolNotation::general_positions()`.

## Finding 8: structured construction

`impl From<SpaceGroupHallSymbol> for HallSymbolNotation`
(src/hall_symbols/mod.rs:208) round-trips through a string and calls
`.unwrap()`. A bad table entry becomes a panic. Build the notation from
the structured fields instead: the lattice symbol, the generator symbols
as values (not re-parsed strings), and the origin shift. If the
`table-coupling` session restructured the enum, use its fields.

## Finding 12: crystal system

`CrystalSystem` exists but nothing connects to it. Add
`crystal_system()` to the reference type from the `table-coupling`
session, and a way to filter space groups by system. Map the space group
number to the seven systems with the standard number ranges. Verify the
ranges against the table content before trusting them. Add a test that
covers one group per system and the range boundaries.

## Constraints

- `cargo clippy --lib`: zero warnings.
- `cargo test --lib`: 16 of 16, including `test_all`.
- Keep `HallSymbolNotation` working as the parse form. Do not remove it.
- Breaking API changes are approved (D1).

## Definition of done

- `SpaceGroup` is reachable from the crate root.
- Both entry paths work: parse a Hall symbol string, and assemble
  structured parts.
- `number()`, `hm_symbol()`, `crystal_system()`, and
  `general_positions()` all return owned or `&'static` values. No
  `String` allocation for static data in the new path.
- The `From` impl for `HallSymbolNotation` no longer parses a string or
  calls `unwrap()`.
- The crystal system tests pass.

## Commit

Commit your work when done. Suggested subject:
"Add the SpaceGroup type as the public center".

## Status contract

Write state to `sessions/space-group/status`:

- `running`: at start.
- `blocked`: add a one-line reason after the word.
- `done`: only when the definition of done holds.
