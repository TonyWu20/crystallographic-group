use std::{iter::Product, marker::PhantomData, ops::Mul};

use itertools::Itertools;
use nalgebra::{Matrix4, Vector3};

use crate::Basis;

use super::SymmetryGroup;

mod hex_basis;
mod standard_basis;

/// The basis cyclic group, representing: 1,2,3,4,6, -1, m, -3, -4 and -6
#[derive(Debug, Clone, Copy)]
pub struct CyclicGroup<T: Basis> {
    /// The basic element `g`
    matrix: Matrix4<f64>,
    /// Order of the group
    order: u8,
    symbol: i8,
    direction: [i8; 3],
    /// Basis of coordinates
    basis: PhantomData<T>,
}

#[derive(Clone)]
pub struct Generators {
    matrices: Vec<Matrix4<f64>>,
    count: u8,
    len: u8,
}

impl Generators {
    pub fn matrices(&self) -> &[Matrix4<f64>] {
        self.matrices.as_ref()
    }
}

impl<T: Basis> CyclicGroup<T> {
    pub fn iter(&self) -> CyclicGroupIter<T> {
        CyclicGroupIter::new(self)
    }
    fn translate_matrix(translate_vector: Vector3<f64>) -> Matrix4<f64> {
        Matrix4::new_translation(&translate_vector)
    }
    pub fn notation(&self) -> String {
        let [h, k, l] = self.direction;
        format!("{}_{}{}{}", self.symbol, h, k, l)
    }
    /// Glide translation along half the lattice vector `a` of this face.
    pub fn to_a(self) -> Self {
        let tran_a = Self::translate_matrix(Vector3::new(1.0 / 2.0, 0.0, 0.0));
        Self {
            matrix: tran_a * self.matrix,
            ..self
        }
    }
    /// Glide translation along half the lattice vector `b` of this face.
    pub fn to_b(self) -> Self {
        let tran_b = Self::translate_matrix(Vector3::new(0.0, 1.0 / 2.0, 0.0));
        Self {
            matrix: tran_b * self.matrix,
            ..self
        }
    }
    /// Glide translation along half the lattice vector `c` of this face.
    pub fn to_c(self) -> Self {
        let tran_c = Self::translate_matrix(Vector3::new(0.0, 0.0, 1.0 / 2.0));
        Self {
            matrix: tran_c * self.matrix,
            ..self
        }
    }
    /// Glide translation along half the diagonal of this face
    pub fn to_n(self) -> Self {
        let tran_n = Self::translate_matrix(Vector3::new(1.0 / 2.0, 1.0 / 2.0, 1.0 / 2.0));
        Self {
            matrix: tran_n * self.matrix,
            ..self
        }
    }
    /// Glide translations along quarter of the lattice vector `a` of this face
    pub fn to_u(self) -> Self {
        let tran_n = Self::translate_matrix(Vector3::new(1.0 / 4.0, 0.0, 0.0));
        Self {
            matrix: tran_n * self.matrix,
            ..self
        }
    }
    /// Glide translations along quarter of the lattice vector `b` of this face
    pub fn to_v(self) -> Self {
        let tran_v = Self::translate_matrix(Vector3::new(0.0, 1.0 / 4.0, 0.0));
        Self {
            matrix: tran_v * self.matrix,
            ..self
        }
    }
    /// Glide translations along quarter of the lattice vector `c` of this face
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

    pub fn matrix(&self) -> Matrix4<f64> {
        self.matrix
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
    pub(crate) fn translate_matrix(translate_vector: Vector3<f64>) -> Matrix4<f64> {
        Matrix4::new_translation(&translate_vector)
    }
}

/// The operation E, identity for both basis.
impl<T: Basis> GroupBuilder<T, 1> {
    pub fn e(&self) -> CyclicGroup<T> {
        CyclicGroup {
            matrix: Matrix4::identity(),
            order: 1,
            symbol: 1,
            direction: [0, 0, 0],
            basis: PhantomData,
        }
    }
    pub fn translate(&self, translate_vector: Vector3<f64>) -> CyclicGroup<T> {
        CyclicGroup {
            matrix: Self::translate_matrix(translate_vector),
            order: 1,
            symbol: 1,
            direction: [0, 0, 0],
            basis: PhantomData,
        }
    }
}

/// The operation I, inversion for both basis.
impl<T: Basis> GroupBuilder<T, -1> {
    pub fn i(&self) -> CyclicGroup<T> {
        CyclicGroup {
            matrix: Matrix4::from_diagonal_element(-1.0),
            order: 2,
            symbol: -1,
            direction: [0, 0, 0],
            basis: PhantomData,
        }
    }
}

impl<'a, T: Basis> IntoIterator for &'a CyclicGroup<T> {
    type Item = Matrix4<f64>;

    type IntoIter = CyclicGroupIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct CyclicGroupIter<'a, T: Basis> {
    curr_element: Matrix4<f64>,
    group: &'a CyclicGroup<T>,
    count: u8,
}

impl<'a, T: Basis> CyclicGroupIter<'a, T> {
    fn new(group: &'a CyclicGroup<T>) -> Self {
        Self {
            curr_element: Matrix4::identity(),
            group,
            count: 0,
        }
    }
}

impl<'a, T: Basis> Iterator for CyclicGroupIter<'a, T> {
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

impl<'a, T: Basis> ExactSizeIterator for CyclicGroupIter<'a, T> {
    fn len(&self) -> usize {
        (self.group.order - self.count) as usize
    }
}

impl<'a, T: Basis> From<CyclicGroupIter<'a, T>> for Generators {
    fn from(value: CyclicGroupIter<'a, T>) -> Self {
        let len = value.len() as u8;
        Self {
            matrices: value.collect(),
            count: 0,
            len,
        }
    }
}

impl<T1: Basis, T2: Basis> Mul<CyclicGroup<T2>> for CyclicGroup<T1> {
    type Output = SymmetryGroup;

    fn mul(self, rhs: CyclicGroup<T2>) -> Self::Output {
        let ops_g1: Vec<Matrix4<f64>> = self.iter().collect();
        let ops_g2: Vec<Matrix4<f64>> = rhs.iter().collect();
        let g1_g2 = ops_g2
            .iter()
            .cartesian_product(ops_g1.iter())
            .map(|(a, b)| a * b)
            .collect();
        SymmetryGroup { elements: g1_g2 }
    }
}

impl<T: Basis> Mul<CyclicGroup<T>> for SymmetryGroup {
    type Output = SymmetryGroup;

    fn mul(self, rhs: CyclicGroup<T>) -> Self::Output {
        let g1g2: Vec<Matrix4<f64>> = self.elements;
        let g3: Vec<Matrix4<f64>> = rhs.iter().collect();
        let g1g2_g3 = g3
            .iter()
            .cartesian_product(g1g2.iter())
            .map(|(a, b)| a * b)
            .collect();
        SymmetryGroup { elements: g1g2_g3 }
    }
}

impl Iterator for Generators {
    type Item = Matrix4<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.len {
            let next_item = self.matrices.get(self.count as usize).unwrap();
            self.count += 1;
            Some(*next_item)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Generators {}

impl Mul for Generators {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_len = rhs.len * self.len;
        let new_matrices: Vec<Matrix4<f64>> =
            rhs.cartesian_product(self).map(|(a, b)| a * b).collect();
        Generators {
            matrices: new_matrices,
            count: 0,
            len: new_len,
        }
    }
}

impl Product for Generators {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a * b).unwrap()
    }
}

#[cfg(test)]
mod test {

    use crate::{crystal_symmetry_directions::DirectionBuilder, HexBasis, Standard};

    use super::{Generators, GroupBuilder};

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
        let axis_111 = DirectionBuilder::<Standard>::new().abc();
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
            "{} at: {:?}: {}",
            r4_010.order, r4_010.direction, r4_010.matrix
        );
        println!(
            "{} at: {:?}: {}",
            r2_010.order, r2_010.direction, r2_010.matrix
        );
        let c = vec![r2_010, r4_010];
        let cp: Generators = c
            .iter()
            .map(|g| -> Generators { g.iter().into() })
            .product();
        cp.for_each(|m| println!("{m}"));
    }
    #[test]
    fn test_iter() {
        let axis_h001 = DirectionBuilder::<HexBasis>::new().c();
        let r6_h001 = GroupBuilder::<HexBasis, 6>::new().c6(&axis_h001);
        let gr6: Generators = r6_h001.iter().into();
        gr6.for_each(|m| println!("{m}"));
    }
}
