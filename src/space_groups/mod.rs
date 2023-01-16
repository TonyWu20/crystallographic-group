use std::fmt::Display;

mod bravais_lattices;

/// Letters representing space groups

pub trait SpaceGroupProperties: Display {
    type Item;
    fn new() -> Self::Item;
    fn points_per_lattice(&self) -> u32;
    fn lattice_coordinates(&self) -> Vec<[f64; 3]>;
}
