use std::marker::PhantomData;

use nalgebra::Matrix3;

use crate::{
    crystal_symmetry_directions::{Axis, DirectionBuilder, Universal, D},
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
    symbol: i8,
    /// Symmetry Direction
    direction: D<T, U>,
    /// Basis of coordinates
    basis: PhantomData<T>,
}

impl<T: Basis, U: Axis> CyclicGroup<T, U> {
    pub fn iter(&self) -> CyclicGroupIter<T, U> {
        CyclicGroupIter::new(self)
    }
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
            symbol: 1,
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
            order: 2,
            symbol: -1,
            direction: DirectionBuilder::new().zero(),
            basis: PhantomData,
        }
    }
}

impl<'a, T: Basis, U: Axis> IntoIterator for &'a CyclicGroup<T, U> {
    type Item = Matrix3<i8>;

    type IntoIter = CyclicGroupIter<'a, T, U>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct CyclicGroupIter<'a, T: Basis, U: Axis> {
    curr_element: Matrix3<i8>,
    group: &'a CyclicGroup<T, U>,
    count: u8,
}

impl<'a, T: Basis, U: Axis> CyclicGroupIter<'a, T, U> {
    fn new(group: &'a CyclicGroup<T, U>) -> Self {
        Self {
            curr_element: Matrix3::identity(),
            group,
            count: 0,
        }
    }
}

impl<'a, T: Basis, U: Axis> Iterator for CyclicGroupIter<'a, T, U> {
    type Item = Matrix3<i8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.group.order {
            let res = self.curr_element;
            self.count += 1;
            self.curr_element *= self.group.generator;
            Some(res)
        } else {
            None
        }
    }
}

impl<'a, T: Basis, U: Axis> ExactSizeIterator for CyclicGroupIter<'a, T, U> {
    fn len(&self) -> usize {
        (self.group.order - self.count) as usize
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

    use nalgebra::{Matrix3, Vector3};

    use crate::{crystal_symmetry_directions::DirectionBuilder, HexBasis, Standard};

    use super::GroupBuilder;

    #[test]
    fn test_rot_generator() {
        let z_axis = DirectionBuilder::<HexBasis>::new().c();
        let ab = DirectionBuilder::<HexBasis>::new().ab();
        let r3_001 = GroupBuilder::<HexBasis, 3>::new().c3(&z_axis);
        let r2_ab = GroupBuilder::<HexBasis, 2>::new().c2(&ab);
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
        println!("{}, {}", r3_001.generator, r2_ab.generator);
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
    fn test_iter() {
        let axis_h001 = DirectionBuilder::<HexBasis>::new().c();
        let r6_h001 = GroupBuilder::<HexBasis, 6>::new().c6(&axis_h001);
        r6_h001.into_iter().for_each(|m| println!("{}", m));
        let m_010: Vec<Matrix3<i8>> = GroupBuilder::<Standard, -2>::new()
            .m(&DirectionBuilder::<Standard>::new().b())
            .iter()
            .collect();
        println!("{:?}", m_010);
    }
    #[test]
    fn find_axis() {
        let r3_111 = GroupBuilder::<Standard, 3>::new()
            .c3(&DirectionBuilder::<Standard>::new().cubic_diagonal());
        let m = &r3_111.generator;
    }
}
