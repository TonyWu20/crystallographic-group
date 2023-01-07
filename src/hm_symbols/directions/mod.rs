/// Define valid directions for crystal systems and space group.

/// Marker trait to mark a direction.
pub trait Direction {}

/// Mark the direction as a valid primary direction
pub trait Primary: Direction {}
/// Mark the direction as a valid secondary direction
pub trait Secondary: Direction {}
/// Mark the direction as a valid tertiary direction
pub trait Tertiary: Direction {}

/// For `triclinic` system.
pub struct None;
impl Direction for None {}
impl Primary for None {}
impl Secondary for None {}
impl Tertiary for None {}

/// [100]
pub struct D100;
pub type X = D100;
impl Direction for D100 {}
impl Primary for D100 {}
impl Secondary for D100 {}
/// [010]
pub struct D010;
pub type Y = D010;
impl Direction for D010 {}
impl Primary for D010 {}
impl Secondary for D010 {}
/// [001]
pub struct D001;
pub type Z = D001;
impl Direction for D001 {}
impl Primary for D001 {}
impl Tertiary for D001 {}
/// [110]
pub struct D110;
impl Direction for D110 {}
impl Tertiary for D110 {}
/// [1-10]
pub struct D1M10;
impl Direction for D1M10 {}
impl Tertiary for D1M10 {}
/// [111]
pub struct D111;
impl Direction for D111 {}
impl Secondary for D111 {}
