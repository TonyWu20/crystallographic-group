use std::{fmt::Display, marker::PhantomData};

use crate::{
    point_groups::{GroupBuilder, SymmetryGroup},
    CrystalSystem, Standard, Triclinic,
};

use self::bravais_lattices::{BravaisLattice, P};

mod bravais_lattices;

/// Letters representing space groups

pub trait SpaceGroupProperties: Display {
    type Item;
    fn new() -> Self::Item;
    fn points_per_lattice(&self) -> u32;
    fn lattice_coordinates(&self) -> Vec<[f64; 3]>;
}

pub struct SpaceGroup<S: CrystalSystem, B: BravaisLattice> {
    generators: SymmetryGroup,
    symbol: String,
    system: PhantomData<S>,
    bravais: PhantomData<B>,
}

pub struct Undefined;
impl CrystalSystem for Undefined {}
impl BravaisLattice for Undefined {}

pub struct SpaceGroupBuilder<S: CrystalSystem, B: BravaisLattice>(PhantomData<S>, PhantomData<B>);

impl<S: CrystalSystem, B: BravaisLattice> SpaceGroupBuilder<S, B> {
    pub fn new_builder() -> SpaceGroupBuilder<Undefined, Undefined> {
        SpaceGroupBuilder(PhantomData, PhantomData)
    }
}

impl SpaceGroupBuilder<Undefined, Undefined> {
    pub fn triclinic() -> SpaceGroupBuilder<Triclinic, P> {
        SpaceGroupBuilder(PhantomData, PhantomData)
    }
}

impl SpaceGroupBuilder<Triclinic, P> {
    pub fn p1() -> SpaceGroup<Triclinic, P> {
        let e = GroupBuilder::<Standard, 1>::new().e();
        let p1 = e * e;
        SpaceGroup {
            generators: p1,
            symbol: "P1".into(),
            system: PhantomData,
            bravais: PhantomData,
        }
    }
}
