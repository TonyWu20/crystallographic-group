use nalgebra::{Matrix4, Vector3};

use crate::{
    impl_space_group_builder,
    point_groups::{Gm, G2},
    CyclicGroup, GroupBuilder, Monoclinic, PointGroup, Standard, M,
};
use std::marker::PhantomData;

use super::{
    bravais_lattices::{C, P},
    short_symbols::SpaceGroupShortSymbol,
    SpaceGroup, SpaceGroupBuilder,
};

// pub type P2 = SpaceGroupShortSymbol<Monoclinic, Standard, 0, 2, 0, 0, 0, 0>;
// pub type P2_1 = SpaceGroupShortSymbol<Monoclinic, Standard, 0, 2, 0, 0, 1, 0>;
// pub type C2 = SpaceGroupShortSymbol<Monoclinic, Standard, 0, 2, 0, 1, 1, 0>;

// impl PointGroup<Monoclinic, Standard, 0, 2, 0> {
//     pub fn p2(&self) -> P2 {
//         SpaceGroupShortSymbol::new(self.generators().into())
//     }
//     pub fn p2_1(&self) -> P2_1 {
//         let mut generators: Vec<CyclicGroup<Standard>> = self.generators().into();
//         generators[0] = generators[0].to_b();
//         SpaceGroupShortSymbol::new(generators)
//     }
//     pub fn c2(&self) -> C2 {
//         let mut generators: Vec<CyclicGroup<Standard>> = self.generators().into();
//         let translate = GroupBuilder::<Standard, 1>::new().translate(Vector3::new(0.5, 0.5, 0.0));
//         generators.push(translate);
//         SpaceGroupShortSymbol::new(generators)
//     }
// }

// pub type Pm = SpaceGroupShortSymbol<Monoclinic, Standard, M, 0, 0, 0, 0, 0>;
// pub type Pc = SpaceGroupShortSymbol<Monoclinic, Standard, M, 0, 0, 0, 0, 1>;

// impl PointGroup<Monoclinic, Standard, M, 0, 0> {
//     pub fn pm(&self) -> Pm {
//         SpaceGroupShortSymbol::new(self.generators().into())
//     }
//     pub fn pc(&self) -> Pc {
//         let mut generators: Vec<CyclicGroup<Standard>> = self.generators().into();
//         generators[0] = generators[0].to_c();
//         SpaceGroupShortSymbol::new(generators)
//     }
// }

// impl_space_group_builder!(Monoclinic, P, G2, p2, "P2", p2);
// impl_space_group_builder!(Monoclinic, P, G2, p2_1, "P2_1", p2_1);

// impl SpaceGroupBuilder<Monoclinic, C, G2> {
//     pub fn c2(&self) -> SpaceGroup<Monoclinic, C> {
//         let set_1 = self.point_group.c2().generator_combo_matrices();
//         let translate = GroupBuilder::<Standard, 1>::new()
//             .translate(Vector3::new(0.5, 0.5, 0.0))
//             .matrix();
//         let set_2: Vec<Matrix4<f64>> = set_1.iter().map(|m| translate * m).collect();
//         let all_set = vec![set_1, set_2];
//         let all_set = all_set.concat();
//         SpaceGroup {
//             generators: all_set,
//             symbol: "C2".into(),
//             system: PhantomData,
//             bravais: PhantomData,
//         }
//     }
// }

impl SpaceGroupBuilder<Monoclinic, P, Gm> {
    pub fn pm(&self) -> SpaceGroup<Monoclinic, P> {
        todo!()
    }
    pub fn build_pc(&self) -> SpaceGroup<Monoclinic, P> {
        todo!()
    }
}

impl SpaceGroupBuilder<Monoclinic, C, Gm> {
    pub fn build(&self) -> SpaceGroup<Monoclinic, C> {
        todo!()
    }
    pub fn build_cc(&self) -> SpaceGroup<Monoclinic, C> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        space_groups::bravais_lattices::{C, P},
        Monoclinic, PointGroupBuilder, SpaceGroup, Standard,
    };

    #[test]
    fn test_mono() {
        // let g2 = PointGroupBuilder::<Monoclinic, Standard>::new().g2();
        // let p2 = SpaceGroup::<Monoclinic, P>::builder()
        //     .with_point_group(&g2)
        //     .p2();
        // let p2_1 = SpaceGroup::<Monoclinic, P>::builder()
        //     .with_point_group(&g2)
        //     .p2_1();
        // let c2 = SpaceGroup::<Monoclinic, C>::builder()
        //     .with_point_group(&g2)
        //     .c2();
        // println!("P2");
        // p2.generators.iter().for_each(|g| println!("{}", g));
        // println!("P2_1");
        // p2_1.generators.iter().for_each(|g| println!("{}", g));
        // println!("C2");
        // c2.generators.iter().for_each(|g| println!("{}", g));
    }
}
