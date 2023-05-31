use std::{f64::consts::PI, marker::PhantomData};

use nalgebra::{Matrix4, Rotation3, Vector3};

use crate::{
    crystal_symmetry_directions::{BodyDiagonal, Principal, RealAxis, D},
    Standard,
};

use super::{CyclicGroup, GroupBuilder};

impl<const N: i8> GroupBuilder<Standard, N> {
    fn rotation_matrix<U: RealAxis, const H: i8, const K: i8, const L: i8>(
        direction: &D<Standard, U, H, K, L>,
    ) -> Matrix4<f64> {
        let angle = 2_f64 * PI / N as f64;
        let axis = direction.axis();
        let generator = Rotation3::from_axis_angle(&axis, angle);
        generator.matrix().map(|i| i.round()).to_homogeneous()
    }
}

impl GroupBuilder<Standard, 2> {
    pub fn c2<U: RealAxis, const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<Standard, U, H, K, L>,
    ) -> CyclicGroup<Standard> {
        CyclicGroup {
            matrix: Self::rotation_matrix(direction),
            order: 2,
            symbol: 2,
            basis: PhantomData,
            direction: [H, K, L],
        }
    }
}

impl GroupBuilder<Standard, -2> {
    pub fn m<U: RealAxis, const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<Standard, U, H, K, L>,
    ) -> CyclicGroup<Standard> {
        let c2 = GroupBuilder::<Standard, 2>::new().c2(direction);
        let i = GroupBuilder::<Standard, -1>::new().i();
        let reflected = c2.matrix * i.matrix;
        CyclicGroup {
            matrix: reflected,
            order: 2,
            symbol: -2,
            direction: [H, K, L],

            basis: PhantomData,
        }
    }
}

impl GroupBuilder<Standard, 4> {
    pub fn c4<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<Standard, Principal, H, K, L>,
    ) -> CyclicGroup<Standard> {
        CyclicGroup {
            matrix: Self::rotation_matrix(direction),
            order: 4,
            symbol: 4,
            direction: [H, K, L],

            basis: PhantomData,
        }
    }
}

impl GroupBuilder<Standard, -4> {
    pub fn i4<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<Standard, Principal, H, K, L>,
    ) -> CyclicGroup<Standard> {
        let c4 = GroupBuilder::<Standard, 4>::new().c4(direction);
        let i = GroupBuilder::<Standard, -1>::new().i();
        CyclicGroup {
            matrix: c4.matrix * i.matrix,
            order: 4,
            symbol: -4,
            direction: [H, K, L],

            basis: PhantomData,
        }
    }
}

impl GroupBuilder<Standard, 3> {
    pub fn c3<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<Standard, BodyDiagonal, H, K, L>,
    ) -> CyclicGroup<Standard> {
        CyclicGroup {
            matrix: Self::rotation_matrix(direction),
            order: 3,
            symbol: 3,
            direction: [H, K, L],
            basis: PhantomData,
        }
    }
}

impl GroupBuilder<Standard, -3> {
    pub fn i3<const H: i8, const K: i8, const L: i8>(
        &self,
        direction: &D<Standard, BodyDiagonal, H, K, L>,
    ) -> CyclicGroup<Standard> {
        let c3 = GroupBuilder::<Standard, 3>::new().c3(direction);
        let i = GroupBuilder::<Standard, -1>::new().i();
        CyclicGroup {
            matrix: c3.matrix * i.matrix,
            order: 3,
            symbol: -3,
            direction: [H, K, L],
            basis: PhantomData,
        }
    }
}
