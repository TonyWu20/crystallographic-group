use std::marker::PhantomData;

use crate::{
    point_groups::{Generators, G1, GI1},
    space_groups::SpaceGroupBuilder,
    Triclinic,
};

use super::{bravais_lattices::P, SpaceGroup};

impl SpaceGroupBuilder<Triclinic, P, G1> {
    pub fn build(&self) -> SpaceGroup<Triclinic, P> {
        let p = &self.point_group;
        let combos: Generators = p
            .generators()
            .iter()
            .map(|g| -> Generators { g.iter().into() })
            .product();
        let generators = combos.matrices().to_vec();
        SpaceGroup {
            generators,
            symbol: "P1".into(),
            system: PhantomData,
            bravais: PhantomData,
        }
    }
}

impl SpaceGroupBuilder<Triclinic, P, GI1> {
    pub fn build(&self) -> SpaceGroup<Triclinic, P> {
        let combos: Generators = self
            .point_group
            .generators()
            .iter()
            .map(|g| g.iter().into())
            .product();
        let generators = combos.matrices().to_vec();
        SpaceGroup {
            generators,
            symbol: "P-1".into(),
            system: PhantomData,
            bravais: PhantomData,
        }
    }
}
