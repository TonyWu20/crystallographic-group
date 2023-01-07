use crate::hm_symbols::Direction;
use std::{fmt::Display, marker::PhantomData};

/// Use a unit struct with type generic
/// to implement the element representation
/// in Hermann-Mauguin Notation.
pub use mirror_sign::M;
use nalgebra::Matrix3;
pub use rotation_mirror_sign::RM;
pub use rotation_sign::R;
pub use rotatory_inversion_sign::RI;
pub use screw_glide_sign::{GlideA, GlideB, GlideC, GlideD, GlideN, Screw};

#[derive(Debug, Default)]
pub struct Nil;

impl Notation for Nil {}
impl Display for Nil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// Marker trait to define a HM notation.
pub trait Notation: Display + Default {}

/// Defined symmetry operation matrices.
/// *Important!* All the work in the type restrictions
/// are for this implementation.
/// Required the `Notation` trait has been implemented.
pub trait SymmetryOps: Notation {
    fn operators(&self) -> Vec<Matrix3<i32>>;
}

/// The unit struct with generics as type notations of the HM
/// element.
/// # Generic types:
/// - `T`: `Notation` - Notation of symmetry elements.
/// - `A`: `Axis` - Notation of the direction/axis of the symmetry element.
#[derive(Debug)]
pub struct HMElement<T: Notation, D: Direction>(T, PhantomData<D>);

impl<T: Notation, D: Direction> Notation for HMElement<T, D> {}
impl<T: Notation, D: Direction> Default for HMElement<T, D> {
    fn default() -> Self {
        Self(T::default(), PhantomData)
    }
}

impl<T: Notation, D: Direction> HMElement<T, D> {
    /// I think this way is the most ergonomic way so far.
    /// Forced Type declaration, and this is the one and only thing to
    /// do when init a new element.
    pub fn new() -> Self {
        Self(T::default(), PhantomData)
    }
}

/// For convenience
macro_rules! impl_notation {
    ($($x: ty), *) => {
        $(impl Notation for $x{})*
        $(impl $x {
            pub fn new() -> Self {
                Self
            }
        })*
    };
}

/// For convenience
macro_rules! impl_hm_display {
    ($(($x: ty, $output: expr)), * ) => {
        $(impl Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $output)
            }
        })*
    };
}

impl<T: Notation, D: Direction> Display for HMElement<T, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
mod mirror_sign;
mod rotation_mirror_sign;
mod rotation_sign;
mod rotatory_inversion_sign;
mod screw_glide_sign;

#[cfg(test)]
mod test {
    use crate::hm_symbols::{M, R, RI, RM, X, Y, Z};

    use super::HMElement;

    #[test]
    fn element() {
        let r3: HMElement<R<3>, X> = HMElement::new();
        let r2: HMElement<R<2>, Y> = HMElement::new();
        let ri3 = HMElement::<RI<3>, X>::new();
        let r2m: HMElement<RM<2>, X> = HMElement::new();
        let r6m = HMElement::<RM<6>, Y>::new();
        let m = HMElement::<M, Z>::new();
        println!("{}, {}", r3, r2);
        println!("{}", ri3);
        println!("{}, {}", r2m, r6m);
        println!("{}{}{}", r2m, m, m);
        // let m = HMElement::Mirror;
        // let r3i = HMElement::RI(TripleI);
        // println!("{}, {}, {}, {}", r3, r2m, m, r3i);
    }
}
