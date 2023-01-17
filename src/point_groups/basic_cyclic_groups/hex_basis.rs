use std::{
    f64::consts::{FRAC_PI_3, PI},
    marker::PhantomData,
};

use nalgebra::{Matrix3, Matrix4, Rotation3};

use crate::{
    crystal_symmetry_directions::{DirectionBuilder, FaceDiagonal, Principal, D},
    HexBasis, Standard,
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
    fn hex_rotation_matrix<const H: i8, const K: i8, const L: i8>(
        direction: &D<HexBasis, Principal, H, K, L>,
    ) -> Matrix4<f64> {
        let generator = Self::rotation_matrix(direction);
        // The transformed coordinate basis
        let basis = Self::hexagonal_basis();
        // Inversion of the basis matrix
        let basis_i = basis.try_inverse().unwrap();
        // Convert the rotation matrix to entries of integer +1, -1, 0 only.
        let hex_gen = basis_i * generator * basis;
        hex_gen.map(|i| i.round()).to_homogeneous()
    }
    /// Original matrix in cartesian basis.
    fn rotation_matrix<const H: i8, const K: i8, const L: i8>(
        direction: &D<HexBasis, Principal, H, K, L>,
    ) -> Matrix3<f64> {
        let angle = 2_f64 * PI / N as f64;
        let axis = direction.axis();
        let generator = Rotation3::from_axis_angle(&axis, angle);
        *generator.matrix()
    }
}

impl GroupBuilder<HexBasis, 3> {
    /// The 3 group generator.
    pub fn c3<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<HexBasis, Principal, H, K, L>,
    ) -> CyclicGroup<HexBasis> {
        CyclicGroup {
            matrix: Self::hex_rotation_matrix(direction),
            order: 3,
            symbol: 3,
            direction: [H, K, L],
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, -3> {
    /// The -3 group generator.
    pub fn i3<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<HexBasis, Principal, H, K, L>,
    ) -> CyclicGroup<HexBasis> {
        let c3 = GroupBuilder::<HexBasis, 3>::new().c3(direction);
        let i = GroupBuilder::<HexBasis, -1>::new().i();
        CyclicGroup {
            matrix: c3.matrix * i.matrix,
            order: 3,
            symbol: -3,
            direction: [H, K, L],
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, 6> {
    /// The 6 group generator.
    pub fn c6<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<HexBasis, Principal, H, K, L>,
    ) -> CyclicGroup<HexBasis> {
        CyclicGroup {
            matrix: Self::hex_rotation_matrix(direction),
            order: 6,
            symbol: 6,
            direction: [H, K, L],
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, -6> {
    /// The -6 group generator.
    pub fn i6<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<HexBasis, Principal, H, K, L>,
    ) -> CyclicGroup<HexBasis> {
        let c6 = GroupBuilder::<HexBasis, 6>::new().c6(direction);
        let i = GroupBuilder::<HexBasis, -1>::new().i();

        CyclicGroup {
            matrix: c6.matrix * i.matrix,
            order: 6,
            symbol: -6,
            direction: [H, K, L],
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, 2> {
    /// The 2 group generator.
    pub fn c2_principal<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<HexBasis, Principal, H, K, L>,
    ) -> CyclicGroup<HexBasis> {
        let c2 = Self::rotation_matrix(direction)
            .map(|i| i.round())
            .to_homogeneous();
        CyclicGroup {
            matrix: c2,
            order: 2,
            symbol: 2,
            direction: [H, K, L],
            basis: PhantomData,
        }
    }
    /// Axis could be [110] or [1-10]
    pub fn c2_face_diag<const K: i8>(
        &self,
        direction: &D<HexBasis, FaceDiagonal, 1, K, 0>,
    ) -> CyclicGroup<HexBasis> {
        // Derived from cartesian C4_001 * C2_100/010
        let r4 = GroupBuilder::<Standard, 4>::new()
            .c4(&DirectionBuilder::<Standard>::new().c())
            .matrix;
        // Branch
        let generator = match direction.hkl()[1] {
            // Direction is [110], C2_110 = C4_001 * C2_100
            1 => {
                let r2 = GroupBuilder::<Standard, 2>::new()
                    .c2(&DirectionBuilder::<Standard>::new().a())
                    .matrix;
                let matrix = r4 * r2;
                matrix.map(|i| i.round())
            }
            // Direction is [110], C2_110 = C4_001 * C2_010
            -1 => {
                let r2 = GroupBuilder::<Standard, 2>::new()
                    .c2(&DirectionBuilder::<Standard>::new().b())
                    .matrix;
                let matrix = r4 * r2;
                matrix.map(|i| i.round())
            }
            // Prevented from type state of the `direction`
            _ => Matrix4::zeros(),
        };
        CyclicGroup {
            matrix: generator,
            order: 2,
            symbol: 2,
            direction: [1, K, 0],
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<HexBasis, -2> {
    /// The m group generator.
    pub fn m<const K: i8>(
        &self,
        direction: &D<HexBasis, FaceDiagonal, 1, K, 0>,
    ) -> CyclicGroup<HexBasis> {
        // C2
        let c2 = GroupBuilder::<HexBasis, 2>::new().c2_face_diag(direction);
        // Inversion
        let i = GroupBuilder::<HexBasis, -1>::new().i();
        // I * C2 = M
        CyclicGroup {
            matrix: c2.matrix * i.matrix,
            order: 2,
            symbol: -2,
            direction: [1, K, 0],
            basis: PhantomData,
        }
    }
}
