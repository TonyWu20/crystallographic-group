use winnow::PResult;

use crate::hall_symbols::{
    matrix_symbol::{MatrixSymbol, NFold, NFoldDiag},
    origin_shift::OriginShift,
};

use super::{lattice_symbol::LatticeSymbol, matrix_symbol::RotationAxis, HallSymbolNotation};

pub fn parse_hall_symbol(input: &mut &str) -> PResult<HallSymbolNotation> {
    let lattice_symbol = LatticeSymbol::try_from_str(input)?;
    let mut matrix_symbols: Vec<MatrixSymbol> = Vec::new();
    while let Ok(symbol) = MatrixSymbol::try_from_str(input) {
        matrix_symbols.push(symbol);
    }
    let origin_shift = OriginShift::try_from_str(input)?;
    restore_information_in_matrix_symbols(&mut matrix_symbols);
    Ok(HallSymbolNotation::new(
        lattice_symbol,
        matrix_symbols,
        origin_shift,
    ))
}

fn restore_information_in_matrix_symbols(symbols_vec: &mut [MatrixSymbol]) {
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
                    symbol
                        .set_nfold_diag(crate::hall_symbols::matrix_symbol::NFoldDiag::SingleQuote);
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
