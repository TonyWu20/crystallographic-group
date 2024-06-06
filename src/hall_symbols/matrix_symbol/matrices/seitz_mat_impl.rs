use fraction::{GenericFraction, Zero};
use nalgebra::{Matrix3, Matrix4, Vector3};

use crate::hall_symbols::{
    matrix_symbol::RotationType, SymmetryElement, SEITZ_TRANSLATE_BASE_NUMBER,
};
use std::{
    cmp::Ordering,
    fmt::Display,
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use super::{SeitzMatrix, SeitzMatrixError};

impl Hash for SeitzMatrix {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for SeitzMatrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rotation_part() == other.rotation_part() {
            let tr_part = self.translation_part();
            let tr_part_rhs = other.translation_part();
            (tr_part - tr_part_rhs)
                .iter()
                .all(|v| v % SEITZ_TRANSLATE_BASE_NUMBER == 0)
        } else {
            false
        }
    }
}

impl SymmetryElement for SeitzMatrix {
    fn equiv_num(&self) -> usize {
        let rotation_order = self.rotation_type().expect("Invalid Seitz Matrix");
        match rotation_order {
            RotationType::E => 1,
            RotationType::N2 => 2,
            RotationType::N3 => 3,
            RotationType::N4 => 4,
            RotationType::N6 => 6,
            RotationType::I => 1,
            RotationType::M => 2,
            RotationType::M3 => 3,
            RotationType::M4 => 4,
            RotationType::M6 => 6,
        }
    }
}

impl SeitzMatrix {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
    pub fn inversion() -> Self {
        Self(Matrix3::identity().map(|v: i32| -v).to_homogeneous())
    }
    pub fn new(v: Matrix4<i32>) -> Self {
        Self(v)
    }
    pub fn is_unique_rotation(&self, reference: &Self) -> bool {
        self.rotation_part() != reference.rotation_part()
            && self.rotation_part().map(|v| -v) != reference.rotation_part()
    }
    pub fn eigenvector(&self) -> Vector3<i32> {
        let m = self.rotation_part();
        let det = self.det();
        let m_i = m - Matrix3::<i32>::identity().map(|v| v * det);
        let eigen_trails: Vec<Vector3<i32>> = (-1..2)
            .rev()
            .flat_map(|i| {
                (-1..2)
                    .rev()
                    .flat_map(|j| {
                        (-1..2)
                            .rev()
                            .filter_map(|k| {
                                let eigen_trial = Vector3::new(i, j, k);
                                if (m_i * eigen_trial) == Vector3::zeros() && [0, 0, 0] != [i, j, k]
                                {
                                    Some(eigen_trial)
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
        choice
    }
    pub fn rotation_type(&self) -> Result<RotationType, SeitzMatrixError> {
        let det = self.det();
        let trace = self.trace();
        match (det, trace) {
            (-1, -3) => Ok(RotationType::I),
            (-1, -2) => Ok(RotationType::M6),
            (-1, -1) => Ok(RotationType::M4),
            (-1, 0) => Ok(RotationType::M3),
            (-1, 1) => Ok(RotationType::M),
            (1, -1) => Ok(RotationType::N2),
            (1, 0) => Ok(RotationType::N3),
            (1, 1) => Ok(RotationType::N4),
            (1, 2) => Ok(RotationType::N6),
            (1, 3) => Ok(RotationType::E),
            _ => Err(SeitzMatrixError::NotRotationMatrix(self.matrix())),
        }
    }

    pub(crate) fn proper_rotation(&self) -> Option<Matrix3<i32>> {
        if self
            .rotation_type()
            .is_ok_and(|typ| !matches!(typ, RotationType::E | RotationType::I))
        {
            let det = self.det();
            let rotation = self.0.fixed_resize::<3, 3>(1).map(|v| v * det);
            Some(rotation)
        } else {
            None
        }
    }

    fn det(&self) -> i32 {
        self.to_f64_mat().fixed_resize::<3, 3>(1.0).determinant() as i32
    }

    fn trace(&self) -> i32 {
        self.0.trace() - 1
    }

    // Property of cyclic group
    pub fn powi(&self, exponent: i32) -> Self {
        match exponent.cmp(&0) {
            Ordering::Less => {
                // All the transformations related here have det ± 1
                // Inverse matrix is guaranteed.
                let inv = self.try_inverse().unwrap();
                let mat = (0..exponent.abs())
                    .map(|_| inv.matrix())
                    .reduce(|acc, x| acc * x)
                    .unwrap();
                Self(mat)
            }
            Ordering::Equal => Self(Matrix4::identity()),
            Ordering::Greater => {
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

    pub fn to_fraction(self) -> Matrix4<GenericFraction<i32>> {
        let mut mat_frac: Matrix4<GenericFraction<i32>> = self.0.map(GenericFraction::<i32>::from);
        mat_frac
            .column_mut(3)
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| {
                if i < 3 {
                    *v /= GenericFraction::<i32>::from(SEITZ_TRANSLATE_BASE_NUMBER);
                }
            });
        mat_frac
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
    pub fn rotation_part(&self) -> Matrix3<i32> {
        self.0.fixed_resize::<3, 3>(1)
    }
    pub fn translation_part(&self) -> Vector3<i32> {
        self.0.column(3).xyz()
    }
    pub fn set_translation_part(&mut self, new_translation: Vector3<i32>) {
        let mut new_column = new_translation.to_homogeneous();
        new_column.fill_row(3, 1);
        self.0.set_column(3, &new_column)
    }
    fn rotation_jf_repr(&self) -> Vec<String> {
        let rotation_part = self
            .rotation_part()
            .column_iter()
            .enumerate()
            .map(|(i, v)| {
                v.map(|val| match i {
                    0 => 'x' as i32 * val,
                    1 => 'y' as i32 * val,
                    2 => 'z' as i32 * val,
                    _ => '0' as i32 * val,
                })
                .map(|val| {
                    let c = char::from_u32(val.unsigned_abs()).unwrap();
                    match val.cmp(&0) {
                        Ordering::Less => format!("-{c}"),
                        Ordering::Equal => "0".into(),
                        Ordering::Greater => format!("+{c}"),
                    }
                })
            })
            .collect::<Vec<Vector3<String>>>();
        let rotation_mat = Matrix3::from_columns(&rotation_part);
        let rotation_xyz = rotation_mat
            .row_iter()
            .map(|row| {
                row.iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .concat()
                    .replace('0', "")
            })
            .map(|s| s.trim_start_matches('+').to_string())
            .collect::<Vec<String>>();
        rotation_xyz
    }
    pub fn jones_faithful_repr_rot(&self) -> String {
        self.rotation_jf_repr().join(",")
    }
    pub fn jones_faithful_repr(&self) -> String {
        let rotation_xyz = self.rotation_jf_repr();
        let tr_part = self
            .translation_part()
            .map(|v| {
                if (-SEITZ_TRANSLATE_BASE_NUMBER + 1..SEITZ_TRANSLATE_BASE_NUMBER).contains(&v) {
                    if v >= 0 {
                        GenericFraction::<i32>::new(v, SEITZ_TRANSLATE_BASE_NUMBER)
                    } else {
                        GenericFraction::<i32>::new_neg(v.abs(), SEITZ_TRANSLATE_BASE_NUMBER)
                    }
                } else {
                    let new_v = v % SEITZ_TRANSLATE_BASE_NUMBER;
                    // if new_v < 0 {
                    //     GenericFraction::<i32>::new(
                    //         v + SEITZ_TRANSLATE_BASE_NUMBER,
                    //         SEITZ_TRANSLATE_BASE_NUMBER,
                    //     )
                    // } else {
                    if new_v >= 0 {
                        GenericFraction::<i32>::new(new_v, SEITZ_TRANSLATE_BASE_NUMBER)
                    } else {
                        GenericFraction::<i32>::new_neg(new_v.abs(), SEITZ_TRANSLATE_BASE_NUMBER)
                    }
                    // }
                }
            })
            .iter()
            .map(|v| match v.cmp(&GenericFraction::zero()) {
                Ordering::Less => format!("{v}"),
                Ordering::Equal => String::new(),
                Ordering::Greater => format!("+{v}"),
            })
            .collect::<Vec<String>>();
        let faithful_repr = rotation_xyz
            .iter()
            .zip(tr_part.iter())
            .map(|(r, t)| format!("{r}{t}"))
            .collect::<Vec<String>>()
            .join(",");
        faithful_repr
    }
}

impl Add for SeitzMatrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Vector3<i32>> for SeitzMatrix {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Vector3<i32>) -> Self::Output {
        let mut mat = self.0;
        let mut column = mat.column(3) + rhs.to_homogeneous();
        column.iter_mut().enumerate().for_each(|(i, v)| {
            if i < 3 {
                let new_v = *v % SEITZ_TRANSLATE_BASE_NUMBER;
                *v = new_v;
            }
        });
        mat.set_column(3, &column);
        Self(mat)
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

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self.0 * rhs.0;
        res.column_mut(3).iter_mut().enumerate().for_each(|(i, v)| {
            if i < 3 {
                *v %= SEITZ_TRANSLATE_BASE_NUMBER;
            }
        });
        Self(res)
    }
}

impl Display for SeitzMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {:?}\n{}",
            self.jones_faithful_repr(),
            self.eigenvector(),
            self.to_fraction()
        )
    }
}

/// For cubic, tetragonal, orthorhombic, monoclinic and triclinic crystal systems
pub(crate) const ORDER_48: [&str; 48] = [
    "x,y,z", "-x,-y,z", "-x,y,-z", "x,-y,-z", "z,x,y", "z,-x,-y", "-z,-x,y", "-z,x,-y", "y,z,x",
    "-y,z,-x", "y,-z,-x", "-y,-z,x", "y,x,-z", "-y,-x,-z", "y,-x,z", "-y,x,z", "x,z,-y", "-x,z,y",
    "-x,-z,-y", "x,-z,y", "z,y,-x", "z,-y,x", "-z,y,x", "-z,-y,-x", "-x,-y,-z", "x,y,-z", "x,-y,z",
    "-x,y,z", "-z,-x,-y", "-z,x,y", "z,x,-y", "z,-x,y", "-y,-z,-x", "y,-z,x", "-y,z,x", "y,z,-x",
    "-y,-x,z", "y,x,z", "-y,x,-z", "y,-x,-z", "-x,-z,y", "x,-z,-y", "x,z,y", "-x,z,-y", "-z,-y,x",
    "-z,y,-x", "z,-y,-x", "z,y,x",
];

/// For hexagonal and trigonal crystal systems
pub(crate) const ORDER_24: [&str; 24] = [
    "x,y,z",
    "-y,x-y,z",
    "-x+y,-x,z",
    "-x,-y,z",
    "y,-x+y,z",
    "x-y,x,z",
    "y,x,-z",
    "x-y,-y,-z",
    "-x,-x+y,-z",
    "-y,-x,-z",
    "-x+y,y,-z",
    "x,x-y,-z",
    "-x,-y,-z",
    "y,-x+y,-z",
    "x-y,x,-z",
    "x,y,-z",
    "-y,x-y,-z",
    "-x+y,-x,-z",
    "-y,-x,z",
    "-x+y,y,z",
    "x,x-y,z",
    "y,x,z",
    "x-y,-y,z",
    "-x,-x+y,z",
];

/// For rhombohedral system
pub(crate) const ORDER_12: [&str; 12] = [
    "x,y,z", "z,x,y", "y,z,x", "-z,-y,-x", "-y,-x,-z", "-x,-z,-y", "-x,-y,-z", "-z,-x,-y",
    "-y,-z,-x", "z,y,x", "y,x,z", "x,z,y",
];
