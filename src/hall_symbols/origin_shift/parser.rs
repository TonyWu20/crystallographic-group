use winnow::{
    ascii::{dec_int, space0, space1},
    combinator::{delimited, preceded, separated},
    error::{AddContext, ContextError, StrContext, StrContextValue},
    stream::Stream,
    PResult, Parser,
};

use super::OriginShift;

pub fn parse_origin_shift(input: &mut &str) -> PResult<OriginShift> {
    let parse_shift: PResult<Vec<i32>> = preceded(
        space0,
        delimited("(", separated(1..=3, dec_int::<_, i32, _>, space1), ")"),
    )
    .context(StrContext::Label("Origin shift vector"))
    .parse_next(input);
    match parse_shift {
        Ok(numbers) => {
            if numbers.len() != 3 {
                let err_context = ContextError::<StrContext>::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Expected(StrContextValue::Description(
                        "Incorrect input: not `i32`",
                    )),
                );
                Err(winnow::error::ErrMode::Backtrack(err_context))
            } else {
                let [va, vb, vc] = numbers.try_into().unwrap();
                Ok(OriginShift::new(va, vb, vc))
            }
        }
        Err(e) => Ok(OriginShift::default()),
    }
}

#[cfg(test)]
mod test {

    use winnow::{
        ascii::{dec_int, space0, space1},
        combinator::{delimited, preceded, separated},
        PResult, Parser,
    };

    #[test]
    fn test_origin_shift_parsing() {
        let mut input = " (0 0 -1)";
        let parse_shift: PResult<Vec<i32>> = preceded(
            space0,
            delimited("(", separated(1..=3, dec_int::<_, i32, _>, space1), ")"),
        )
        .context(winnow::error::StrContext::Label(input))
        .parse_next(&mut input);
        dbg!(parse_shift.unwrap());
    }
}
