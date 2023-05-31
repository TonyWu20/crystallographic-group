//! Definition and implementation of the 32 crystallographic point groups.
use std::{fmt::Display, marker::PhantomData};

use nalgebra::Matrix4;

use crate::{
    crystal_symmetry_directions::{ABAxis, ABCAxis, ABmAxis, BAxis, CAxis},
    Basis, CrystalSystem, Cubic, Generators, HexBasis, Hexagonal, Monoclinic, Orthorhombic,
    Standard, Tetragonal, Triclinic, Trigonal,
};

use super::{CyclicGroup, GroupBuilder, PointGroupSymbol};

#[derive(Debug, Clone)]
pub struct PointGroup<T: CrystalSystem, U: Basis, const A: i8, const B: i8, const C: i8> {
    generators: Vec<CyclicGroup<U>>,
    system: PhantomData<T>,
}

impl<T, U, const A: i8, const B: i8, const C: i8> PointGroup<T, U, A, B, C>
where
    T: CrystalSystem,
    U: Basis,
{
    pub fn get_symbol(&self) -> String {
        let to_symbols = vec![match_lookup(A), match_lookup(B), match_lookup(C)];
        to_symbols.concat()
    }

    pub fn generators(&self) -> &[CyclicGroup<U>] {
        self.generators.as_ref()
    }

    pub fn generator_combo_matrices(&self) -> Vec<Matrix4<f64>> {
        let combos: Generators = self
            .generators()
            .iter()
            .map(|g| -> Generators { g.iter().into() })
            .product();
        combos.matrices().to_vec()
    }
}
impl<T, U, const A: i8, const B: i8, const C: i8> PointGroupSymbol<T> for PointGroup<T, U, A, B, C>
where
    T: CrystalSystem,
    U: Basis,
{
    fn symbol(&self) -> String {
        self.get_symbol()
    }
}

impl<T, U, const A: i8, const B: i8, const C: i8> Display for PointGroup<T, U, A, B, C>
where
    T: CrystalSystem,
    U: Basis,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

fn match_lookup(value: i8) -> String {
    match value {
        1 => "1".into(),
        2 => "2".into(),
        3 => "3".into(),
        4 => "4".into(),
        6 => "6".into(),
        -1 => "-1".into(),
        M => "m".into(),
        MC => "c".into(),
        -3 => "-3".into(),
        -4 => "-4".into(),
        -6 => "-6".into(),
        S2M => "2/m".into(),
        S4M => "4/m".into(),
        S6M => "6/m".into(),
        _ => "".into(),
    }
}

pub struct PointGroupBuilder<T: CrystalSystem, U: Basis>(PhantomData<T>, PhantomData<U>);

impl<T: CrystalSystem, U: Basis> PointGroupBuilder<T, U> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
    fn symmetry_group<const N: i8>(&self) -> GroupBuilder<U, N> {
        GroupBuilder::<U, N>::new()
    }
    fn i() -> CyclicGroup<U> {
        GroupBuilder::<U, -1>::new().i()
    }
}

pub const M: i8 = -2;
pub const MC: i8 = -23;
pub const S2M: i8 = 20;
pub const S4M: i8 = 40;
pub const S6M: i8 = 60;

/// Triclinic system
pub type G1 = PointGroup<Triclinic, Standard, 1, 0, 0>;
pub type GI1 = PointGroup<Triclinic, Standard, -1, 0, 0>;

impl PointGroupBuilder<Triclinic, Standard> {
    pub fn g1(&self) -> G1 {
        let generator = GroupBuilder::<Standard, 1>::new().e();
        let generators = vec![generator];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gi1(&self) -> GI1 {
        let generator = GroupBuilder::<Standard, -1>::new().i();
        let generators = vec![generator];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
}

/// Monoclinic System
pub type G2 = PointGroup<Monoclinic, Standard, 0, 2, 0>;
pub type Gm = PointGroup<Monoclinic, Standard, M, 0, 0>;
pub type Gc = PointGroup<Monoclinic, Standard, MC, 0, 0>;
pub type G2m = PointGroup<Monoclinic, Standard, S2M, 0, 0>;

impl PointGroupBuilder<Monoclinic, Standard> {
    pub fn g2(&self) -> G2 {
        let generators = vec![GroupBuilder::<Standard, 2>::new().c2(&BAxis::new())];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gm(&self) -> Gm {
        let generators = vec![GroupBuilder::<Standard, -2>::new().m(&BAxis::new())];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g2m() -> G2m {
        let r2_010 = GroupBuilder::<Standard, 2>::new().c2(&BAxis::new());
        let i = GroupBuilder::<Standard, -1>::new().i();
        let generators = vec![r2_010, i];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
}

/// Orthorhombic system
pub type G222 = PointGroup<Orthorhombic, Standard, 2, 2, 2>;
pub type Gmm2 = PointGroup<Orthorhombic, Standard, M, M, 2>;
pub type G2m2m2m = PointGroup<Orthorhombic, Standard, S2M, S2M, S2M>;

impl PointGroupBuilder<Orthorhombic, Standard> {
    pub fn g222(&self) -> G222 {
        let generators = vec![
            GroupBuilder::<Standard, 2>::new().c2(&CAxis::new()),
            GroupBuilder::<Standard, 2>::new().c2(&BAxis::new()),
        ];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gmm2(&self) -> Gmm2 {
        let generators = vec![
            GroupBuilder::<Standard, 2>::new().c2(&CAxis::new()),
            GroupBuilder::<Standard, -2>::new().m(&BAxis::new()),
        ];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g2m2m2m(&self) -> G2m2m2m {
        let generators = vec![
            GroupBuilder::<Standard, 2>::new().c2(&CAxis::new()),
            GroupBuilder::<Standard, 2>::new().c2(&BAxis::new()),
            GroupBuilder::<Standard, -1>::new().i(),
        ];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
}

/// Tetragonal system
pub type G4 = PointGroup<Tetragonal, Standard, 4, 0, 0>;
pub type GI4 = PointGroup<Tetragonal, Standard, -4, 0, 0>;
pub type G4m = PointGroup<Tetragonal, Standard, S4M, 0, 0>;
pub type G422 = PointGroup<Tetragonal, Standard, 4, 2, 2>;
pub type G4mm = PointGroup<Tetragonal, Standard, 4, M, M>;
pub type GI42m = PointGroup<Tetragonal, Standard, -4, 2, M>;
pub type G4m2m2m = PointGroup<Tetragonal, Standard, S4M, S2M, S2M>;

impl PointGroupBuilder<Tetragonal, Standard> {
    fn c4() -> CyclicGroup<Standard> {
        GroupBuilder::<Standard, 4>::new().c4(&CAxis::new())
    }
    fn i4() -> CyclicGroup<Standard> {
        GroupBuilder::<Standard, -4>::new().i4(&CAxis::new())
    }
    fn c2_001() -> CyclicGroup<Standard> {
        GroupBuilder::<Standard, 2>::new().c2(&CAxis::new())
    }
    fn c2_010() -> CyclicGroup<Standard> {
        GroupBuilder::<Standard, 2>::new().c2(&BAxis::new())
    }
    fn m_010() -> CyclicGroup<Standard> {
        GroupBuilder::<Standard, -2>::new().m(&BAxis::new())
    }
    pub fn g4(&self) -> G4 {
        let generators = vec![Self::c2_001(), Self::c4()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gi4(&self) -> GI4 {
        let generators = vec![Self::c2_001(), Self::i4()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g4m(&self) -> G4m {
        let generators = vec![Self::c2_001(), Self::c4(), Self::i()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g422(&self) -> G422 {
        let generators = vec![Self::c2_001(), Self::c4(), Self::c2_010()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g4mm(&self) -> G4mm {
        let generators = vec![Self::c2_001(), Self::c4(), Self::m_010()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gi42m(&self) -> GI42m {
        let generators = vec![Self::c2_001(), Self::i4(), Self::c2_010()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g4mmm(&self) -> G4mm {
        let generators = vec![Self::c2_001(), Self::c4(), Self::c2_010(), Self::i()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
}

/// Trigonal system
pub type G3 = PointGroup<Trigonal, HexBasis, 3, 0, 0>;
pub type GI3 = PointGroup<Trigonal, HexBasis, -3, 0, 0>;
pub type G32 = PointGroup<Trigonal, HexBasis, 3, 2, 0>;
pub type G3m = PointGroup<Trigonal, HexBasis, 3, M, 0>;
pub type GI32m = PointGroup<Trigonal, HexBasis, -3, S2M, 0>;

impl PointGroupBuilder<Trigonal, HexBasis> {
    fn c3() -> CyclicGroup<HexBasis> {
        GroupBuilder::<HexBasis, 3>::new().c3(&CAxis::new())
    }
    fn c2() -> CyclicGroup<HexBasis> {
        GroupBuilder::<HexBasis, 2>::new().c2_face_diag(&ABAxis::new())
    }
    fn m() -> CyclicGroup<HexBasis> {
        GroupBuilder::<HexBasis, -2>::new().m_face_diag(&ABAxis::new())
    }
    pub fn g3(&self) -> G3 {
        let generators = vec![Self::c3()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gi3(&self) -> GI3 {
        let generators = vec![Self::c3(), Self::i()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g32(&self) -> G32 {
        let generators = vec![Self::c3(), Self::c2()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g3m(&self) -> G3m {
        let generators = vec![Self::c3(), Self::m()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gi32m(&self) -> GI32m {
        let generators = vec![Self::c3(), Self::m(), Self::i()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
}

/// Hexagonal system
pub type G6 = PointGroup<Hexagonal, HexBasis, 6, 0, 0>;
pub type GI6 = PointGroup<Hexagonal, HexBasis, -6, 0, 0>;
pub type G6m = PointGroup<Hexagonal, HexBasis, S6M, 0, 0>;
pub type G622 = PointGroup<Hexagonal, HexBasis, 6, 2, 2>;
pub type G6mm = PointGroup<Hexagonal, HexBasis, 6, M, M>;
pub type GI6M2 = PointGroup<Hexagonal, HexBasis, -6, M, 2>;
pub type G6m2m2m = PointGroup<Hexagonal, HexBasis, S6M, S2M, S2M>;

impl PointGroupBuilder<Hexagonal, HexBasis> {
    fn c3() -> CyclicGroup<HexBasis> {
        GroupBuilder::<HexBasis, 3>::new().c3(&CAxis::new())
    }
    fn c2_001() -> CyclicGroup<HexBasis> {
        GroupBuilder::<HexBasis, 2>::new().c2_principal(&CAxis::new())
    }
    fn m_001() -> CyclicGroup<HexBasis> {
        GroupBuilder::<HexBasis, -2>::new().m_principal(&CAxis::new())
    }
    fn c2_110() -> CyclicGroup<HexBasis> {
        GroupBuilder::<HexBasis, 2>::new().c2_face_diag(&ABAxis::new())
    }
    fn m_110() -> CyclicGroup<HexBasis> {
        GroupBuilder::<HexBasis, -2>::new().m_face_diag(&ABAxis::new())
    }
    pub fn g6(&self) -> G6 {
        let generators = vec![Self::c3(), Self::c2_001()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gi6(&self) -> GI6 {
        let generators = vec![Self::c3(), Self::m_001()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g6m(&self) -> G6m {
        let generators = vec![Self::c3(), Self::c2_001(), Self::i()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g622(&self) -> G622 {
        let generators = vec![Self::c3(), Self::c2_001(), Self::c2_110()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g6mm(&self) -> G6mm {
        let generators = vec![Self::c3(), Self::c2_001(), Self::m_110()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gi6m2(&self) -> GI6M2 {
        let generators = vec![Self::c3(), Self::m_001(), Self::m_110()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g6m2m2m(&self) -> G6m2m2m {
        let generators = vec![Self::c3(), Self::c2_001(), Self::c2_110(), Self::i()];
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
}

/// Cubic system
pub type G23 = PointGroup<Cubic, Standard, 2, 3, 0>;
pub type GmI3 = PointGroup<Cubic, Standard, S2M, -3, 0>;
pub type G432 = PointGroup<Cubic, Standard, 4, 3, 2>;
pub type GI43m = PointGroup<Cubic, Standard, -4, 3, M>;
pub type GmI3m = PointGroup<Cubic, Standard, S4M, -3, S2M>;

impl PointGroupBuilder<Cubic, Standard> {
    fn common() -> Vec<CyclicGroup<Standard>> {
        vec![
            GroupBuilder::<Standard, 2>::new().c2(&CAxis::new()),
            GroupBuilder::<Standard, 2>::new().c2(&BAxis::new()),
            GroupBuilder::<Standard, 3>::new().c3(&ABCAxis::new()),
        ]
    }
    pub fn g23(&self) -> G23 {
        PointGroup {
            generators: Self::common(),
            system: PhantomData,
        }
    }
    pub fn gmi3(&self) -> GmI3 {
        let mut generators = Self::common();
        generators.push(Self::i());
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn g432(&self) -> G432 {
        let mut generators = Self::common();
        generators.push(GroupBuilder::<Standard, 2>::new().c2(&ABAxis::new()));
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gi43m(&self) -> GI43m {
        let mut generators = Self::common();
        generators.push(GroupBuilder::<Standard, -2>::new().m(&ABmAxis::new()));
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
    pub fn gmi3m(&self) -> GmI3m {
        let mut generators = Self::common();
        generators.push(GroupBuilder::<Standard, 2>::new().c2(&ABAxis::new()));
        generators.push(Self::i());
        PointGroup {
            generators,
            system: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        point_groups::point_group_symbols::PointGroupBuilder, Cubic, Monoclinic, Standard,
        Triclinic,
    };

    #[test]
    fn test_point_group() {
        let g1 = PointGroupBuilder::<Triclinic, Standard>::new().g1();
        let g2 = PointGroupBuilder::<Monoclinic, Standard>::new().g2();
        let g3 = PointGroupBuilder::<Cubic, Standard>::new().g432();
        println!("{}", g1.get_symbol());
        g1.generators
            .iter()
            .for_each(|g| println!("{}", g.notation()));
        println!("{}", g2.get_symbol());
        g2.generators
            .iter()
            .for_each(|g| println!("{}", g.notation()));
        println!("{}", g3.get_symbol());
        g3.generators
            .iter()
            .for_each(|g| println!("{}", g.notation()));
    }
}
