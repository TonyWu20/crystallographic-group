# Changelog

All notable changes to `crystallographic-group` are recorded here.

## 0.4.0 (2026-10-04)

The release closes the ergonomics review recorded in
`docs/ergonomics-review.md`.

### Added

- `SpaceGroup`, the public center type. It holds the lattice, the
  generator symbols, and the origin shift. It exposes the group
  number, the HM symbol, the crystal system, and the general
  positions.
- `SpaceGroupNumber`, the typed space group index.
- `SpaceGroupTable`, the concrete lookups over the symbol tables.
  Lookups return `&'static str`.
- `CrystalSystem::of_number`, the standard system split by group
  number.
- `restore_information_in_matrix_symbols`, now public and
  documented.
- `examples/entry_paths.rs`, which shows both entry paths: parse a
  Hall symbol, and assemble a group from typed parts.
- Doc comments on the public items. The crate doc states the 12-fold
  translation base.

### Changed (breaking)

- The Hall symbol parser moved from `winnow` to `chumsky` with
  `ariadne` diagnostics. Every `try_from_str` entry takes a `&str`
  and returns `Result<T, HallParseError>`. The error is crate-owned
  and lifetime-free. No parser crate type appears in the public API.
- `MatrixSymbolError` owns the symbol it reports. It has no lifetime
  parameter. The `IncompleteFields` variant is gone.
- `MatrixSymbol` builds through a `bon`-generated typestate builder.
  Omitting `nfold_body` is a compile error. `MatrixSymbol::new()` is
  fallible and runs the cross-field rules.
- The 530-variant `SpaceGroupHallSymbol` enum and the
  `LookUpSpaceGroup` trait are removed. Use `SpaceGroupNumber` and
  `SpaceGroupTable` instead.
- The silent `From<&str>` and `From<&char>` impls for `NFoldSub`
  are removed.
- `SeitzMatrix` invariants hold. `Hash` normalizes the translation
  part like `PartialEq`. `eigenvector()` returns `Option`.
  `Display` has a safe fallback. `Add` for two matrices adds
  translation parts only. Adding a vector normalizes to positive
  residues.
- Renames: `jones_faithful_repr` is now `formula`. `num_of_general_pos`
  is now `len`. The `matrice` typo is fixed. The two text output
  methods on `GeneralPositions` are merged into `formulas()`.
- The table statics are renamed. `FULL_SPACE_GROUP_SYMBOLS` is now
  `ALL_SPACE_GROUP_SYMBOLS`. `DEFAULT_SPACE_GROUP_SYMBOLS` is now
  `PER_NUMBER_SPACE_GROUP_SYMBOLS`.

### Fixed

- The `Hash`/`PartialEq` invariant on `SeitzMatrix`. Equal values
  now hash equal.
- The panic path in `SeitzMatrix` `Display`.
