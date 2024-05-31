use winnow::{
    ascii::{space0, take_escaped},
    combinator::{alt, preceded, repeat},
    error::{AddContext, ContextError, ErrMode, StrContext, StrContextValue},
    stream::Stream,
    token::one_of,
    PResult, Parser,
};

use crate::hall_symbols::{matrix_symbol::NFold, translation_symbol::TranslationSymbol};

use super::{MatrixSymbol, NFoldDiag, NFoldSub, RotationAxis};

/// Parse the matrix symbols in Hall notations.
pub fn parse_hall_matrix_symbol(input: &mut &str) -> PResult<MatrixSymbol> {
    let (sign, nfold) = parse_sign_fold(input)?;
    if sign && matches!(nfold, NFold::Invalid) {
        let err_context = ContextError::<StrContext>::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Expected(StrContextValue::Description(
                "Not a valid sign and `NFold` combination",
            )),
        );
        return Err(ErrMode::Backtrack(err_context));
    }
    match parse_axis(input) {
        Ok((axis, diag)) => {
            if let Ok((sub, translations)) = parse_translations(input) {
                Ok(MatrixSymbol::new_builder()
                    .set_minus_sign(sign)
                    .set_nfold_body(nfold)
                    .set_rotation_axis(axis)
                    .set_nfold_diag(diag)
                    .set_nfold_sub(sub)
                    .set_translation_symbols(translations)
                    .build()
                    .unwrap())
            } else {
                Ok(MatrixSymbol::new_builder()
                    .set_minus_sign(sign)
                    .set_nfold_body(nfold)
                    .set_rotation_axis(axis)
                    .set_nfold_diag(diag)
                    .build()
                    .unwrap())
            }
        }
        Err(_) => {
            if let Ok((sub, translations)) = parse_translations(input) {
                Ok(MatrixSymbol::new_builder()
                    .set_minus_sign(sign)
                    .set_nfold_body(nfold)
                    .set_nfold_sub(sub)
                    .set_translation_symbols(translations)
                    .build()
                    .unwrap())
            } else {
                Ok(MatrixSymbol::new_builder()
                    .set_minus_sign(sign)
                    .set_nfold_body(nfold)
                    .build()
                    .unwrap())
            }
        }
    }
}

fn parse_sign_fold(input: &mut &str) -> PResult<(bool, NFold)> {
    preceded(
        space0,
        alt(("1", "-1", "2", "-2", "3", "-3", "4", "-4", "6", "-6")),
    )
    .map(|s| match s {
        "1" => (false, NFold::N1),
        "2" => (false, NFold::N2),
        "-2" => (true, NFold::N2),
        "3" => (false, NFold::N3),
        "-3" => (true, NFold::N3),
        "4" => (false, NFold::N4),
        "-4" => (true, NFold::N4),
        "6" => (false, NFold::N6),
        "-6" => (true, NFold::N6),
        "-1" => (true, NFold::N1),
        _ => (true, NFold::Invalid),
    })
    .parse_next(input)
}

fn parse_axis(input: &mut &str) -> PResult<(RotationAxis, NFoldDiag)> {
    let axis_try = take_escaped(
        alt(('x', 'y', 'z', '\'', '"', '*')),
        '\\',
        one_of(['"', '\\', '\'']),
    )
    .map(|s| match s {
        "x" => (RotationAxis::X, NFoldDiag::None),
        "y" => (RotationAxis::Y, NFoldDiag::None),
        "z" => (RotationAxis::Z, NFoldDiag::None),
        "\'" => (RotationAxis::Omitted, NFoldDiag::SingleQuote),
        "\"" => (RotationAxis::Omitted, NFoldDiag::DoubleQuote),
        "*" => (RotationAxis::Omitted, NFoldDiag::Asterisk),
        _ => (RotationAxis::Omitted, NFoldDiag::None),
    })
    .parse_next(input)?;
    if matches!(axis_try, (RotationAxis::Omitted, NFoldDiag::None)) {
        let err_context = ContextError::<StrContext>::new().add_context(
            input,
            &input.checkpoint(),
            StrContext::Expected(StrContextValue::Description(
                "Not a valid axis symbol, possibly the axis is omitted and there are translations symbols instead",
            )),
        );
        Err(ErrMode::Backtrack(err_context))
    } else {
        Ok(axis_try)
    }
}

/// Successful cases have been naturally return by matching
fn parse_translations(input: &mut &str) -> PResult<(NFoldSub, Option<Vec<TranslationSymbol>>)> {
    let to_peek: &str = input;
    let (_, peeking) = one_of([
        '1', '2', '3', '4', '5', 'a', 'b', 'c', 'n', 'u', 'v', 'w', 'd',
    ])
    .parse_peek(to_peek)?;
    match peeking {
        peeking if ['1', '2', '3', '4', '5'].contains(&peeking) => {
            one_of(['1', '2', '3', '4', '5'])
                .map(|c| (NFoldSub::from(&c), None))
                .parse_next(input)
        }
        peeking if ['a', 'b', 'c', 'n', 'u', 'v', 'w', 'd'].contains(&peeking) => {
            repeat(0.., one_of(['a', 'b', 'c', 'n', 'u', 'v', 'w', 'd']))
                .map(|trs: Vec<char>| {
                    let translation_symbols: Vec<TranslationSymbol> =
                        trs.iter().map(TranslationSymbol::from).collect();
                    (NFoldSub::None, Some(translation_symbols))
                })
                .parse_next(input)
        }
        _ => {
            let err_context = ContextError::<StrContext>::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Expected(StrContextValue::Description(
                    "Not valid translation symbols",
                )),
            );
            Err(ErrMode::Backtrack(err_context))
        }
    }
}

#[cfg(test)]
mod test {

    use super::parse_hall_matrix_symbol;

    #[test]
    fn parse_single_matrix_symbol() {
        let mut symbol = "61 4acd 2ab 3 -2\" -2ac -2n-1bc";
        while let Ok(notation) = parse_hall_matrix_symbol(&mut symbol) {
            println!("{}, {:?}", notation, notation)
        }
    }
}
