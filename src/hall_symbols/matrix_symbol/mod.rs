use std::fmt::Display;

use self::parser::parse_hall_matrix_symbol;

use super::translation_symbol::TranslationSymbol;

mod builder;
mod matrices;
mod notations;
mod parser;

pub use builder::MatrixSymbolBuilder;
pub use matrices::SeitzMatrix;
pub use notations::*;
use winnow::PResult;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatrixSymbol {
    // `-` or not
    minus_sign: bool,
    nfold_body: NFold,
    nfold_sub: NFoldSub,
    nfold_diag: NFoldDiag,
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
    pub fn try_from_str(input: &mut &str) -> PResult<Self> {
        parse_hall_matrix_symbol(input)
    }

    pub fn new_builder() -> MatrixSymbolBuilder {
        MatrixSymbolBuilder::default()
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
pub enum MatrixSymbolError<'a> {
    Invalid(&'a MatrixSymbol),
    IncompleteFields,
}

impl<'a> Display for MatrixSymbolError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixSymbolError::Invalid(symbol) => write!(f, "Invalid symbol {:?}", symbol),
            MatrixSymbolError::IncompleteFields => {
                f.write_str("Incomplete fields of `MatrixSymbol`")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::hall_symbols::{
        matrix_symbol::{matrices::SeitzMatrix, NFoldSub},
        translation_symbol::TranslationSymbol,
    };

    use super::{
        notations::{NFold, RotationAxis},
        MatrixSymbol,
    };

    #[test]
    fn matrix_symbol_build() {
        let m_4acd: MatrixSymbol = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N4)
            .set_rotation_axis(RotationAxis::Z)
            .set_translation_symbols(Some(vec![
                TranslationSymbol::A,
                TranslationSymbol::C,
                TranslationSymbol::D,
            ]))
            .build()
            .unwrap();
        let matrix = m_4acd.seitz_matrix().unwrap();
        println!("4acd: {matrix}");
        let m_2xc = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N2)
            .set_rotation_axis(RotationAxis::X)
            .set_translation_symbols(Some(vec![TranslationSymbol::C]))
            .build()
            .unwrap();
        println!("2xc: {}", m_2xc.seitz_matrix().unwrap());
        let m_4vw = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N4)
            .set_rotation_axis(RotationAxis::default())
            .set_translation_symbols(Some(vec![TranslationSymbol::V, TranslationSymbol::W]))
            .build()
            .unwrap();
        println!("4vw: {}", m_4vw.seitz_matrix().unwrap());
        let m_4 = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N4)
            .set_rotation_axis(RotationAxis::Z)
            .build()
            .unwrap();
        let mut set: HashSet<SeitzMatrix> = HashSet::new();
        let e = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N1)
            .set_rotation_axis(RotationAxis::Z)
            .build()
            .unwrap()
            .seitz_matrix()
            .unwrap();
        let m_4_mat = m_4.seitz_matrix().unwrap();
        set.insert(m_4_mat);
        set.insert(e);
        dbg!(set.insert(m_4_mat * m_4_mat));
        dbg!(set.insert(m_4_mat.powi(3)));
        dbg!(set.insert(m_4_mat.powi(4)));
        let m_3 = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N3)
            .set_rotation_axis(RotationAxis::Z)
            .build()
            .unwrap();
        let m_3_mat = m_3.seitz_matrix().unwrap();
        println!("{}", m_3_mat);
        let m_61 = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N6)
            .set_nfold_sub(NFoldSub::N1)
            .build()
            .unwrap();
        println!("{}", m_61.seitz_matrix().unwrap());
    }
}
