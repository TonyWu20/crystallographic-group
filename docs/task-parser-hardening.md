# Task: parser hardening (the two HallParseError nits, findings 4 and 7)

Session: `parser-hardening`. Repo: `/home/tony/programming/crystallographic-group`.
This is the first session of the five-session queue. Read
`docs/ergonomics-review.md` for the decision record.

## Goal

Harden the parse boundary. Remove the silent `NFoldSub` mapping. Publish the
restore rules. Keep the suite green. `cargo clippy --lib` ends at zero
warnings.

## Work 1: the two HallParseError nits

From the re-inspection section of the review.

- `HallParseError<'a>` (src/hall_symbols/errors.rs) borrows the input. It
  cannot outlive that input. Make the error lifetime-free: it owns the input
  string and the error spans. ariadne `Rich` stays internal to the parse
  functions. Store owned span data (byte offsets and label text), not `Rich`.
  Every `try_from_str` entry then returns `Result<Self, HallParseError>`
  with no lifetime.
- Prove the lifetime is gone with a compile-time check: a
  `fn assert_static<T: 'static>() {}` call for the error type in the test
  module.
- `HallParseError::print` writes to stdout and uses `.expect` on an error
  path. Replace it with a render method that returns the formatted report
  text as a `String`. If you keep a print convenience, it writes to stderr.
  No panic on the error path.

## Work 2: finding 4, delete the silent NFoldSub mapping

- Delete `impl From<&str> for NFoldSub` and `impl From<&char> for NFoldSub`
  in src/hall_symbols/matrix_symbol/notations/mod.rs:51-71. They map any
  unknown input to `NFoldSub::None`. That is silent data loss.
- The only caller is src/hall_symbols/matrix_symbol/parser.rs:52. Map the
  digit to the variant with a local match there. The `one_of("12345")`
  above it restricts the domain, so the match is total over it.
- The `None` variant itself stays. It means "no sub-number", which is a
  real value.
- Decision D7 in the review authorizes the deletion.

## Work 3: finding 7, publish the restore rules

- `restore_information_in_matrix_symbols` (src/hall_symbols/parser.rs:32)
  is private to the parse path. A user who builds symbols by hand must
  rediscover the implied-axis rules.
- Make it `pub` with a doc comment that states the rules it applies.
  Re-export it at the `hall_symbols` module root.
- Decision D8 in the review authorizes the promotion.

## Out of scope

- Findings 5, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20.
- The parser framework and grammar.
- The `SpaceGroup` type.

## Definition of done

- `cargo clippy --lib`: zero warnings.
- `cargo test --lib`: 16 of 16, including `test_all`.
- `HallParseError` has no lifetime parameter. The compile-time `'static`
  check is in the test module.
- No `From<&str>` or `From<&char>` impl remains for `NFoldSub`.
- `restore_information_in_matrix_symbols` is reachable as
  `crystallographic_group::hall_symbols::restore_information_in_matrix_symbols`.

## Commit

Commit your work when done. One commit per work item is fine, or one
combined commit. Subject lines:

- "Make HallParseError lifetime-free and render to a String"
- "Delete the silent NFoldSub From impls"
- "Publish the Hall symbol restore rules"

## Status contract

Write state to `sessions/parser-hardening/status`:

- `running`: at start.
- `blocked`: add a one-line reason after the word.
- `done`: only when the definition of done holds.
