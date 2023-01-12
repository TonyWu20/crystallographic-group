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
    /// The matrix form of the hexagonal coordinate basis.
    /// The angle between x- and y-axis is 120 degrees.
    fn hexagonal_basis() -> Matrix3<f64> {
        let theta: f64 = FRAC_PI_3 * 2_f64;
        let sin = theta.sin();
        let cos = theta.cos();
        Matrix3::new(1.0, cos, 0.0, 0.0, sin, 0.0, 0.0, 0.0, 1.0)
    }
    /// The matrix form of the given n-fold rotation at given direction.
    /// The accept direction is `HexBasis` and Principal` only.
    fn matrix(direction: &D<HexBasis, Principal>) -> Matrix3<i8> {
        let angle = 2_f64 * PI / N as f64;
        let [x, y, z] = direction.hkl();
        let axis = Unit::new_normalize(Vector3::new(x as f64, y as f64, z as f64));
        let generator = Rotation3::from_axis_angle(&axis, angle);
        // The transformed coordinate basis
        let basis = Self::hexagonal_basis();
        // Inversion of the basis matrix
        let basis_i = basis.try_inverse().unwrap();
        // Convert the rotation matrix to entries of integer +1, -1, 0 only.
        let hex_gen = basis_i * generator.matrix() * basis;
        hex_gen.map(|i| i.round() as i8)
    }
}

impl GroupBuilder<HexBasis, 3> {
    /// The 3 group generator.
    pub fn c3(&self, direction: &D<HexBasis, Principal>) -> CyclicGroup<HexBasis, Principal> {
        CyclicGroup {
            generator: Self::matrix(direction),
            order: 3,
            symbol: 3,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, -3> {
    /// The -3 group generator.
    pub fn i3(&self, direction: &D<HexBasis, Principal>) -> CyclicGroup<HexBasis, Principal> {
        let c3 = GroupBuilder::<HexBasis, 3>::new().c3(direction);
        let i = GroupBuilder::<HexBasis, -1>::new().i();
        CyclicGroup {
            generator: c3.generator * i.generator,
            order: 3,
            symbol: -3,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, 6> {
    /// The 6 group generator.
    pub fn c6(&self, direction: &D<HexBasis, Principal>) -> CyclicGroup<HexBasis, Principal> {
        CyclicGroup {
            generator: Self::matrix(direction),
            order: 6,
            symbol: 6,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, -6> {
    /// The -6 group generator.
    pub fn i6(&self, direction: &D<HexBasis, Principal>) -> CyclicGroup<HexBasis, Principal> {
        let c6 = GroupBuilder::<HexBasis, 6>::new().c6(direction);
        let i = GroupBuilder::<HexBasis, -1>::new().i();

        CyclicGroup {
            generator: c6.generator * i.generator,
            order: 6,
            symbol: -6,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, 2> {
    /// The 2 group generator.
    pub fn c2(&self, direction: &D<HexBasis, FaceDiagonal>) -> CyclicGroup<HexBasis, FaceDiagonal> {
        CyclicGroup {
            generator: Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
            order: 2,
            symbol: 2,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, -2> {
    /// The m group generator.
    pub fn m(&self, direction: &D<HexBasis, FaceDiagonal>) -> CyclicGroup<HexBasis, FaceDiagonal> {
        // C2
        let c2 = GroupBuilder::<HexBasis, 2>::new().c2(direction);
        // Inversion
        let i = GroupBuilder::<HexBasis, -1>::new().i();
        // I * C2 = M
        CyclicGroup {
            generator: c2.generator * i.generator,
            order: 2,
            symbol: -2,
            direction: *direction,
            basis: PhantomData,
        }
    }
}
