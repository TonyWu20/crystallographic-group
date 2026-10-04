use chumsky::{Parser, extra, error::Rich, prelude::*};

use super::OriginShift;

/// Parse the optional origin shift at the end of `input`.
///
/// A shift is `( va vb vc )` with optional leading whitespace.
/// When no `(` is present, the default `(0 0 0)` is returned.
/// A `(` holding one or two integers is a parse error, as is any other
/// malformed parenthesis content.
pub(crate) fn parse_origin_shift<'a>(
    input: &'a str,
) -> Result<OriginShift, Vec<Rich<'a, char>>> {
    origin_shift()
        .parse(input)
        .into_result()
}

/// Chumsky builder for the origin shift grammar.
///
/// A missing shift yields [`OriginShift::default`]. A `(` holding one or
/// two integers fails with one custom `Rich` error.
pub(crate) fn origin_shift<'a>()
-> impl Parser<'a, &'a str, OriginShift, extra::Err<Rich<'a, char>>> {
    let open = text::whitespace().then(just('('));
    // `( va vb vc )`, exactly 3 integers.
    let shift3 = open
        .then(three_ints())
        .then_ignore(just(')'))
        .map(|(_open, (va, vb, vc))| OriginShift::new(va, vb, vc));
    // `( i )` or `( i i )` is a parse error.
    let bad_shift = choice((two_ints_closed(), one_int_closed()))
        .then(fail_shift())
        .map(|_| OriginShift::default());
    let without_parenthesis =
        text::whitespace().then(just('(').not()).map(|_| OriginShift::default());
    choice((shift3, bad_shift, without_parenthesis))
}

/// One shift integer: optional `-`, then one or more decimal digits.
fn shift_int<'a>() -> impl Parser<'a, &'a str, i32, extra::Err<Rich<'a, char>>> {
    let digits = text::digits::<&'a str, extra::Err<Rich<'a, char>>>(10).to_slice();
    just('-')
        .or_not()
        .then(digits)
        .validate(|(sign, value): (Option<char>, &'a str), _extra, emitter| {
            let magnitude: i64 = match value.parse() {
                Ok(magnitude) => magnitude,
                Err(_) => {
                    emitter.emit(Rich::custom(_extra.span(), "shift integer is out of range"));
                    return (sign, value);
                }
            };
            let signed = if sign.is_some() {
                -magnitude
            } else {
                magnitude
            };
            if i32::try_from(signed).is_err() {
                emitter.emit(Rich::custom(
                    _extra.span(),
                    "shift integer does not fit in i32",
                ));
            }
            (sign, value)
        })
        .map(|(sign, value): (Option<char>, &'a str)| {
            let magnitude: i64 = value.parse().unwrap_or(i64::MAX);
            let signed = if sign.is_some() {
                -magnitude
            } else {
                magnitude
            };
            i32::try_from(signed).unwrap_or(i32::MAX)
        })
}

/// One or more spaces (or tabs) between two integers.
fn space1<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    one_of(" \t").repeated().at_least(1)
}

/// Exactly 3 integers after the opening `(`.
fn three_ints<'a>() -> impl Parser<'a, &'a str, (i32, i32, i32), extra::Err<Rich<'a, char>>> {
    space1()
        .or_not()
        .then(shift_int())
        .then(space1())
        .then(shift_int())
        .then(space1())
        .then(shift_int())
        .map(|(((((_spaces, va), _sep1), vb), _sep2), vc)| (va, vb, vc))
}

/// `( i )` after the opening `(`.
fn one_int_closed<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    space1()
        .or_not()
        .then(shift_int())
        .then_ignore(just(')'))
        .map(|_| ())
}

/// `( i i )` after the opening `(`.
fn two_ints_closed<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    space1()
        .or_not()
        .then(shift_int())
        .then(space1())
        .then(shift_int())
        .then_ignore(just(')'))
        .map(|_| ())
}

/// A parser that always fails with one `Rich` error.
fn fail_shift<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    custom(|inp| {
        Err(Rich::custom(
            inp.span_since(&inp.cursor()),
            "origin shift must have exactly 3 integers",
        ))
    })
}

#[cfg(test)]
mod test {

    use super::parse_origin_shift;
    use crate::hall_symbols::origin_shift::OriginShift;

    #[test]
    fn test_origin_shift_parsing() {
        let parsed = parse_origin_shift(" (0 0 -1)").unwrap();
        dbg!(parsed);
        assert_eq!(parsed, OriginShift::new(0, 0, -1));
        assert_eq!(parse_origin_shift("").unwrap(), OriginShift::default());
    }
}
