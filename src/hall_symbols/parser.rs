use chumsky::{IterParser, Parser};

use crate::hall_symbols::{
    errors::HallParseError,
    matrix_symbol::{MatrixSymbol, NFold, NFoldDiag, RotationAxis},
};

use super::{
    lattice_symbol::lattice_symbol,
    matrix_symbol::matrix_symbol,
    origin_shift::origin_shift,
    HallSymbolNotation,
};

/// Parse a full Hall symbol notation:
/// lattice symbol, then repeated matrix symbols, then an optional origin shift.
pub fn parse_hall_symbol(input: &str) -> Result<HallSymbolNotation, HallParseError> {
    let ((lattice, mut matrix_symbols), origin_shift) = lattice_symbol()
        .then(matrix_symbol().repeated().collect::<Vec<MatrixSymbol>>())
        .then(origin_shift())
        .parse(input)
        .into_result()
        .map_err(|errors| HallParseError::new(input, errors))?;
    restore_information_in_matrix_symbols(&mut matrix_symbols);
    Ok(HallSymbolNotation::new(
        lattice,
        matrix_symbols,
        origin_shift,
    ))
}

/// Apply the implied-axis rules of the Hall notation to a list of matrix
/// symbols.
///
/// For most Hall symbols the rotation axes are implied by the position of
/// each symbol, so an explicit axis symbol is not written. This function
/// restores the missing axis information. The rules it applies are:
///
/// - The first rotation gets the `c` axis (`RotationAxis::Z`) when its axis
///   was omitted.
/// - The second rotation, when its N is 2, gets the `a` axis
///   (`RotationAxis::X`) if the first N is 2 or 4. If the first N is 3 or 6,
///   it gets the `c` axis (`RotationAxis::Z`, the `a-b` direction), and the
///   single-quote diagnostic is added when the symbol carries none.
/// - The third rotation, when its N is 3, gets the asterisk diagnostic: its
///   axis is the `a+b+c` body diagonal.
///
/// Call this on a hand-built list of [`MatrixSymbol`] to apply the same
/// rules the parse path applies.
pub fn restore_information_in_matrix_symbols(symbols_vec: &mut [MatrixSymbol]) {
    // For most Hall symbols the rotation axes applicable to each N are implied and an explicit axis symbol A is not needed. The rules for default axis directions are:
    // the first rotation has an axis direction of c
    // the second rotation (if N is 2) has an axis direction of
    // a     if preceded by an N of 2 or 4
    // a-b if preceded by an N of 3 or 6
    // the third rotation (N is always 3) has an axis direction of
    // a+b+c
    let rotation_folds: Vec<NFold> = symbols_vec
        .iter()
        .map(|symbol| symbol.nfold_body())
        .collect();
    symbols_vec.iter_mut().enumerate().for_each(|(i, symbol)| {
        if i == 0 && matches!(symbol.rotation_axis(), RotationAxis::Omitted) {
            symbol.set_rotation_axis(RotationAxis::Z)
        }
        if i == 1 && matches!(symbol.nfold_body(), NFold::N2) {
            match rotation_folds[i - 1] {
                NFold::N2 | NFold::N4 => symbol.set_rotation_axis(RotationAxis::X),
                NFold::N3 | NFold::N6 => {
                    if matches!(symbol.nfold_diag(), NFoldDiag::None) {
                        symbol.set_nfold_diag(NFoldDiag::SingleQuote)
                    }
                    symbol.set_rotation_axis(RotationAxis::Z);
                }
                _ => (),
            }
        }
        if i == 2 && matches!(symbol.nfold_body(), NFold::N3) {
            symbol.set_nfold_diag(NFoldDiag::Asterisk);
        }
    });
}
