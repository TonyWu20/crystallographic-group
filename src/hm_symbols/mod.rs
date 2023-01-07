mod directions;
mod notations;
mod space_group;

use std::fmt::Display;

pub use directions::*;
use nalgebra::Matrix3;
pub use notations::*;
pub use space_group::*;

/// General data structure to abstract a space group.
pub struct SpaceGroup<C, T, U, V, D1, D2, D3, const N: usize>
where
    C: SpaceGroupProperties,
    T: Notation,
    U: Notation,
    V: Notation,
    D1: Primary,
    D2: Secondary,
    D3: Tertiary,
{
    letter: C,
    symbol: HMSymbol<T, U, V, D1, D2, D3>,
}

pub trait Generators {
    fn generators(&self) -> Vec<Matrix3<i32>>;
}

impl<C, T, U, V, D1, D2, D3, const N: usize> SpaceGroup<C, T, U, V, D1, D2, D3, N>
where
    C: SpaceGroupProperties + space_group::SpaceGroupProperties<Item = C>,
    T: Notation,
    U: Notation,
    V: Notation,
    D1: Primary,
    D2: Secondary,
    D3: Tertiary,
{
    pub fn new() -> Self {
        Self {
            letter: C::new(),
            symbol: HMSymbol::new(),
        }
    }
}

impl<C, T, U, V, D1, D2, D3, const N: usize> Display for SpaceGroup<C, T, U, V, D1, D2, D3, N>
where
    C: SpaceGroupProperties,
    T: Notation,
    U: Notation,
    V: Notation,
    D1: Primary,
    D2: Secondary,
    D3: Tertiary,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}{}", N, self.letter, self.symbol)
    }
}

// /// The Hermann-Mauguin notation for space group.
pub struct HMSymbol<T, U, V, D1, D2, D3>(HMElement<T, D1>, HMElement<U, D2>, HMElement<V, D3>)
where
    T: Notation,
    U: Notation,
    V: Notation,
    D1: Primary,
    D2: Secondary,
    D3: Tertiary;

impl<T, U, V, D1, D2, D3> HMSymbol<T, U, V, D1, D2, D3>
where
    T: Notation,
    U: Notation,
    V: Notation,
    D1: Primary,
    D2: Secondary,
    D3: Tertiary,
{
    pub fn new() -> Self {
        Self(
            HMElement::<T, D1>::new(),
            HMElement::<U, D2>::new(),
            HMElement::<V, D3>::new(),
        )
    }
}

impl<T, U, V, D1, D2, D3> Display for HMSymbol<T, U, V, D1, D2, D3>
where
    T: Notation,
    U: Notation,
    V: Notation,
    D1: Primary,
    D2: Secondary,
    D3: Tertiary,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod test {
    use crate::hm_symbols::P1;

    #[test]
    fn test_symbol() {
        let p1 = P1::new();
        println!("{}", p1);
    }
}
