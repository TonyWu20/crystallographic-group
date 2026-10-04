//! Crystallographic space group reference and computation.
//!
//! The crate parses Hall symbol notations and assembles space groups
//! from typed parts. It resolves each group to its row of the
//! all-settings space group table and computes the general positions.
//!
//! Two entry paths lead to a [`SpaceGroup`]:
//!
//! - Parse a Hall symbol string with
//!   [`SpaceGroup::try_from_str`].
//! - Assemble the typed parts with
//!   [`SpaceGroup::new`].
//!
//! Both paths resolve the table row at construction. The group then
//! exposes the number, the Hermann-Mauguin symbol, the crystal system,
//! and the general positions.
//!
//! The translation part of every Seitz matrix stores integer residues
//! in a fixed base. That base is the value `12`, held by the constant
//! `SEITZ_TRANSLATE_BASE_NUMBER`. The number matches the maximum order
//! of a rotation in a space group. Equality and hashing treat the
//! translation part modulo this base.

pub mod database;
pub mod hall_symbols;
pub mod space_group;

mod utils;

pub use hall_symbols::{GeneralPositions, HallSymbolNotation, SeitzMatrix};
pub use space_group::{SpaceGroup, SpaceGroupError};
