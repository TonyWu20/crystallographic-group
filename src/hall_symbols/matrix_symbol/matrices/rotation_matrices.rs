use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};

use crate::hall_symbols::matrix_symbol::{NFold, NFoldDiag, NFoldSub, RotationAxis};
use crate::hall_symbols::SEITZ_TRANSLATE_BASE_NUMBER;

use super::{MatrixSymbol, MatrixSymbolError};
use super::{Rotation, RotationMatrix};

impl<const D: char, const A: char> RotationMatrix for Rotation<1, D, A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        Some(Matrix4::identity())
    }
}

impl<const A: char> RotationMatrix for Rotation<2, '_', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        match A {
            'x' => Some(Matrix4::from_diagonal(&Vector4::new(1, -1, -1, 1))),
            'y' => Some(Matrix4::from_diagonal(&Vector4::new(-1, 1, -1, 1))),
            'z' => Some(Matrix4::from_diagonal(&Vector4::new(-1, -1, 1, 1))),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<3, '_', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [[1, 0, 0, 0], [0, 0, 1, 0], [0, -1, -1, 0], [0, 0, 0, 1]].map(Vector4::from);
        let y = [[-1, 0, -1, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1]].map(Vector4::from);
        let z = [[0, 1, 0, 0], [-1, -1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]].map(Vector4::from);
        match A {
            'x' => Some(Matrix4::from_columns(&x)),
            'y' => Some(Matrix4::from_columns(&y)),
            'z' => Some(Matrix4::from_columns(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<4, '_', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [
            1, 0, 0, 0, //x
            0, 0, 1, 0, //y
            0, -1, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let y = [
            0, 0, -1, 0, //x
            0, 1, 0, 0, //y
            1, 0, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let z = [
            0, 1, 0, 0, //x
            -1, 0, 0, 0, //y
            0, 0, 1, 0, //z
            0, 0, 0, 1, //t
        ];
        match A {
            'x' => Some(Matrix4::from_column_slice(&x)),
            'y' => Some(Matrix4::from_column_slice(&y)),
            'z' => Some(Matrix4::from_column_slice(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<6, '_', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [
            1, 0, 0, 0, //x
            0, 1, 1, 0, //y
            0, -1, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let y = [
            0, 0, -1, 0, //x
            0, 1, 0, 0, //y
            1, 0, 1, 0, //z
            0, 0, 0, 1, //t
        ];
        let z = [
            1, 1, 0, 0, //x
            -1, 0, 0, 0, //y
            0, 0, 1, 0, //z
            0, 0, 0, 1, //t
        ];
        match A {
            'x' => Some(Matrix4::from_column_slice(&x)),
            'y' => Some(Matrix4::from_column_slice(&y)),
            'z' => Some(Matrix4::from_column_slice(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<2, '\'', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [
            -1, 0, 0, 0, //x
            0, 0, -1, 0, //y
            0, -1, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let y = [
            0, 0, -1, 0, //x
            0, -1, 0, 0, //y
            -1, 0, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let z = [
            0, -1, 0, 0, //x
            -1, 0, 0, 0, //y
            0, 0, -1, 0, //z
            0, 0, 0, 1, //t
        ];
        match A {
            'x' => Some(Matrix4::from_column_slice(&x)),
            'y' => Some(Matrix4::from_column_slice(&y)),
            'z' => Some(Matrix4::from_column_slice(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<2, '"', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [
            -1, 0, 0, 0, //x
            0, 0, 1, 0, //y
            0, 1, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let y = [
            0, 0, 1, 0, //x
            0, -1, 0, 0, //y
            1, 0, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let z = [
            0, 1, 0, 0, //x
            1, 0, 0, 0, //y
            0, 0, -1, 0, //z
            0, 0, 0, 1, //t
        ];
        match A {
            'x' => Some(Matrix4::from_column_slice(&x)),
            'y' => Some(Matrix4::from_column_slice(&y)),
            'z' => Some(Matrix4::from_column_slice(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<3, '*', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        Some(Matrix4::from_column_slice(&[
            0, 1, 0, 0, // x
            0, 0, 1, 0, // y
            1, 0, 0, 0, // z
            0, 0, 0, 1, // t
        ]))
    }
}

impl MatrixSymbol {
    pub(crate) fn get_rotation_matrix(&self) -> Result<Matrix4<i32>, MatrixSymbolError> {
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
            (NFold::Invalid, _, _) => Err(MatrixSymbolError::Invalid(self)),
        }
    }

    pub(crate) fn set_inversion(&self, rot_mat: Matrix4<i32>) -> Matrix4<i32> {
        let inversion_mat = Matrix3::<i32>::from_diagonal_element(-1);
        let tr = rot_mat.column(3);
        let mut new_mat = (inversion_mat * rot_mat.fixed_resize::<3, 3>(1)).to_homogeneous();
        new_mat.set_column(3, &tr);
        new_mat
    }

    pub(crate) fn set_translation_from_symbols(
        &self,
        mut rot_mat: Matrix4<i32>,
    ) -> Result<Matrix4<i32>, MatrixSymbolError> {
        match &self.translation_symbols {
            Some(translations) => {
                let final_translations: Vector3<i32> = translations
                    .iter()
                    .map(|sym| sym.translation_vector())
                    .sum::<Vector3<i32>>()
                    .map(|v| {
                        if !(0..12).contains(&v) {
                            let new_v = v % 12;
                            if new_v < 0 {
                                new_v + SEITZ_TRANSLATE_BASE_NUMBER
                            } else {
                                new_v
                            }
                        } else {
                            v
                        }
                    });
                let t_col = final_translations.to_homogeneous();
                rot_mat.set_column(3, &(rot_mat.column(3) + t_col));
                Ok(rot_mat)
            }
            None => Ok(rot_mat),
        }
    }

    pub(crate) fn set_transform(
        &self,
        mut rot_mat: Matrix4<i32>,
    ) -> Result<Matrix4<i32>, MatrixSymbolError> {
        match (self.nfold_body, self.nfold_sub) {
            (NFold::Invalid, _) => Err(MatrixSymbolError::Invalid(self)),
            (NFold::N1, _) => Ok(rot_mat),
            (NFold::N2, _) => Ok(rot_mat),
            (NFold::N3, NFoldSub::N1) => {
                let t_col = match self.rotation_axis {
                    RotationAxis::Omitted => Err(MatrixSymbolError::Invalid(self)),
                    RotationAxis::X => Ok(Vector4::new(4, 0, 0, 1)),
                    RotationAxis::Y => Ok(Vector4::new(0, 4, 0, 1)),
                    RotationAxis::Z => Ok(Vector4::new(0, 0, 4, 1)),
                }?;
                rot_mat.set_column(3, &t_col);
                Ok(rot_mat)
            }
            (NFold::N3, NFoldSub::N2) => {
                let t_col = match self.rotation_axis {
                    RotationAxis::Omitted => Err(MatrixSymbolError::Invalid(self)),
                    RotationAxis::X => Ok(Vector4::new(8, 0, 0, 1)),
                    RotationAxis::Y => Ok(Vector4::new(0, 8, 0, 1)),
                    RotationAxis::Z => Ok(Vector4::new(0, 0, 8, 1)),
                }?;
                rot_mat.set_column(3, &t_col);
                Ok(rot_mat)
            }
            (NFold::N3, NFoldSub::None) => Ok(rot_mat),
            (NFold::N3, _) => Err(MatrixSymbolError::Invalid(self)),
            (NFold::N4, NFoldSub::None) => Ok(rot_mat),
            (NFold::N4, NFoldSub::N1) => {
                let t_col = match self.rotation_axis {
                    RotationAxis::Omitted => Err(MatrixSymbolError::Invalid(self)),
                    RotationAxis::X => Ok(Vector4::new(3, 0, 0, 1)),
                    RotationAxis::Y => Ok(Vector4::new(0, 3, 0, 1)),
                    RotationAxis::Z => Ok(Vector4::new(0, 0, 3, 1)),
                }?;
                rot_mat.set_column(3, &t_col);
                Ok(rot_mat)
            }
            (NFold::N4, NFoldSub::N3) => {
                let t_col = match self.rotation_axis {
                    RotationAxis::Omitted => Err(MatrixSymbolError::Invalid(self)),
                    RotationAxis::X => Ok(Vector4::new(9, 0, 0, 1)),
                    RotationAxis::Y => Ok(Vector4::new(0, 9, 0, 1)),
                    RotationAxis::Z => Ok(Vector4::new(0, 0, 9, 1)),
                }?;
                rot_mat.set_column(3, &t_col);
                Ok(rot_mat)
            }
            (NFold::N4, _) => Err(MatrixSymbolError::Invalid(self)),
            (NFold::N6, NFoldSub::None) => Ok(rot_mat),
            (NFold::N6, NFoldSub::N1) => {
                let t_col = match self.rotation_axis {
                    RotationAxis::Omitted => Err(MatrixSymbolError::Invalid(self)),
                    RotationAxis::X => Ok(Vector4::new(2, 0, 0, 1)),
                    RotationAxis::Y => Ok(Vector4::new(0, 2, 0, 1)),
                    RotationAxis::Z => Ok(Vector4::new(0, 0, 2, 1)),
                }?;
                rot_mat.set_column(3, &t_col);
                Ok(rot_mat)
            }
            (NFold::N6, NFoldSub::N2) => {
                let t_col = match self.rotation_axis {
                    RotationAxis::Omitted => Err(MatrixSymbolError::Invalid(self)),
                    RotationAxis::X => Ok(Vector4::new(4, 0, 0, 1)),
                    RotationAxis::Y => Ok(Vector4::new(0, 4, 0, 1)),
                    RotationAxis::Z => Ok(Vector4::new(0, 0, 4, 1)),
                }?;
                rot_mat.set_column(3, &t_col);
                Ok(rot_mat)
            }
            (NFold::N6, NFoldSub::N3) => Err(MatrixSymbolError::Invalid(self)),
            (NFold::N6, NFoldSub::N4) => {
                let t_col = match self.rotation_axis {
                    RotationAxis::Omitted => Err(MatrixSymbolError::Invalid(self)),
                    RotationAxis::X => Ok(Vector4::new(8, 0, 0, 1)),
                    RotationAxis::Y => Ok(Vector4::new(0, 8, 0, 1)),
                    RotationAxis::Z => Ok(Vector4::new(0, 0, 8, 1)),
                }?;
                rot_mat.set_column(3, &t_col);
                Ok(rot_mat)
            }
            (NFold::N6, NFoldSub::N5) => {
                let t_col = match self.rotation_axis {
                    RotationAxis::Omitted => Err(MatrixSymbolError::Invalid(self)),
                    RotationAxis::X => Ok(Vector4::new(10, 0, 0, 1)),
                    RotationAxis::Y => Ok(Vector4::new(0, 10, 0, 1)),
                    RotationAxis::Z => Ok(Vector4::new(0, 0, 10, 1)),
                }?;
                rot_mat.set_column(3, &t_col);
                Ok(rot_mat)
            }
        }
    }
}
