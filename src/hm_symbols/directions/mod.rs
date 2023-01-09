use crate::{
    CrystalSystem, Cubic, Hexagonal, Monoclinic, Orthorhombic, Tetragonal, Triclinic, Trigonal,
};

pub trait Direction {}

pub trait Primary: Direction {}
pub trait Secondary: Direction {}
pub trait Tertiary: Direction {}

pub struct D<const H: i8, const K: i8, const L: i8>;

impl<const H: i8, const K: i8, const L: i8> CrystalSystem for D<H, K, L> {}

macro_rules! impl_for_dir {
    ($t:ty, $($trait: ty),*) => {
        $(impl $trait for $t {})*
    };
}

// This represents "None", for triclinic system.
impl_for_dir!(D<0, 0, 0>, Direction, Primary, Triclinic);
// [010] - The axis parallel or plane perpendicular to `b`, or y-axis
impl_for_dir!(D<0,1,0>, Direction, Primary, Secondary,Monoclinic, Cubic,Orthorhombic,Tetragonal,Hexagonal,Trigonal);
// [001] - The axis parallel or plane perpendicular to `c`, or z-axis
impl_for_dir!(D<0,0,1>, Direction, Primary, Tertiary, Tetragonal, Hexagonal, Trigonal, Cubic,Orthorhombic);
// [100] - The axis parallel or plane perpendicular to `a`, or x-axis
impl_for_dir!(D<1,0,0>, Direction, Primary, Orthorhombic, Cubic, Secondary, Tetragonal,Hexagonal,Trigonal);
// [110] - The axis parallel or plane perpendicular to the line running at 45 degrees to the `a` and `b` axis.
impl_for_dir!(D<1,1,0>, Direction, Tertiary, Tetragonal, Cubic);
impl_for_dir!(D<1,-1,0>, Direction, Tertiary, Hexagonal, Trigonal);
impl_for_dir!(D<1,2,0>, Direction, Tertiary, Hexagonal, Trigonal);
