use std::ops::Mul;

use itertools::Itertools;
use nalgebra::Matrix4;

use crate::{crystal_symmetry_directions::Axis, Basis};

use self::basic_cyclic_groups::CyclicGroup;

mod basic_cyclic_groups;

pub struct PointGroup {
    elements: Vec<Matrix4<f64>>,
    symbol: String,
}

impl<T1: Basis, U1: Axis, T2: Basis, U2: Axis> Mul<CyclicGroup<T2, U2>> for CyclicGroup<T1, U1> {
    type Output = PointGroup;

    fn mul(self, rhs: CyclicGroup<T2, U2>) -> Self::Output {
        let ops_g1: Vec<Matrix4<f64>> = self.iter().collect();
        let ops_g2: Vec<Matrix4<f64>> = rhs.iter().collect();
        let g1_g2 = ops_g2
            .iter()
            .cartesian_product(ops_g1.iter())
            .map(|(a, b)| a * b)
            .collect();
        PointGroup {
            elements: g1_g2,
            symbol: "N/A".into(),
        }
    }
}

impl<T: Basis, U: Axis> Mul<CyclicGroup<T, U>> for PointGroup {
    type Output = PointGroup;

    fn mul(self, rhs: CyclicGroup<T, U>) -> Self::Output {
        let g1g2: Vec<Matrix4<f64>> = self.elements;
        let g3: Vec<Matrix4<f64>> = rhs.iter().collect();
        let g1g2_g3 = g3
            .iter()
            .cartesian_product(g1g2.iter())
            .map(|(a, b)| a * b)
            .collect();
        PointGroup {
            elements: g1g2_g3,
            symbol: "N/A".into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{crystal_symmetry_directions::DirectionBuilder, HexBasis, Standard};

    use super::basic_cyclic_groups::GroupBuilder;

    #[test]
    fn test_group_product() {
        let r2_001 =
            GroupBuilder::<Standard, 2>::new().c2(&DirectionBuilder::<Standard>::new().c());
        let r2_010 =
            GroupBuilder::<Standard, 2>::new().c2(&DirectionBuilder::<Standard>::new().b());
        let r3_111 = GroupBuilder::<Standard, 3>::new()
            .c3(&DirectionBuilder::<Standard>::new().cubic_diagonal());
        let r2_110 =
            GroupBuilder::<Standard, 2>::new().c2(&DirectionBuilder::<Standard>::new().ab());
        // 2_001 * 2_010 * 3+_111 * 2_110
        let p432 = r2_001 * r2_010 * r3_111 * r2_110;
        p432.elements.iter().for_each(|m| println!("{}", m));
        let m_1_10 =
            GroupBuilder::<Standard, -2>::new().m(&DirectionBuilder::<Standard>::new().a_b());
        let p_43m = r2_001 * r2_010 * r3_111 * m_1_10;
        println!("P -43m");
        p_43m.elements.iter().for_each(|m| println!("{}", m));
        let r3_h001 =
            GroupBuilder::<HexBasis, 3>::new().c3(&DirectionBuilder::<HexBasis>::new().c());
        let c1 = GroupBuilder::<HexBasis, -2>::new()
            .m(&DirectionBuilder::<HexBasis>::new().ab())
            .to_c();
        let p3c1 = r3_h001 * c1;
        println!("P 3c1");
        p3c1.elements.iter().for_each(|m| println!("{}", m));
    }
    #[test]
    fn p6322() {
        let r3_h001 =
            GroupBuilder::<HexBasis, 3>::new().c3(&DirectionBuilder::<HexBasis>::new().c());
        let c2_h001 = GroupBuilder::<HexBasis, 2>::new()
            .c2_principal(&DirectionBuilder::<HexBasis>::new().c())
            .to_c();
        let c2_h110 = GroupBuilder::<HexBasis, 2>::new()
            .c2_face_diag(&DirectionBuilder::<HexBasis>::new().ab());
        let p6322 = r3_h001 * c2_h001 * c2_h110;
        println!("P 6_3 22");
        p6322.elements.iter().for_each(|m| println!("{}", m));
    }
}
