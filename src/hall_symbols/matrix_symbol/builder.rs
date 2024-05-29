use crate::hall_symbols::translation_symbol::TranslationSymbol;

use super::{
    notations::{NFold, NFoldDiag, NFoldSub, RotationAxis},
    MatrixSymbol, MatrixSymbolError,
};

#[derive(Debug, Clone, Default)]
pub struct MatrixSymbolBuilder {
    minus_sign: Option<bool>,
    nfold_body: Option<NFold>,
    nfold_sub: Option<NFoldSub>,
    nfold_diag: Option<NFoldDiag>,
    rotation_axis: Option<RotationAxis>,
    translation_symbols: Option<Vec<TranslationSymbol>>,
}

impl MatrixSymbolBuilder {
    pub fn set_minus_sign(&mut self, minus_sign: bool) -> &mut Self {
        self.minus_sign = Some(minus_sign);
        self
    }

    pub fn set_nfold_body(&mut self, nfold_body: NFold) -> &mut Self {
        self.nfold_body = Some(nfold_body);
        self
    }

    pub fn set_nfold_sub(&mut self, nfold_sub: NFoldSub) -> &mut Self {
        self.nfold_sub = Some(nfold_sub);
        self
    }

    pub fn set_nfold_diag(&mut self, nfold_diag: NFoldDiag) -> &mut Self {
        self.nfold_diag = Some(nfold_diag);
        self
    }

    pub fn set_rotation_axis(&mut self, rotation_axis: RotationAxis) -> &mut Self {
        self.rotation_axis = Some(rotation_axis);
        self
    }

    pub fn set_translation_symbols(
        &mut self,
        translation_symbols: Vec<TranslationSymbol>,
    ) -> &mut Self {
        self.translation_symbols = Some(translation_symbols);
        self
    }
    pub fn build<'a>(&mut self) -> Result<MatrixSymbol, MatrixSymbolError<'a>> {
        if self.nfold_body.is_some() && self.rotation_axis.is_some() {
            Ok(MatrixSymbol {
                minus_sign: self.minus_sign.unwrap_or(false),
                nfold_body: self.nfold_body.unwrap(),
                nfold_sub: self.nfold_sub.unwrap_or_default(),
                nfold_diag: self.nfold_diag.unwrap_or_default(),
                rotation_axis: self.rotation_axis.unwrap(),
                translation_symbols: self.translation_symbols.clone(),
            })
        } else {
            Err(MatrixSymbolError::IncompleteFields)
        }
    }
}
