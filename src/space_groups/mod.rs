use std::{fmt::Display, marker::PhantomData};

use nalgebra::Matrix4;

use crate::{
    crystal_symmetry_directions::DirectionOrder, point_groups::PointGroupSymbol, CrystalSystem,
};

use self::bravais_lattices::BravaisLattice;

mod short_symbols;

mod bravais_lattices;

mod monoclinic;
mod triclinic;

pub trait SpaceGroupProperties: Display {
    type Item;
    fn new() -> Self::Item;
    fn points_per_lattice(&self) -> u32;
    fn lattice_coordinates(&self) -> Vec<[f64; 3]>;
}

pub struct SpaceGroup<S: CrystalSystem, B: BravaisLattice> {
    generators: Vec<Matrix4<f64>>,
    symbol: String,
    system: PhantomData<S>,
    bravais: PhantomData<B>,
}

impl<S: CrystalSystem, B: BravaisLattice> SpaceGroup<S, B> {
    pub fn builder() -> SpaceGroupBuilder<S, B, Empty> {
        SpaceGroupBuilder {
            point_group: Empty,
            system: PhantomData,
            bravais: PhantomData,
        }
    }

    pub fn generators(&self) -> &[Matrix4<f64>] {
        self.generators.as_ref()
    }

    pub fn symbol(&self) -> &str {
        self.symbol.as_ref()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Undefined;
#[derive(Debug, Clone, Copy)]
pub struct Empty;
impl CrystalSystem for Undefined {}
impl BravaisLattice for Undefined {}
impl DirectionOrder for Undefined {}
impl<T: CrystalSystem> PointGroupSymbol<T> for Empty {
    fn symbol(&self) -> String {
        "".into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpaceGroupBuilder<S: CrystalSystem, B: BravaisLattice, P: PointGroupSymbol<S>> {
    point_group: P,
    system: PhantomData<S>,
    bravais: PhantomData<B>,
}

/// Macro implementations of name customized build function `build_$func_name` of `space_group_builder`
#[macro_export]
macro_rules! impl_space_group_builder {
    ($system: ty, $bravais: ty, $point_group: ty, $space_group_func: ident, $symbol: literal, $func_name: ident) => {
        impl SpaceGroupBuilder<$system, $bravais, $point_group> {
            pub fn $func_name(&self) -> SpaceGroup<$system, $bravais> {
                let generators = self
                    .point_group
                    .$space_group_func()
                    .generator_combo_matrices();
                SpaceGroup {
                    generators,
                    symbol: $symbol.into(),
                    system: PhantomData,
                    bravais: PhantomData,
                }
            }
        }
    };
}

impl<S, B> SpaceGroupBuilder<S, B, Empty>
where
    S: CrystalSystem,
    B: BravaisLattice,
{
    pub fn with_point_group<P: PointGroupSymbol<S>>(
        self,
        point_group: &P,
    ) -> SpaceGroupBuilder<S, B, P> {
        SpaceGroupBuilder {
            point_group: point_group.clone(),
            system: PhantomData,
            bravais: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{point_groups::PointGroupBuilder, Triclinic};

    use super::{bravais_lattices::P, SpaceGroup};

    #[test]
    fn test_space_group() {
        let _tri_builder =
            SpaceGroup::<Triclinic, P>::builder().with_point_group(&PointGroupBuilder::new().g1());
    }
}
