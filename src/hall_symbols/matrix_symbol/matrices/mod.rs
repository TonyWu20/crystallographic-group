use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use nalgebra::{Matrix4, Vector3};

use crate::hall_symbols::SEITZ_TRANSLATE_BASE_NUMBER;

use super::{MatrixSymbol, MatrixSymbolError, NFold, NFoldDiag, RotationAxis};

mod rotation_matrices;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub struct SeitzMatrix(Matrix4<i32>);

impl SeitzMatrix {
    // Property of cyclic group
    pub fn powi(&self, exponent: i32) -> Self {
        match exponent.cmp(&0) {
            std::cmp::Ordering::Less => {
                // All the transformations related here have det ± 1
                // Inverse matrix is guaranteed.
                let inv = self.try_inverse().unwrap();
                let mat = (0..exponent.abs())
                    .map(|_| inv.matrix())
                    .reduce(|acc, x| acc * x)
                    .unwrap();
                Self(mat)
            }
            std::cmp::Ordering::Equal => Self(Matrix4::identity()),
            std::cmp::Ordering::Greater => {
                let mat = (0..exponent)
                    .map(|_| self.0)
                    .reduce(|acc, x| acc * x)
                    .unwrap();
                Self(mat)
            }
        }
    }

    pub fn matrix(&self) -> Matrix4<i32> {
        self.0
    }

    pub fn to_f64_mat(self) -> Matrix4<f64> {
        let mut mat_f64: Matrix4<f64> = self.0.map(|v| v as f64);
        mat_f64
            .column_mut(3)
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| {
                if i < 3 {
                    *v /= SEITZ_TRANSLATE_BASE_NUMBER as f64
                }
            });
        mat_f64
    }

    pub fn try_inverse(&self) -> Option<Self> {
        let mut inv = self.to_f64_mat().try_inverse()?;
        inv.column_mut(3).iter_mut().enumerate().for_each(|(i, v)| {
            if i < 3 {
                *v *= SEITZ_TRANSLATE_BASE_NUMBER as f64
            }
        });
        Some(Self(inv.map(|v| v as i32)))
    }
}

impl Add for SeitzMatrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for SeitzMatrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for SeitzMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Display for SeitzMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_f64_mat())
    }
}

impl MatrixSymbol {
    fn get_rotation_matrix(&self) -> Result<Matrix4<i32>, MatrixSymbolError> {
        let fold = self.nfold_body;
        let diag = self.nfold_diag;
        let axis = self.rotation_axis;
        match (fold, diag, axis) {
            (NFold::N1, _, _) => Ok(Matrix4::identity()),
            (NFold::N2, NFoldDiag::None, RotationAxis::X) => {
                Rotation::<2, '_', 'x'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::None, RotationAxis::Y) => {
                Rotation::<2, '_', 'y'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::None, RotationAxis::Z) => {
                Rotation::<2, '_', 'z'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::None, RotationAxis::Omitted) => {
                Err(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::SingleQuote, RotationAxis::Omitted) => {
                Err(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::SingleQuote, RotationAxis::X) => {
                Rotation::<2, '\'', 'x'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::SingleQuote, RotationAxis::Y) => {
                Rotation::<2, '\'', 'y'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::SingleQuote, RotationAxis::Z) => {
                Rotation::<2, '\'', 'z'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::DoubleQuote, RotationAxis::Omitted) => {
                Err(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::DoubleQuote, RotationAxis::X) => {
                Rotation::<2, '"', 'x'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::DoubleQuote, RotationAxis::Y) => {
                Rotation::<2, '"', 'y'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::DoubleQuote, RotationAxis::Z) => {
                Rotation::<2, '"', 'z'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N2, NFoldDiag::Asterisk, _) => Err(MatrixSymbolError::Invalid(self)),
            (NFold::N3, NFoldDiag::None, RotationAxis::Omitted) => {
                Err(MatrixSymbolError::Invalid(self))
            }
            (NFold::N3, NFoldDiag::None, RotationAxis::X) => {
                Rotation::<3, '_', 'x'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N3, NFoldDiag::None, RotationAxis::Y) => {
                Rotation::<3, '_', 'y'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N3, NFoldDiag::None, RotationAxis::Z) => {
                Rotation::<3, '_', 'z'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N3, NFoldDiag::SingleQuote, _) => Err(MatrixSymbolError::Invalid(self)),
            (NFold::N3, NFoldDiag::DoubleQuote, _) => Err(MatrixSymbolError::Invalid(self)),
            (NFold::N3, NFoldDiag::Asterisk, _) => {
                Rotation::<3, '*', '_'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N4, NFoldDiag::None, RotationAxis::Omitted) => {
                Err(MatrixSymbolError::Invalid(self))
            }
            (NFold::N4, NFoldDiag::None, RotationAxis::X) => {
                Rotation::<4, '_', 'x'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N4, NFoldDiag::None, RotationAxis::Y) => {
                Rotation::<4, '_', 'y'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N4, NFoldDiag::None, RotationAxis::Z) => {
                Rotation::<4, '_', 'z'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N4, _, _) => Err(MatrixSymbolError::Invalid(self)),
            (NFold::N6, NFoldDiag::None, RotationAxis::Omitted) => {
                Err(MatrixSymbolError::Invalid(self))
            }
            (NFold::N6, NFoldDiag::None, RotationAxis::X) => {
                Rotation::<6, '_', 'x'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N6, NFoldDiag::None, RotationAxis::Y) => {
                Rotation::<6, '_', 'y'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N6, NFoldDiag::None, RotationAxis::Z) => {
                Rotation::<6, '_', 'z'>::rotation_matrix().ok_or(MatrixSymbolError::Invalid(self))
            }
            (NFold::N6, _, _) => Err(MatrixSymbolError::Invalid(self)),
        }
    }
    fn set_transform(&self, mut rot_mat: Matrix4<i32>) -> Matrix4<i32> {
        match &self.translation_symbols {
            Some(translations) => {
                let final_translations: Vector3<i32> = translations
                    .iter()
                    .map(|sym| sym.translation_vector())
                    .sum::<Vector3<i32>>()
                    .map(|v| if !(0..12).contains(&v) { v % 12 } else { v });
                let mut t_col = final_translations.to_homogeneous();
                t_col.w = 1;
                rot_mat.set_column(3, &t_col);
                rot_mat
            }
            None => rot_mat,
        }
    }
    pub fn seitz_matrix(&self) -> Result<SeitzMatrix, MatrixSymbolError> {
        let rot_mat = self.get_rotation_matrix()?;
        Ok(SeitzMatrix(self.set_transform(rot_mat)))
    }
}

struct Rotation<const N: u8, const D: char, const A: char>;

trait RotationMatrix {
    fn rotation_matrix() -> Option<Matrix4<i32>>;
}
