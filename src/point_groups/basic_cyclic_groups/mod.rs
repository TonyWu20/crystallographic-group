use std::marker::PhantomData;

use nalgebra::{Matrix4, Vector3};

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
    matrix: Matrix4<f64>,
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
    fn translate_matrix(translate_vector: Vector3<f64>) -> Matrix4<f64> {
        Matrix4::new_translation(&translate_vector)
    }
    pub fn to_a(self) -> Self {
        let tran_a = Self::translate_matrix(Vector3::new(1.0 / 2.0, 0.0, 0.0));
        Self {
            matrix: tran_a * self.matrix,
            ..self
        }
    }
    pub fn to_b(self) -> Self {
        let tran_b = Self::translate_matrix(Vector3::new(0.0, 1.0 / 2.0, 0.0));
        Self {
            matrix: tran_b * self.matrix,
            ..self
        }
    }
    pub fn to_c(self) -> Self {
        let tran_c = Self::translate_matrix(Vector3::new(0.0, 0.0, 1.0 / 2.0));
        Self {
            matrix: tran_c * self.matrix,
            ..self
        }
    }
    pub fn to_n(self) -> Self {
        let tran_n = Self::translate_matrix(Vector3::new(1.0 / 2.0, 1.0 / 2.0, 1.0 / 2.0));
        Self {
            matrix: tran_n * self.matrix,
            ..self
        }
    }
    pub fn to_u(self) -> Self {
        let tran_n = Self::translate_matrix(Vector3::new(1.0 / 4.0, 0.0, 0.0));
        Self {
            matrix: tran_n * self.matrix,
            ..self
        }
    }
    pub fn to_v(self) -> Self {
        let tran_v = Self::translate_matrix(Vector3::new(0.0, 1.0 / 4.0, 0.0));
        Self {
            matrix: tran_v * self.matrix,
            ..self
        }
    }
    pub fn to_w(self) -> Self {
        let tran_v = Self::translate_matrix(Vector3::new(0.0, 0.0, 1.0 / 4.0));
        Self {
            matrix: tran_v * self.matrix,
            ..self
        }
    }
    pub fn to_d(self) -> Self {
        let tran_v = Self::translate_matrix(Vector3::new(1.0 / 4.0, 1.0 / 4.0, 1.0 / 4.0));
        Self {
            matrix: tran_v * self.matrix,
            ..self
        }
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
            matrix: Matrix4::identity(),
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
            matrix: Matrix4::from_diagonal_element(-1.0),
            order: 2,
            symbol: -1,
            direction: DirectionBuilder::new().zero(),
            basis: PhantomData,
        }
    }
}

impl<'a, T: Basis, U: Axis> IntoIterator for &'a CyclicGroup<T, U> {
    type Item = Matrix4<f64>;

    type IntoIter = CyclicGroupIter<'a, T, U>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct CyclicGroupIter<'a, T: Basis, U: Axis> {
    curr_element: Matrix4<f64>,
    group: &'a CyclicGroup<T, U>,
    count: u8,
}

impl<'a, T: Basis, U: Axis> CyclicGroupIter<'a, T, U> {
    fn new(group: &'a CyclicGroup<T, U>) -> Self {
        Self {
            curr_element: Matrix4::identity(),
            group,
            count: 0,
        }
    }
}

impl<'a, T: Basis, U: Axis> Iterator for CyclicGroupIter<'a, T, U> {
    type Item = Matrix4<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.group.order {
            let res = self.curr_element;
            self.count += 1;
            self.curr_element *= self.group.matrix;
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

    use nalgebra::{Matrix4, Vector3};

    use crate::{crystal_symmetry_directions::DirectionBuilder, HexBasis, Standard};

    use super::GroupBuilder;

    #[test]
    fn test_rot_generator() {
        let z_axis = DirectionBuilder::<HexBasis>::new().c();
        let ab = DirectionBuilder::<HexBasis>::new().ab();
        let a_b = DirectionBuilder::<HexBasis>::new().a_b();
        let r3_001 = GroupBuilder::<HexBasis, 3>::new().c3(&z_axis);
        let r2_ab = GroupBuilder::<HexBasis, 2>::new().c2_face_diag(&ab);
        let r2_a_b = GroupBuilder::<HexBasis, 2>::new().c2_face_diag(&a_b);
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
        println!("{}", r3_001.matrix);
        println!("{}", r2_ab.matrix);
        println!("{}", r2_a_b.matrix);
        println!("Standard basis");
        println!("{}", r2_z.matrix);
        println!("{}", r2_s_ab.matrix);
        println!("{}", r3_111.matrix);
        println!(
            "{} at {:?}: {}",
            r4_010.order,
            r4_010.direction.hkl(),
            r4_010.matrix
        );
        println!(
            "{} at {:?}: {}",
            r2_010.order,
            r2_010.direction.hkl(),
            r2_010.matrix
        );
    }
    #[test]
    fn test_iter() {
        let axis_h001 = DirectionBuilder::<HexBasis>::new().c();
        let r6_h001 = GroupBuilder::<HexBasis, 6>::new().c6(&axis_h001);
        r6_h001.into_iter().for_each(|m| println!("{}", m));
        let m_010: Vec<Matrix4<f64>> = GroupBuilder::<Standard, -2>::new()
            .m(&DirectionBuilder::<Standard>::new().b())
            .iter()
            .collect();
        println!("{:?}", m_010);
    }
    #[test]
    fn test_translate() {
        let a = Matrix4::new_translation(&Vector3::new(0.5, 0.0, 0.0));
        let r3_001 =
            GroupBuilder::<HexBasis, 3>::new().c3(&DirectionBuilder::<HexBasis>::new().c());
        let r3a_001 = a * r3_001.matrix;
        println!("{}", r3a_001);
        println!("{}", r3a_001 * r3a_001);
        println!("{}", r3a_001 * r3a_001 * r3a_001);
    }
}
