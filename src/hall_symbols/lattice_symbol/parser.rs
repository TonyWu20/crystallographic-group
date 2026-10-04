use chumsky::{Parser, extra, error::Rich, prelude::*};

use super::{LatticeSymbol, Lattices};

/// Parse the lattice symbol at the start of `input`:
/// an optional `-`, then one of `PABCIRF`.
pub(crate) fn parse_lattice_symbol<'a>(
    input: &'a str,
) -> Result<LatticeSymbol, Vec<Rich<'a, char>>> {
    lattice_symbol()
        .parse(input)
        .into_result()
}

/// Chumsky builder for the lattice symbol grammar.
pub(crate) fn lattice_symbol<'a>()
-> impl Parser<'a, &'a str, LatticeSymbol, extra::Err<Rich<'a, char>>> {
    just('-')
        .or_not()
        .then(one_of("PABCIRF"))
        .map(|(minus_sign, symbol_char): (Option<char>, char)| {
            let lattice = match symbol_char {
                'P' => Lattices::P,
                'A' => Lattices::A,
                'B' => Lattices::B,
                'C' => Lattices::C,
                'I' => Lattices::I,
                'R' => Lattices::R,
                'F' => Lattices::F,
                // `one_of` above restricts the input to PABCIRF.
                _ => unreachable!(),
            };
            LatticeSymbol::new(minus_sign.is_some(), lattice)
        })
}
