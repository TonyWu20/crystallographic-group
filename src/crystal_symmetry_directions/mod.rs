use std::marker::PhantomData;

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
pub struct D<T: Basis, U: Axis> {
    /// [hkl] representation of the direction.
    hkl: [i8; 3],
    /// Mark the coordinate system.
    basis: PhantomData<T>,
    /// Mark the axis type
    axis_type: PhantomData<U>,
}

/// Common methods
impl<T: Basis, U: Axis> D<T, U> {
    pub fn new(hkl: [i8; 3]) -> Self {
        Self {
            hkl,
            basis: PhantomData,
            axis_type: PhantomData,
        }
    }

    /// Get the [hkl] representation.
    pub fn hkl(&self) -> [i8; 3] {
        self.hkl
    }
}

/// Builder struct to limit the generation results to
/// the designated directions used for crystallographic groups.
pub struct DirectionBuilder<U: Basis>(PhantomData<U>);

impl<U: Basis> DirectionBuilder<U> {
    pub fn zero() -> D<U, Universal> {
        D::new([0, 0, 0])
    }
}

/// Directions available in the standard coodinate system
impl DirectionBuilder<Standard> {
    pub fn a() -> D<Standard, Principal> {
        D::new([1, 0, 0])
    }
    pub fn b() -> D<Standard, Principal> {
        D::new([0, 1, 0])
    }
    pub fn c() -> D<Standard, Principal> {
        D::new([0, 0, 1])
    }
    pub fn cubic_diagonal() -> D<Standard, BodyDiagonal> {
        D::new([1, 1, 1])
    }
    pub fn ab() -> D<Standard, FaceDiagonal> {
        D::new([1, 1, 0])
    }
}
/// Directions available in the hex-basis coordinate system.
impl DirectionBuilder<HexBasis> {
    pub fn a() -> D<HexBasis, Principal> {
        D::new([1, 0, 0])
    }
    pub fn b() -> D<HexBasis, Principal> {
        D::new([0, 1, 0])
    }
    pub fn c() -> D<HexBasis, Principal> {
        D::new([0, 0, 1])
    }
    pub fn ab() -> D<HexBasis, FaceDiagonal> {
        D::new([1, 1, 0])
    }
}