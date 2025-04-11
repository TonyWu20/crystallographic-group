use winnow::{
    error::{ContextError, StrContext},
    token::one_of,
    ModalResult, Parser,
};

use super::{LatticeSymbol, Lattices};

pub fn parse_lattice_symbol(input: &mut &str) -> ModalResult<LatticeSymbol> {
    if parse_minus_sign(input).is_ok() {
        let symbol_char = parse_symbol_char(input)?;
        Ok(LatticeSymbol::new(true, symbol_char))
    } else {
        let symbol_char = parse_symbol_char(input)?;
        Ok(LatticeSymbol::new(false, symbol_char))
    }
}

fn parse_minus_sign<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    "-".parse_next(input)
}

fn parse_symbol_char(input: &mut &str) -> ModalResult<Lattices> {
    let symbol_char = one_of(['P', 'A', 'B', 'C', 'I', 'R', 'F']).parse_next(input)?;
    match symbol_char {
        'P' => Ok(Lattices::P),
        'A' => Ok(Lattices::A),
        'B' => Ok(Lattices::B),
        'C' => Ok(Lattices::C),
        'I' => Ok(Lattices::I),
        'R' => Ok(Lattices::R),
        'F' => Ok(Lattices::F),
        _ => Err(winnow::error::ErrMode::Backtrack(
            ContextError::<StrContext>::new(),
        )),
    }
}
