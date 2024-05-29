use self::{lattice_symbol::LatticeSymbol, matrix_symbol::MatrixSymbol, origin_shift::OriginShift};

mod lattice_symbol;
mod matrix_symbol;
mod origin_shift;
mod translation_symbol;

pub const SEITZ_TRANSLATE_BASE_NUMBER: i32 = 12;

pub struct HallSymbolNotation {
    lattice_symbol: LatticeSymbol,
    matrix_symbols: Vec<MatrixSymbol>,
    origin_shift: OriginShift,
}
