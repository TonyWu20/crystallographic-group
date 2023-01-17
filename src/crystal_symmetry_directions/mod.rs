use std::marker::PhantomData;

use nalgebra::{Unit, UnitVector3, Vector3};

use crate::{Basis, HexBasis, Standard};

/// Basis trait of an axis
pub trait Axis {}
/// For axis that is not [000]
pub trait RealAxis: Axis + Copy {}
pub trait Primary {}
pub trait Secondary {}
pub trait Tertiary {}

/// For identity and inversion
#[derive(Debug, Clone, Copy)]
pub struct Universal;
impl Axis for Universal {}

/// Axes parallel to or planes perpendicular to a, b, c
#[derive(Debug, Clone, Copy)]
pub struct Principal;
impl Axis for Principal {}
impl RealAxis for Principal {}

#[derive(Debug, Clone, Copy)]
pub struct CAxis;
impl Axis for CAxis {}
impl RealAxis for CAxis {}
impl Primary for CAxis {}

/// [110]
#[derive(Debug, Clone, Copy)]
pub struct FaceDiagonal;
impl Axis for FaceDiagonal {}
impl RealAxis for FaceDiagonal {}

/// [111]
#[derive(Debug, Clone, Copy)]
pub struct BodyDiagonal;
impl Axis for BodyDiagonal {}
impl RealAxis for BodyDiagonal {}

/// Struct to represent a direction under given coordinate system, carries the type of axis info.
#[derive(Debug, Clone, Copy)]
pub struct D<T: Basis, U: Axis, const H: i8, const K: i8, const L: i8> {
    /// Mark the coordinate system.
    basis: PhantomData<T>,
    /// Mark the axis type
    axis_type: PhantomData<U>,
}

/// Common methods
impl<T: Basis, U: Axis, const H: i8, const K: i8, const L: i8> D<T, U, H, K, L> {
    pub fn new() -> Self {
        Self {
            basis: PhantomData,
            axis_type: PhantomData,
        }
    }

    /// Get the [hkl] representation.
    pub fn hkl(&self) -> [i8; 3] {
        [H, K, L]
    }
    pub fn axis(&self) -> UnitVector3<f64> {
        let [x, y, z] = self.hkl();
        Unit::new_normalize(Vector3::new(x as f64, y as f64, z as f64))
    }
}

/// Builder struct to limit the generation results to
/// the designated directions used for crystallographic groups.
pub struct DirectionBuilder<U: Basis>(PhantomData<U>);

/// Common directions available in both cartesian and hexagonal basis.
impl<U: Basis> DirectionBuilder<U> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
    pub fn zero(&self) -> D<U, Universal, 0, 0, 0> {
        D::<U, Universal, 0, 0, 0>::new()
    }
    pub fn a(&self) -> D<U, Principal, 1, 0, 0> {
        D::new()
    }
    pub fn b(&self) -> D<U, Principal, 0, 1, 0> {
        D::new()
    }
    pub fn c(&self) -> D<U, Principal, 0, 0, 1> {
        D::new()
    }
    pub fn ab(&self) -> D<U, FaceDiagonal, 1, 1, 0> {
        D::new()
    }
    /// [1-10]
    pub fn a_b(&self) -> D<U, FaceDiagonal, 1, -1, 0> {
        D::new()
    }
}

/// Special directions available in the standard coodinate system
impl DirectionBuilder<Standard> {
    pub fn abc(&self) -> D<Standard, BodyDiagonal, 1, 1, 1> {
        D::new()
    }
}
