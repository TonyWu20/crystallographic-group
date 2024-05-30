use std::fmt::Display;

use nalgebra::Vector3;
use winnow::PResult;

use self::parser::parse_lattice_symbol;

mod parser;

pub trait LatticeSymbolChar {
    type Output;
    fn translations() -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
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
    pub fn new(minus_sign: bool, char: Lattices) -> Self {
        Self { minus_sign, char }
    }
    pub fn from_str(input: &mut &str) -> PResult<Self> {
        parse_lattice_symbol(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Lattices {
    P,
    A,
    B,
    C,
    I,
    R,
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

#[derive(Debug, Clone, Copy)]
pub struct P;

#[derive(Debug, Clone, Copy)]
pub struct A;

#[derive(Debug, Clone, Copy)]
pub struct B;

#[derive(Debug, Clone, Copy)]
pub struct C;

#[derive(Debug, Clone, Copy)]
pub struct I;

#[derive(Debug, Clone, Copy)]
pub struct R;

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
