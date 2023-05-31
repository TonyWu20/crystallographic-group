use std::marker::PhantomData;

use crate::{
    point_groups::{G1, GI1},
    space_groups::SpaceGroupBuilder,
    Standard, Triclinic,
};

use super::{bravais_lattices::P, short_symbols::SpaceGroupShortSymbol, SpaceGroup};

pub type P1 = SpaceGroupShortSymbol<Triclinic, Standard, P, 1, 0, 0, 0, 0, 0>;
pub type PI1 = SpaceGroupShortSymbol<Triclinic, Standard, P, -1, 0, 0, 0, 0, 0>;

impl G1 {
    pub fn p1(&self) -> P1 {
        SpaceGroupShortSymbol::new(self.generators().to_vec())
    }
}

impl GI1 {
    pub fn pi1(&self) -> PI1 {
        SpaceGroupShortSymbol::new(self.generators().to_vec())
    }
}

impl SpaceGroupBuilder<Triclinic, P, G1> {
    pub fn p1(&self) -> SpaceGroup<Triclinic, P> {
        SpaceGroup {
            generators: self.point_group.p1().generators_sets(),
            symbol: "P1".into(),
            system: PhantomData,
            bravais: PhantomData,
        }
    }
}

impl SpaceGroupBuilder<Triclinic, P, GI1> {
    pub fn pi1(&self) -> SpaceGroup<Triclinic, P> {
        SpaceGroup {
            generators: self.point_group.pi1().generators_sets(),
            symbol: "P-1".into(),
            system: PhantomData,
            bravais: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        space_groups::bravais_lattices::P, PointGroupBuilder, SpaceGroup, Standard, Triclinic,
    };

    #[test]
    fn test_triclinic() {
        let g1 = PointGroupBuilder::<Triclinic, Standard>::new().g1();
        let gi1 = PointGroupBuilder::<Triclinic, Standard>::new().gi1();
        let p1 = SpaceGroup::<Triclinic, P>::builder()
            .with_point_group(&g1)
            .p1();
        p1.generators.iter().for_each(|g| println!("{}", g));
        let pi1 = SpaceGroup::<Triclinic, P>::builder()
            .with_point_group(&gi1)
            .pi1();
        pi1.generators.iter().for_each(|g| println!("{}", g));
    }
}
