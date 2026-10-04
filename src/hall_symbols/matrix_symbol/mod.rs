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
    pub fn try_from_str(input: &str) -> Result<Self, HallParseError<'_>> {
        parse_hall_matrix_symbol(input).map_err(|errors| HallParseError::new(input, errors))
    }

    /// Public entry to the `bon`-generated typestate builder.
    /// Omitting `nfold_body` is a compile error.
    pub fn new_builder() -> MatrixSymbolBuilder {
        Self::builder()
    }

    pub fn minus_sign(&self) -> bool {
        self.minus_sign
    }

    pub fn nfold_body(&self) -> NFold {
        self.nfold_body
    }

    pub fn nfold_sub(&self) -> NFoldSub {
        self.nfold_sub
    }

    pub fn nfold_diag(&self) -> NFoldDiag {
        self.nfold_diag
    }

    pub fn rotation_axis(&self) -> RotationAxis {
        self.rotation_axis
    }

    pub fn translation_symbols(&self) -> Option<&Vec<TranslationSymbol>> {
        self.translation_symbols.as_ref()
    }

    pub fn set_minus_sign(&mut self, minus_sign: bool) {
        self.minus_sign = minus_sign;
    }

    pub fn set_nfold_body(&mut self, nfold_body: NFold) {
        self.nfold_body = nfold_body;
    }

    pub fn set_nfold_sub(&mut self, nfold_sub: NFoldSub) {
        self.nfold_sub = nfold_sub;
    }

    pub fn set_nfold_diag(&mut self, nfold_diag: NFoldDiag) {
        self.nfold_diag = nfold_diag;
    }

    pub fn set_rotation_axis(&mut self, rotation_axis: RotationAxis) {
        self.rotation_axis = rotation_axis;
    }
}

#[derive(Debug, Clone)]
pub enum MatrixSymbolError {
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
