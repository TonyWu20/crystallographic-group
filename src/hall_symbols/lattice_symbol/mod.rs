use std::fmt::Display;

use nalgebra::Vector3;

use crate::hall_symbols::errors::HallParseError;

use self::parser::parse_lattice_symbol;

use super::{matrix_symbol::SeitzMatrix, SymmetryElement};

mod parser;

pub(crate) use parser::lattice_symbol;

/// A marker for one lattice type.
///
/// Each marker names the lattice translation vectors. The lattice
/// types are the unit structs [`P`], [`A`], [`B`], [`C`], [`I`],
/// [`R`], and [`F`].
pub trait LatticeSymbolChar {
    /// The type of the translation vectors for this lattice.
    type Output;

    /// The lattice translation vectors, in units of the 12-fold base.
    fn translations() -> Self::Output;
}

/// The lattice part of a Hall symbol: the sign and the lattice letter.
///
/// The sign marks the centrosymmetric settings. The letter selects the
/// lattice type and its translation vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LatticeSymbol {
    minus_sign: bool,
    char: Lattices,
}

impl Display for LatticeSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.minus_sign { "-" } else { "" };
        write!(f, "{}{:?}", sign, self.char)
    }
}

impl LatticeSymbol {
    /// Build a lattice symbol from the sign and the lattice type.
    pub fn new(minus_sign: bool, char: Lattices) -> Self {
        Self { minus_sign, char }
    }

    /// Parse a lattice symbol string, e.g. `"-F"`.
    pub fn try_from_str(input: &str) -> Result<Self, HallParseError> {
        parse_lattice_symbol(input).map_err(|errors| HallParseError::new(input, errors))
    }

    /// The lattice translation vectors, in units of the 12-fold base.
    pub fn get_translations(&self) -> Vec<Vector3<i32>> {
        self.char.get_translations()
    }

    /// The number of lattice translations of this lattice type.
    pub fn num_of_translations(&self) -> usize {
        match self.char {
            Lattices::P => 1,
            Lattices::A => 2,
            Lattices::B => 2,
            Lattices::C => 2,
            Lattices::I => 2,
            Lattices::R => 3,
            Lattices::F => 4,
        }
    }

    /// Whether the symbol carries the leading minus sign.
    pub fn minus_sign(&self) -> bool {
        self.minus_sign
    }

    /// The Seitz matrices of the lattice, one per translation vector.
    ///
    /// A centrosymmetric lattice doubles the set with the inversion.
    pub fn seitz_matrices(&self) -> Vec<SeitzMatrix> {
        if self.minus_sign {
            // vec![SeitzMatrix::identity(), SeitzMatrix::inversion()]
            self.get_translations()
                .iter()
                .map(|&v| [SeitzMatrix::identity() + v, SeitzMatrix::inversion() + v])
                .collect::<Vec<[SeitzMatrix; 2]>>()
                .concat()
        } else {
            self.get_translations()
                .iter()
                .map(|&v| SeitzMatrix::identity() + v)
                .collect()
            // vec![SeitzMatrix::identity()]
        }
    }

    /// The lattice type of this symbol.
    pub fn char(&self) -> Lattices {
        self.char
    }
}

impl SymmetryElement for LatticeSymbol {
    fn equiv_num(&self) -> usize {
        match self.minus_sign {
            true => self.num_of_translations() * 2,
            false => self.num_of_translations(),
        }
    }
}

/// The Bravais lattice type of a crystal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lattices {
    /// Primitive lattice.
    P,
    /// Base-centered on the `a` axis.
    A,
    /// Base-centered on the `b` axis.
    B,
    /// Base-centered on the `c` axis.
    C,
    /// Body-centered lattice.
    I,
    /// Rhombohedral lattice.
    R,
    /// Face-centered lattice.
    F,
}

impl Lattices {
    fn get_translations(&self) -> Vec<Vector3<i32>> {
        match self {
            Lattices::P => P::translations().to_vec(),
            Lattices::A => A::translations().to_vec(),
            Lattices::B => B::translations().to_vec(),
            Lattices::C => C::translations().to_vec(),
            Lattices::I => I::translations().to_vec(),
            Lattices::R => R::translations().to_vec(),
            Lattices::F => F::translations().to_vec(),
        }
    }
}

/// The primitive lattice marker.
#[derive(Debug, Clone, Copy)]
pub struct P;

/// The `a`-base-centered lattice marker.
#[derive(Debug, Clone, Copy)]
pub struct A;

/// The `b`-base-centered lattice marker.
#[derive(Debug, Clone, Copy)]
pub struct B;

/// The `c`-base-centered lattice marker.
#[derive(Debug, Clone, Copy)]
pub struct C;

/// The body-centered lattice marker.
#[derive(Debug, Clone, Copy)]
pub struct I;

/// The rhombohedral lattice marker.
#[derive(Debug, Clone, Copy)]
pub struct R;

/// The face-centered lattice marker.
#[derive(Debug, Clone, Copy)]
pub struct F;

impl LatticeSymbolChar for P {
    type Output = [Vector3<i32>; 1];

    fn translations() -> Self::Output {
        [Vector3::new(0, 0, 0)]
    }
}

impl LatticeSymbolChar for A {
    type Output = [Vector3<i32>; 2];

    fn translations() -> Self::Output {
        [[0, 0, 0], [0, 6, 6]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for B {
    type Output = [Vector3<i32>; 2];

    fn translations() -> Self::Output {
        [[0, 0, 0], [6, 0, 6]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for C {
    type Output = [Vector3<i32>; 2];

    fn translations() -> Self::Output {
        [[0, 0, 0], [6, 6, 0]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for I {
    type Output = [Vector3<i32>; 2];

    fn translations() -> Self::Output {
        [[0, 0, 0], [6, 6, 6]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for R {
    type Output = [Vector3<i32>; 3];
    fn translations() -> Self::Output {
        [[0, 0, 0], [8, 4, 4], [4, 8, 8]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for F {
    type Output = [Vector3<i32>; 4];
    fn translations() -> Self::Output {
        [[0, 0, 0], [0, 6, 6], [6, 0, 6], [6, 6, 0]].map(Vector3::from)
    }
}
