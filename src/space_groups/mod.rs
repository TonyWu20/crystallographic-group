use std::{fmt::Display, marker::PhantomData};
mod triclinic;
pub use triclinic::*;

/// Letters representing space groups

/// Primitive
pub struct P;
/// A-face centered
pub struct A;
/// B-face centered
pub struct B;
/// C-face centered
pub struct C;
/// Body centered
pub struct I;
/// Face centered (all)
pub struct F;
/// Rhombohedral
pub struct Rh<T: RhombohedralCenter>(PhantomData<T>);
/// Centered Hexagonal
pub struct H;

pub trait RhombohedralCenter {}
pub struct RbAxis;
pub struct HexAxis;

impl RhombohedralCenter for RbAxis {}
impl RhombohedralCenter for HexAxis {}

// /// Letter-H-M Symbol-Space group id
// /// The space group symbol.
// pub struct SpaceGroup<C, T, U, V, D1, D2, D3, const N: usize>
// where
//     C: SpaceGroupProperties,
//     T: Notation,
//     U: Notation,
//     V: Notation,
//     D1: Primary,
//     D2: Secondary,
//     D3: Tertiary,
// {
//     letter: C,
//     symbol: HMSymbol<T, U, V, D1, D2, D3>,
// }
//
// pub trait Generators {
//     fn generators(&self) -> Vec<Matrix3<i32>>;
// }
//
// impl<C, T, U, V, D1, D2, D3, const N: usize> SpaceGroup<C, T, U, V, D1, D2, D3, N>
// where
//     C: SpaceGroupProperties + SpaceGroupProperties<Item = C>,
//     T: Notation,
//     U: Notation,
//     V: Notation,
//     D1: Primary,
//     D2: Secondary,
//     D3: Tertiary,
// {
//     pub fn new() -> Self {
//         Self {
//             letter: C::new(),
//             symbol: HMSymbol::new(),
//         }
//     }
// }
//
// impl<C, T, U, V, D1, D2, D3, const N: usize> Display for SpaceGroup<C, T, U, V, D1, D2, D3, N>
// where
//     C: SpaceGroupProperties,
//     T: Notation,
//     U: Notation,
//     V: Notation,
//     D1: Primary,
//     D2: Secondary,
//     D3: Tertiary,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}:{}{}", N, self.letter, self.symbol)
//     }
// }

pub trait SpaceGroupProperties: Display {
    type Item;
    fn new() -> Self::Item;
    fn points_per_lattice(&self) -> u32;
    fn lattice_coordinates(&self) -> Vec<[f64; 3]>;
}

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

impl SpaceGroupProperties for Rh<RbAxis> {
    type Item = Rh<RbAxis>;

    fn new() -> Self::Item {
        Self(PhantomData)
    }

    fn points_per_lattice(&self) -> u32 {
        1
    }

    fn lattice_coordinates(&self) -> Vec<[f64; 3]> {
        vec![[0.0, 0.0, 0.0]]
    }
}

impl SpaceGroupProperties for Rh<HexAxis> {
    type Item = Rh<HexAxis>;

    fn new() -> Self::Item {
        Self(PhantomData)
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

macro_rules! impl_display {
    ($(($x: ty, $output: expr)), * ) => {
        $(impl Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $output)
            }
        })*
    };
}
impl_display!(
(P, "P"),
(A, "A"),
(B, "B"),
(C, "C"),
(I, "I"),
(F, "F"),
(Rh<RbAxis>, "R-Primitive"),
(Rh<HexAxis>, "R-Centered"),
(H, "H")
);