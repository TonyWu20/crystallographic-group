use crate::{
    point_groups::{Gc, Gm, G2},
    Monoclinic,
};

use super::{
    bravais_lattices::{C, P},
    SpaceGroup, SpaceGroupBuilder,
};

impl SpaceGroupBuilder<Monoclinic, P, G2> {
    pub fn build(&self) -> SpaceGroup<Monoclinic, P> {
        todo!();
    }
    pub fn build_p2_1(&self) -> SpaceGroup<Monoclinic, P> {
        todo!();
    }
}

impl SpaceGroupBuilder<Monoclinic, C, G2> {
    pub fn build_c2(&self) -> SpaceGroup<Monoclinic, C> {
        todo!();
    }
}

impl SpaceGroupBuilder<Monoclinic, P, Gm> {
    pub fn build(&self) -> SpaceGroup<Monoclinic, P> {
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
