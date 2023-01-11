use std::{marker::PhantomData, ops::Mul};

use nalgebra::Matrix3;

use crate::{
    crystal_symmetry_directions::{Axis, DirectionBuilder, RealAxis, Universal, D},
    Basis,
};

mod hex_basis;
mod standard_basis;

/// The basis cyclic group, representing: 1,2,3,4,6, -1, m, -3, -4 and -6
#[derive(Clone, Copy)]
pub struct CyclicGroup<T: Basis, U: Axis> {
    /// The basic element `g`
    generator: Matrix3<i8>,
    /// Order of the group
    order: u8,
    /// Symmetry Direction
    direction: D<T, U>,
    /// Basis of coordinates
    basis: PhantomData<T>,
}

/// The builder struct to limit the scope of valid crystallographic cyclic group
/// # Generics
/// - `const N`: `u8` - the operation symbol
/// - `T`: Basis - mark the coordinate basis system.
pub struct GroupBuilder<T: Basis, const N: i8>(PhantomData<T>);

impl<T: Basis, const N: i8> GroupBuilder<T, N> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// The operation E, identity for both basis.
impl<T: Basis> GroupBuilder<T, 1> {
    pub fn e(&self) -> CyclicGroup<T, Universal> {
        CyclicGroup {
            generator: Matrix3::identity(),
            order: 1,
            direction: DirectionBuilder::new().zero(),
            basis: PhantomData,
        }
    }
}

/// The operation I, inversion for both basis.
impl<T: Basis> GroupBuilder<T, -1> {
    pub fn i(&self) -> CyclicGroup<T, Universal> {
        CyclicGroup {
            generator: Matrix3::from_diagonal_element(-1),
            order: 1,
            direction: DirectionBuilder::new().zero(),
            basis: PhantomData,
        }
    }
}

/// Multiply with E or I.
impl<T: Basis, U: RealAxis> Mul<CyclicGroup<T, Universal>> for CyclicGroup<T, U> {
    type Output = Self;

    fn mul(self, rhs: CyclicGroup<T, Universal>) -> Self::Output {
        let new_generator = rhs.generator * self.generator;
        Self {
            generator: new_generator,
            order: self.order,
            direction: self.direction,
            basis: PhantomData,
        }
    }
}

impl<T: Basis, U: RealAxis> Mul<CyclicGroup<T, U>> for CyclicGroup<T, Universal> {
    type Output = CyclicGroup<T, U>;

    fn mul(self, rhs: CyclicGroup<T, U>) -> Self::Output {
        rhs * self
    }
}

// impl Mul for CyclicGroup {
//     type Output = PointGroup;
//
//     fn mul(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }

#[cfg(test)]
mod test {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

    use nalgebra::{Matrix3, Rotation3, Unit, Vector3};

    use crate::{crystal_symmetry_directions::DirectionBuilder, HexBasis, Standard};

    use super::GroupBuilder;

    #[test]
    fn test_rot_generator() {
        let z_axis = DirectionBuilder::<HexBasis>::new().c();
        let ab = DirectionBuilder::<HexBasis>::new().ab();
        let r3_001 = GroupBuilder::<HexBasis, 3>::new().c3(&z_axis);
        let r2_ab = GroupBuilder::<HexBasis, 2>::new().m2(&ab);
        let r_i = GroupBuilder::<HexBasis, -1>::new().i();
        let ri3_001 = r_i * r3_001;
        let z_axis = DirectionBuilder::<Standard>::new().c();
        let r2_z = GroupBuilder::<Standard, 2>::new().c2(&z_axis);
        let standard_ab = DirectionBuilder::<Standard>::new().ab();
        let r2_s_ab = GroupBuilder::<Standard, 2>::new().c2(&standard_ab);
        let axis_111 = DirectionBuilder::<Standard>::new().cubic_diagonal();
        let axis_010 = DirectionBuilder::<Standard>::new().b();
        let r3_111 = GroupBuilder::<Standard, 3>::new().c3(&axis_111);
        let r4_010 = GroupBuilder::<Standard, 4>::new().c4(&z_axis);
        let r2_010 = GroupBuilder::<Standard, 2>::new().c2(&axis_010);
        println!("Hexagonal basis");
        println!(
            "{}, {}, {}",
            r3_001.generator, ri3_001.generator, r2_ab.generator
        );
        println!("Standard basis");
        println!("{}", r2_z.generator);
        println!("{}", r2_s_ab.generator);
        println!("{}", r3_111.generator);
        println!(
            "{} at {:?}: {}",
            r4_010.order,
            r4_010.direction.hkl(),
            r4_010.generator
        );
        println!(
            "{} at {:?}: {}",
            r2_010.order,
            r2_010.direction.hkl(),
            r2_010.generator
        );
    }
    #[test]
    fn test_rh() {
        let theta = 2_f64 * FRAC_PI_3;
        let cos = theta.cos();
        let sin = theta.sin();
        let rh_b = Matrix3::new(-cos, -sin, 0.0, -cos, sin, 0.0, 0.0, 0.0, 1.0);
        let rh_bi = rh_b.try_inverse().unwrap();
        let axis = Unit::new_normalize(rh_b * Vector3::new(2.0, 1.0, 0.0));
        let angle = FRAC_PI_2;
        let rot = Rotation3::from_axis_angle(&axis, angle);
        let rh_rot = rh_bi * rot.matrix();
        println!("{}", rh_rot);
    }
}
