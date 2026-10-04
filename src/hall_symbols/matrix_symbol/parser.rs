use chumsky::{Parser, extra, error::Rich, prelude::*};

use crate::hall_symbols::{matrix_symbol::NFold, translation_symbol::TranslationSymbol};

use super::{MatrixSymbol, NFoldDiag, NFoldSub, RotationAxis};

/// Parse one matrix symbol at the start of `input`.
pub(crate) fn parse_hall_matrix_symbol<'a>(
    input: &'a str,
) -> Result<MatrixSymbol, Vec<Rich<'a, char>>> {
    matrix_symbol()
        .parse(input)
        .into_result()
}

/// Chumsky builder for one matrix symbol:
/// optional `-`, one of `12346`, an optional axis or diagnostic char
/// (`x y z " ' *`, plus the escaped forms `\"` and `\'`), an optional
/// sub-number digit, and optional translation chars.
///
/// Leading whitespace is allowed.
pub(crate) fn matrix_symbol<'a>()
-> impl Parser<'a, &'a str, MatrixSymbol, extra::Err<Rich<'a, char>>> {
    let minus_sign = just('-').or_not();
    let nfold_body = one_of("12346").map(|digit: char| match digit {
        '1' => NFold::N1,
        '2' => NFold::N2,
        '3' => NFold::N3,
        '4' => NFold::N4,
        '6' => NFold::N6,
        // `one_of` above restricts the input to 12346.
        _ => unreachable!(),
    });
    let axis = choice((
        just('x').map(|_| (RotationAxis::X, NFoldDiag::None)),
        just('y').map(|_| (RotationAxis::Y, NFoldDiag::None)),
        just('z').map(|_| (RotationAxis::Z, NFoldDiag::None)),
        just('\'').map(|_| (RotationAxis::Omitted, NFoldDiag::SingleQuote)),
        just('"').map(|_| (RotationAxis::Omitted, NFoldDiag::DoubleQuote)),
        just('*').map(|_| (RotationAxis::Omitted, NFoldDiag::Asterisk)),
        // Escaped diagnostic chars, mirroring the winnow `take_escaped` form.
        just('\\')
            .ignore_then(one_of("\"'"))
            .map(|escaped: char| match escaped {
                '"' => (RotationAxis::Omitted, NFoldDiag::DoubleQuote),
                '\'' => (RotationAxis::Omitted, NFoldDiag::SingleQuote),
                // `one_of` above restricts the input to " and '.
                _ => unreachable!(),
            }),
    ));
    let nfold_sub = one_of("12345")
        .map(|digit: char| NFoldSub::from(&digit))
        .or_not();
    let translation_symbols = one_of("abcnuvwd")
        .repeated()
        .collect::<Vec<char>>()
        .map(|chars: Vec<char>| {
            if chars.is_empty() {
                None
            } else {
                Some(chars.iter().map(TranslationSymbol::from).collect())
            }
        });

    text::whitespace()
        .then(minus_sign)
        .then(nfold_body)
        .then(axis.or_not())
        .then(nfold_sub.then(translation_symbols))
        .map(
            |((((_leading, minus_sign), nfold_body), axis), (nfold_sub, translation_symbols))| {
                let mut builder = MatrixSymbol::new_builder();
                builder
                    .set_minus_sign(minus_sign.is_some())
                    .set_nfold_body(nfold_body);
                if let Some((rotation_axis, nfold_diag)) = axis {
                    builder.set_rotation_axis(rotation_axis);
                    builder.set_nfold_diag(nfold_diag);
                }
                if let Some(nfold_sub) = nfold_sub {
                    builder.set_nfold_sub(nfold_sub);
                }
                builder.set_translation_symbols(translation_symbols);
                // `nfold_body` is always set, so the build cannot fail.
                builder
                    .build()
                    .expect("matrix symbol build: nfold_body is set")
            },
        )
}

#[cfg(test)]
mod test {

    use chumsky::{IterParser, Parser};

    use super::matrix_symbol;

    #[test]
    fn parse_single_matrix_symbol() {
        let input = "61 4acd 2ab 3 -2\" -2ac -2n-1bc";
        let symbols = matrix_symbol()
            .repeated()
            .collect::<Vec<_>>()
            .parse(input)
            .into_result()
            .unwrap();
        for symbol in &symbols {
            println!("{}, {:?}", symbol, symbol);
        }
    }
}
