use std::marker::PhantomData;

use nalgebra::{Matrix4, Vector3};

use crate::{Basis, CrystalSystem, CyclicGroup, Generators, GroupBuilder, SpaceGroupProperties};

use super::bravais_lattices::{BravaisLattice, A, P};

#[derive(Debug, Clone)]
pub struct SpaceGroupShortSymbol<
    T: CrystalSystem,
    S: Basis,
    G: BravaisLattice,
    const A: i8,
    const B: i8,
    const C: i8,
    const U: u8,
    const V: u8,
    const W: u8,
> {
    generators: Vec<CyclicGroup<S>>,
    system: PhantomData<T>,
    bravais_lattices: PhantomData<G>,
}

impl<
        T: CrystalSystem,
        S: Basis,
        G: BravaisLattice,
        const A: i8,
        const B: i8,
        const C: i8,
        const U: u8,
        const V: u8,
        const W: u8,
    > SpaceGroupShortSymbol<T, S, G, { A }, B, C, U, V, W>
{
    pub fn new(generators: Vec<CyclicGroup<S>>) -> Self {
        Self {
            generators,
            system: PhantomData,
            bravais_lattices: PhantomData,
        }
    }
    pub fn generators(&self) -> &[CyclicGroup<S>] {
        self.generators.as_ref()
    }
}

impl<
        T: CrystalSystem,
        S: Basis,
        const A: i8,
        const B: i8,
        const C: i8,
        const U: u8,
        const V: u8,
        const W: u8,
    > SpaceGroupShortSymbol<T, S, P, { A }, B, C, U, V, W>
{
    pub fn generators_sets(&self) -> Vec<Matrix4<f64>> {
        let lattice_coordinates = P::new().lattice_coordinates();
        let translations: Vec<CyclicGroup<S>> = lattice_coordinates
            .iter()
            .map(|coord| -> CyclicGroup<S> {
                GroupBuilder::<S, 1>::new().translate(Vector3::from_vec(coord.to_vec()))
            })
            .collect();
        let all_generators = vec![self.generators().to_vec(), translations];
        let all_generators = all_generators.concat();
        let products: Generators = all_generators.iter().map(|g| g.iter().into()).product();
        products.matrices().to_vec()
    }
}

impl<
        T: CrystalSystem,
        S: Basis,
        const A1: i8,
        const B: i8,
        const C: i8,
        const U: u8,
        const V: u8,
        const W: u8,
    > SpaceGroupShortSymbol<T, S, A, A1, B, C, U, V, W>
{
    pub fn generators_sets(&self) -> Vec<CyclicGroup<S>> {
        let lattice_coordinates = A::new().lattice_coordinates();
        let translations: Vec<CyclicGroup<S>> = lattice_coordinates
            .iter()
            .map(|coord| -> CyclicGroup<S> {
                GroupBuilder::<S, 1>::new().translate(Vector3::from_vec(coord.to_vec()))
            })
            .collect();
        let all_generators = vec![self.generators().to_vec(), translations];
        all_generators.concat()
    }
}
