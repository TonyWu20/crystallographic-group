use core::panic;
use std::{fmt::Display, hash::Hash};

use nalgebra::Matrix4;

use super::{MatrixSymbol, MatrixSymbolError};

/// Implementation detail for `MatrixSymbol`
mod rotation_matrices;
/// Implementation detail for `SeitzMatrix`
mod seitz_mat_impl;

#[derive(Debug, Clone, Copy, Eq)]
pub struct SeitzMatrix(Matrix4<i32>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub enum SeitzMatrixError {
    NotRotationMatrix(Matrix4<i32>),
}

impl Display for SeitzMatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeitzMatrixError::NotRotationMatrix(m) => write!(f, "{m} is not a rotation matrix!"),
        }
    }
}

impl MatrixSymbol {
    pub fn seitz_matrix(&self) -> Result<SeitzMatrix, MatrixSymbolError> {
        let rot_mat = self.get_rotation_matrix()?;
        if self.minus_sign {
            let mut mat = Matrix4::<i32>::identity().map(|v| -v) * self.set_transform(rot_mat)?;
            mat.column_mut(3).w = 1;
            Ok(SeitzMatrix(mat))
        } else {
            Ok(SeitzMatrix(self.set_transform(rot_mat).unwrap_or_else(
                |_| panic!("Set transform matrix failed for {:?}", self),
            )))
        }
    }
}

struct Rotation<const N: u8, const D: char, const A: char>;

trait RotationMatrix {
    fn rotation_matrix() -> Option<Matrix4<i32>>;
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use nalgebra::Matrix4;

    use super::SeitzMatrix;

    #[test]
    fn test_sm_eq() {
        let m1 = SeitzMatrix::new(Matrix4::new(
            0, -1, 0, 0, 1, -1, 0, 0, 0, 0, 1, 2, 0, 0, 0, 1,
        ));
        let m2 = SeitzMatrix::new(Matrix4::new(
            0, -1, 0, 0, 1, -1, 0, 0, 0, 0, 1, -10, 0, 0, 0, 1,
        ));
        println!("m1^1: {m1}, m1^2: {}", m1.powi(2));
        println!("equal ? {}", m1.powi(3) == m2.powi(3));
        let m3 = SeitzMatrix::new(Matrix4::from_row_iterator([
            -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 6, 0, 0, 0, 1,
        ]));
        println!("{m3}");
        let m4 = SeitzMatrix::new(Matrix4::from_row_iterator([
            -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, -6, 0, 0, 0, 1,
        ]));
        println!("m4 equal m3? {}", m4 == m3);
        let mut set = HashSet::new();
        set.insert(m3);
        println!("{:?}", set);
        println!("{}", (11 - (-1)) % 12);
    }
}
