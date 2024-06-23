use std::{fmt::Display, hash::Hash};

use nalgebra::Matrix4;

use super::{MatrixSymbol, MatrixSymbolError};

/// Implementation detail for `MatrixSymbol`
mod rotation_matrices;
/// Implementation detail for `SeitzMatrix`
mod seitz_mat_impl;

#[derive(Debug, Clone, Copy, Eq, PartialOrd)]
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
            let transformed_mat = self
                .set_transform(rot_mat)
                .and_then(|v| self.set_translation_from_symbols(v))?;
            let mat = self.set_inversion(transformed_mat);
            Ok(SeitzMatrix(mat))
        } else {
            let mat = self
                .set_transform(rot_mat)
                .and_then(|v| self.set_translation_from_symbols(v))?;
            Ok(SeitzMatrix(mat))
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

    use nalgebra::{Matrix3, Matrix4, Vector3};

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
        let xyz = Vector3::<char>::new('x', 'y', 'z');
        let xyz_rotated = (m4.rotation_part() * xyz.map(|v| v as i32)).map(|v| {
            let c = char::from_u32(v.unsigned_abs()).unwrap();
            if v < 0 {
                format!("-{c}")
            } else {
                format!("{c}")
            }
        });
        println!("{xyz_rotated}")
    }
    #[test]
    fn test_eigen() {
        let m = Matrix3::new(
            -1_f64, 0_f64, 0_f64, 0_f64, 0_f64, 1_f64, 0_f64, 1_f64, 0_f64,
        );
        let m_i = m - Matrix3::identity();
        let lu = m_i.lu();
        println!("{}", lu.u());
        let eigen_trails: Vec<Vector3<i32>> = (-1..2)
            .flat_map(|i| {
                (-1..2)
                    .flat_map(|j| {
                        (-1..2)
                            .filter_map(|k| {
                                let eigen_trial = Vector3::new(i as f64, j as f64, k as f64);
                                if (m_i * eigen_trial).norm_squared() == 0.0
                                    && [0, 0, 0] != [i, j, k]
                                {
                                    Some(eigen_trial.map(|v| v as i32))
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<Vector3<i32>>>()
                    })
                    .collect::<Vec<Vector3<i32>>>()
            })
            .collect();
        let choice = eigen_trails
            .iter()
            .filter(|f| match f.z {
                v if v > 0 => true,
                0 => match f.y {
                    v if v > 0 => true,
                    0 => f.x > 0,
                    _ => false,
                },
                _ => false,
            })
            .cloned()
            .next()
            .unwrap();
        println!("{}", choice);
    }
}
