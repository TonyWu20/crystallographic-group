use std::{
    f64::consts::{FRAC_PI_3, PI},
    marker::PhantomData,
};

use nalgebra::{Matrix3, Rotation3, Unit, Vector3};

use crate::{
    crystal_symmetry_directions::{FaceDiagonal, Principal, D},
    HexBasis,
};

use super::{CyclicGroup, GroupBuilder};

impl<const N: i8> GroupBuilder<HexBasis, N> {
    fn hexagonal_basis() -> Matrix3<f64> {
        let theta: f64 = FRAC_PI_3 * 2_f64;
        let sin = theta.sin();
        let cos = theta.cos();
        Matrix3::new(1.0, cos, 0.0, 0.0, sin, 0.0, 0.0, 0.0, 1.0)
    }
    fn matrix(direction: &D<HexBasis, Principal>) -> Matrix3<i8> {
        let angle = 2_f64 * PI / N as f64;
        let [x, y, z] = direction.hkl();
        let axis = Unit::new_normalize(Vector3::new(x as f64, y as f64, z as f64));
        let generator = Rotation3::from_axis_angle(&axis, angle);
        let basis = Self::hexagonal_basis();
        let basis_i = basis.try_inverse().unwrap();
        let hex_gen = basis_i * generator.matrix() * basis;
        hex_gen.map(|i| i.round() as i8)
    }
}

impl GroupBuilder<HexBasis, 3> {
    pub fn c3(&self, direction: &D<HexBasis, Principal>) -> CyclicGroup<HexBasis, Principal> {
        CyclicGroup {
            generator: Self::matrix(direction),
            order: 3,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, 6> {
    pub fn c6(&self, direction: &D<HexBasis, Principal>) -> CyclicGroup<HexBasis, Principal> {
        CyclicGroup {
            generator: Self::matrix(direction),
            order: 6,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, 2> {
    pub fn m2(&self, direction: &D<HexBasis, FaceDiagonal>) -> CyclicGroup<HexBasis, FaceDiagonal> {
        CyclicGroup {
            generator: Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
            order: 2,
            direction: *direction,
            basis: PhantomData,
        }
    }
}
