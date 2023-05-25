use std::fmt::Display;

use super::SpaceGroupProperties;

pub trait BravaisLattice {}
pub trait MonoclinicLattice: BravaisLattice {}
pub trait OrthorhombicLattice: BravaisLattice {}
pub trait TetragonalLattice: BravaisLattice {}
pub trait TrigonalLattice: BravaisLattice {}
pub trait HexagonalLattice: BravaisLattice {}
pub trait CubicLattice: BravaisLattice {}

/// Primitive
pub struct P;
impl MonoclinicLattice for P {}
impl OrthorhombicLattice for P {}
impl TetragonalLattice for P {}
impl TrigonalLattice for P {}
impl HexagonalLattice for P {}
impl CubicLattice for P {}
/// A-face centered
pub struct A;
impl OrthorhombicLattice for A {}
/// B-face centered
pub struct B;
/// C-face centered
pub struct C;
impl MonoclinicLattice for C {}
impl OrthorhombicLattice for C {}
/// Body centered
pub struct I;
impl OrthorhombicLattice for I {}
impl TetragonalLattice for I {}
impl CubicLattice for I {}
/// Face centered (all)
pub struct F;
impl OrthorhombicLattice for F {}
impl CubicLattice for F {}
/// Rhombohedral
pub struct R;
impl TrigonalLattice for R {}
/// Centered Hexagonal
pub struct H;

impl SpaceGroupProperties for P {
    type Item = P;
    fn new() -> Self::Item {
        Self
    }
    fn points_per_lattice(&self) -> u32 {
        1
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![[0.0, 0.0, 0.0]]
    }
}

impl SpaceGroupProperties for A {
    type Item = A;

    fn new() -> Self::Item {
        Self
    }

    fn points_per_lattice(&self) -> u32 {
        2
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![[0.0, 0.0, 0.0], [0.0, 1_f64 / 2_f64, 1_f64 / 2_f64]]
    }
}

impl SpaceGroupProperties for B {
    type Item = B;

    fn new() -> Self::Item {
        Self
    }

    fn points_per_lattice(&self) -> u32 {
        2
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![[0.0, 0.0, 0.0], [1_f64 / 2_f64, 0.0, 1_f64 / 2_f64]]
    }
}

impl SpaceGroupProperties for C {
    type Item = C;

    fn new() -> Self::Item {
        Self
    }

    fn points_per_lattice(&self) -> u32 {
        2
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![[0.0, 0.0, 0.0], [1_f64 / 2_f64, 1_f64 / 2_f64, 0.0]]
    }
}

impl SpaceGroupProperties for I {
    type Item = I;

    fn new() -> Self::Item {
        Self
    }

    fn points_per_lattice(&self) -> u32 {
        2
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![
            [0.0, 0.0, 0.0],
            [1_f64 / 2_f64, 1_f64 / 2_f64, 1_f64 / 2_f64],
        ]
    }
}

impl SpaceGroupProperties for F {
    type Item = F;

    fn new() -> Self::Item {
        Self
    }

    fn points_per_lattice(&self) -> u32 {
        4
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![
            [0.0, 0.0, 0.0],
            [1_f64 / 2_f64, 1_f64 / 2_f64, 0_f64],
            [1_f64 / 2_f64, 0_f64, 1_f64 / 2_f64],
            [0_f64, 1_f64 / 2_f64, 1_f64 / 2_f64],
        ]
    }
}

impl SpaceGroupProperties for R {
    type Item = R;

    fn new() -> Self::Item {
        Self
    }

    fn points_per_lattice(&self) -> u32 {
        3
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![
            [0.0, 0.0, 0.0],
            [2_f64 / 3_f64, 1_f64 / 3_f64, 1_f64 / 3_f64],
            [1_f64 / 3_f64, 2_f64 / 3_f64, 2_f64 / 3_f64],
        ]
    }
}

impl SpaceGroupProperties for H {
    type Item = H;

    fn new() -> Self::Item {
        Self
    }

    fn points_per_lattice(&self) -> u32 {
        3
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![
            [0.0, 0.0, 0.0],
            [2_f64 / 3_f64, 1_f64 / 3_f64, 0_f64],
            [1_f64 / 3_f64, 2_f64 / 3_f64, 0_f64],
        ]
    }
}

macro_rules! impl_bravais {
    ($(($x: ty, $output: expr)), * ) => {
        $(impl Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $output)
            }
        }
          impl BravaisLattice for $x {}
        )*
    };
}
impl_bravais!(
    (P, "P"),
    (A, "A"),
    (B, "B"),
    (C, "C"),
    (I, "I"),
    (F, "F"),
    (R, "R"),
    (H, "H")
);
