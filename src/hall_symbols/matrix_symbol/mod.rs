use std::fmt::Display;

use bon::Builder;

use self::parser::parse_hall_matrix_symbol;

use crate::hall_symbols::{errors::HallParseError, translation_symbol::TranslationSymbol};

mod builder;
mod matrices;
mod notations;
mod parser;

pub use matrices::SeitzMatrix;
pub use notations::*;

pub(crate) use parser::matrix_symbol;

/// One generator symbol of a Hall notation, e.g. `2`, `61`, or `-2c`.
///
/// The fields split the symbol into its parts: the sign, the fold, the
/// sub-number, the diagnostic, the rotation axis, and the translation
/// symbols. Build it with the typestate builder
/// [`MatrixSymbol::new_builder`] or parse it with
/// [`MatrixSymbol::try_from_str`]. Resolve it to a
/// [`SeitzMatrix`](super::SeitzMatrix) with
/// [`MatrixSymbol::seitz_matrix`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Builder)]
pub struct MatrixSymbol {
    // `-` or not
    #[builder(default)]
    minus_sign: bool,
    nfold_body: NFold,
    #[builder(default)]
    nfold_sub: NFoldSub,
    #[builder(default)]
    nfold_diag: NFoldDiag,
    #[builder(default)]
    rotation_axis: RotationAxis,
    translation_symbols: Option<Vec<TranslationSymbol>>,
}

impl Display for MatrixSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.minus_sign { "-" } else { "" };
        let translation_symbol = if let Some(symbols) = &self.translation_symbols {
            symbols
                .iter()
                .map(|s| format!("{s}"))
                .collect::<Vec<String>>()
                .concat()
        } else {
            "".to_string()
        };
        write!(
            f,
            "{sign}{}{}{}{}{}",
            self.nfold_body,
            self.rotation_axis,
            self.nfold_diag,
            self.nfold_sub,
            translation_symbol
        )
    }
}

impl MatrixSymbol {
    /// Parse one matrix symbol string, e.g. `"61"` or `"-2c"`.
    pub fn try_from_str(input: &str) -> Result<Self, HallParseError> {
        parse_hall_matrix_symbol(input).map_err(|errors| HallParseError::new(input, errors))
    }

    /// Public entry to the `bon`-generated typestate builder.
    /// Omitting `nfold_body` is a compile error.
    pub fn new_builder() -> MatrixSymbolBuilder {
        Self::builder()
    }

    /// Whether the symbol carries the leading minus sign.
    pub fn minus_sign(&self) -> bool {
        self.minus_sign
    }

    /// The fold body, the rotation or inversion fold of the symbol.
    pub fn nfold_body(&self) -> NFold {
        self.nfold_body
    }

    /// The sub-number, if the fold carries one.
    pub fn nfold_sub(&self) -> NFoldSub {
        self.nfold_sub
    }

    /// The diagnostic character, if the fold carries one.
    pub fn nfold_diag(&self) -> NFoldDiag {
        self.nfold_diag
    }

    /// The rotation axis, or `RotationAxis::Omitted` when implied.
    pub fn rotation_axis(&self) -> RotationAxis {
        self.rotation_axis
    }

    /// The translation symbols, or `None` when the symbol has none.
    pub fn translation_symbols(&self) -> Option<&Vec<TranslationSymbol>> {
        self.translation_symbols.as_ref()
    }

    /// Set the leading minus sign.
    pub fn set_minus_sign(&mut self, minus_sign: bool) {
        self.minus_sign = minus_sign;
    }

    /// Set the fold body.
    pub fn set_nfold_body(&mut self, nfold_body: NFold) {
        self.nfold_body = nfold_body;
    }

    /// Set the sub-number.
    pub fn set_nfold_sub(&mut self, nfold_sub: NFoldSub) {
        self.nfold_sub = nfold_sub;
    }

    /// Set the diagnostic character.
    pub fn set_nfold_diag(&mut self, nfold_diag: NFoldDiag) {
        self.nfold_diag = nfold_diag;
    }

    /// Set the rotation axis.
    pub fn set_rotation_axis(&mut self, rotation_axis: RotationAxis) {
        self.rotation_axis = rotation_axis;
    }
}

/// The error of resolving a matrix symbol to a Seitz matrix.
#[derive(Debug, Clone)]
pub enum MatrixSymbolError {
    /// The symbol is not a valid combination of fields.
    Invalid(MatrixSymbol),
}

impl Display for MatrixSymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixSymbolError::Invalid(symbol) => write!(f, "Invalid symbol {:?}", symbol),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::hall_symbols::translation_symbol::TranslationSymbol;

    use super::{
        notations::{NFold, RotationAxis},
        MatrixSymbol,
    };

    #[test]
    fn matrix_symbol_build() {
        let m2z = MatrixSymbol::new_builder().nfold_body(NFold::N2).build();
        let m2yd = MatrixSymbol::new_builder()
            .nfold_body(NFold::N2)
            .rotation_axis(RotationAxis::Y)
            .translation_symbols(vec![TranslationSymbol::D])
            .build();
        let m1 = m2z.seitz_matrix().unwrap() * m2yd.seitz_matrix().unwrap();
        println!("{}", m1);
        let m2 = m2yd.seitz_matrix().unwrap() * m2z.seitz_matrix().unwrap();
        println!("{}", m2);
        println!("{}", m2yd.seitz_matrix().unwrap() * m1);
        let mut m3 = MatrixSymbol::try_from_str("2\"").unwrap();
        m3.set_rotation_axis(RotationAxis::Z);
        println!("{}", m3);
        println!("{}", m3.seitz_matrix().unwrap());
    }
}
