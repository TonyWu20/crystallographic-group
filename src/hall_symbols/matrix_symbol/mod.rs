use std::fmt::Display;

use super::translation_symbol::TranslationSymbol;

mod builder;
mod matrices;
mod notations;

pub use builder::MatrixSymbolBuilder;
pub use matrices::SeitzMatrix;
pub use notations::*;

#[derive(Debug, Clone)]
pub struct MatrixSymbol {
    // `-` or not
    minus_sign: bool,
    nfold_body: NFold,
    nfold_sub: NFoldSub,
    nfold_diag: NFoldDiag,
    rotation_axis: RotationAxis,
    translation_symbols: Option<Vec<TranslationSymbol>>,
}

impl MatrixSymbol {
    pub fn new_builder() -> MatrixSymbolBuilder {
        MatrixSymbolBuilder::default()
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
        matrix_symbol::matrices::SeitzMatrix, translation_symbol::TranslationSymbol,
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
            .set_translation_symbols(vec![
                TranslationSymbol::A,
                TranslationSymbol::C,
                TranslationSymbol::D,
            ])
            .build()
            .unwrap();
        let matrix = m_4acd.seitz_matrix().unwrap();
        println!("4acd: {matrix}");
        let m_2xc = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N2)
            .set_rotation_axis(RotationAxis::X)
            .set_translation_symbols(vec![TranslationSymbol::C])
            .build()
            .unwrap();
        println!("2xc: {}", m_2xc.seitz_matrix().unwrap());
        let m_4vw = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N4)
            .set_rotation_axis(RotationAxis::default())
            .set_translation_symbols(vec![TranslationSymbol::V, TranslationSymbol::W])
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
    }
}
