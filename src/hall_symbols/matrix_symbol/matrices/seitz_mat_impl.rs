use fraction::GenericFraction;
use nalgebra::{Matrix3, Matrix4, Vector3};

use crate::hall_symbols::{matrix_symbol::RotationType, SEITZ_TRANSLATE_BASE_NUMBER};
use std::{
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

impl SeitzMatrix {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
    pub fn new(v: Matrix4<i32>) -> Self {
        Self(v)
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
                let mut new_v = *v % SEITZ_TRANSLATE_BASE_NUMBER;
                if new_v < 0 {
                    new_v += SEITZ_TRANSLATE_BASE_NUMBER;
                }
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
                if *v < 0 {
                    *v += SEITZ_TRANSLATE_BASE_NUMBER;
                }
            }
        });
        Self(res)
    }
}

impl Display for SeitzMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fraction())
    }
}

impl PartialOrd for SeitzMatrix {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SeitzMatrix {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rotation_type()
            .unwrap()
            .cmp(&other.rotation_type().unwrap())
    }
}
