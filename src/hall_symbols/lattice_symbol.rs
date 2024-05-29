use nalgebra::Vector3;

pub trait LatticeSymbolChar {
    type Output;
    fn translations(&self) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
pub struct LatticeSymbol {
    minus_sign: bool,
    char: Lattices,
}

#[derive(Debug, Clone, Copy)]
pub enum Lattices {
    P(P),
    A(A),
    B(B),
    C(C),
    I(I),
    R(R),
    F(F),
}

impl Lattices {
    fn get_translations(&self) -> Vec<Vector3<f32>> {
        match &self {
            Lattices::P(symbol) => symbol.translations().to_vec(),
            Lattices::A(symbol) => symbol.translations().to_vec(),
            Lattices::B(symbol) => symbol.translations().to_vec(),
            Lattices::C(symbol) => symbol.translations().to_vec(),
            Lattices::I(symbol) => symbol.translations().to_vec(),
            Lattices::R(symbol) => symbol.translations().to_vec(),
            Lattices::F(symbol) => symbol.translations().to_vec(),
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
    type Output = [Vector3<f32>; 1];

    fn translations(&self) -> Self::Output {
        [Vector3::new(0.0, 0.0, 0.0)]
    }
}

impl LatticeSymbolChar for A {
    type Output = [Vector3<f32>; 2];

    fn translations(&self) -> Self::Output {
        [[0.0, 0.0, 0.0], [0.0, 0.5, 0.5]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for B {
    type Output = [Vector3<f32>; 2];

    fn translations(&self) -> Self::Output {
        [[0.0, 0.0, 0.0], [0.5, 0.0, 0.5]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for C {
    type Output = [Vector3<f32>; 2];

    fn translations(&self) -> Self::Output {
        [[0.0, 0.0, 0.0], [0.5, 0.5, 0.0]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for I {
    type Output = [Vector3<f32>; 2];

    fn translations(&self) -> Self::Output {
        [[0.0, 0.0, 0.0], [0.5, 0.5, 0.5]].map(Vector3::from)
    }
}

impl LatticeSymbolChar for R {
    type Output = [Vector3<f32>; 3];
    fn translations(&self) -> Self::Output {
        [
            [0.0, 0.0, 0.0],
            [2.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0],
            [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0],
        ]
        .map(Vector3::from)
    }
}

impl LatticeSymbolChar for F {
    type Output = [Vector3<f32>; 4];
    fn translations(&self) -> Self::Output {
        [
            [0.0, 0.0, 0.0],
            [0.0, 0.5, 0.5],
            [0.5, 0.0, 0.5],
            [0.5, 0.5, 0.0],
        ]
        .map(Vector3::from)
    }
}
