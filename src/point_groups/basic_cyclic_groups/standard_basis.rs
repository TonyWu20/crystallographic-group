use std::{f64::consts::PI, marker::PhantomData};

use nalgebra::{Matrix3, Rotation3, Unit, Vector3};

use crate::{
    crystal_symmetry_directions::{BodyDiagonal, Principal, RealAxis, D},
    Standard,
};

use super::{CyclicGroup, GroupGenerator};

impl<const N: i8> GroupGenerator<Standard, N> {
    fn matrix<U: RealAxis>(direction: &D<Standard, U>) -> Matrix3<i8> {
        let angle = 2_f64 * PI / N as f64;
        let [x, y, z] = direction.hkl();
        let axis = Unit::new_normalize(Vector3::new(x as f64, y as f64, z as f64));
        let generator = Rotation3::from_axis_angle(&axis, angle);
        generator.matrix().map(|i| i.round() as i8)
    }
}

impl GroupGenerator<Standard, 2> {
    pub fn c2<U: RealAxis>(&self, direction: &D<Standard, U>) -> CyclicGroup<Standard, U> {
        CyclicGroup {
            generator: Self::matrix(direction),
            order: 2,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupGenerator<Standard, 4> {
    pub fn c4(&self, direction: &D<Standard, Principal>) -> CyclicGroup<Standard, Principal> {
        CyclicGroup {
            generator: Self::matrix(direction),
            order: 4,
            direction: *direction,
            basis: PhantomData,
        }
    }
}

impl GroupGenerator<Standard, 3> {
    pub fn c3(&self, direction: &D<Standard, BodyDiagonal>) -> CyclicGroup<Standard, BodyDiagonal> {
        CyclicGroup {
            generator: Self::matrix(direction),
            order: 3,
            direction: *direction,
            basis: PhantomData,
        }
    }
}
