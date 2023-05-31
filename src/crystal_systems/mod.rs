use std::fmt::Debug;

pub trait CrystalSystem: Clone {}

pub trait Basis: Debug + Clone {}

#[derive(Debug, Clone, Copy)]
pub struct Standard;
impl Basis for Standard {}

#[derive(Debug, Clone, Copy)]
pub struct HexBasis;
impl Basis for HexBasis {}

// #[derive(Debug, Clone, Copy)]
// pub struct RhombohedralBasis;
// impl Basis for RhombohedralBasis {}

#[derive(Debug, Clone, Copy)]
pub struct Triclinic;
#[derive(Debug, Clone, Copy)]
pub struct Monoclinic;
#[derive(Debug, Clone, Copy)]
pub struct Orthorhombic;
#[derive(Debug, Clone, Copy)]
pub struct Tetragonal;
#[derive(Debug, Clone, Copy)]
pub struct Hexagonal;
#[derive(Debug, Clone, Copy)]
pub struct Trigonal;
#[derive(Debug, Clone, Copy)]
pub struct Cubic;

impl CrystalSystem for Triclinic {}

impl CrystalSystem for Monoclinic {}

impl CrystalSystem for Orthorhombic {}

impl CrystalSystem for Tetragonal {}

impl CrystalSystem for Hexagonal {}

impl CrystalSystem for Trigonal {}

impl CrystalSystem for Cubic {}
