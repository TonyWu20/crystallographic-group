use nalgebra::Matrix4;

mod basic_cyclic_groups;
mod point_group_symbols;
pub use basic_cyclic_groups::{CyclicGroup, CyclicGroupIter, Generators, GroupBuilder};

use crate::CrystalSystem;

pub use self::point_group_symbols::*;

pub struct SymmetryGroup {
    pub(crate) elements: Vec<Matrix4<f64>>,
}

pub trait PointGroupSymbol<T: CrystalSystem> {
    fn symbol(&self) -> String;
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
        let r3_111 =
            GroupBuilder::<Standard, 3>::new().c3(&DirectionBuilder::<Standard>::new().abc());
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
            .m_face_diag(&DirectionBuilder::<HexBasis>::new().ab())
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
