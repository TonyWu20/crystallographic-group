//! The space group reference database.
//!
//! This module holds the two static symbol tables and the typed row
//! index. The all-settings table holds all 530 space group settings.
//! The per-number table holds one default setting per space group
//! number. Lookups are bounds-checked and allocation-free.

mod crystal_system;
mod space_group_table;
mod sym_ops_order;

pub use crystal_system::CrystalSystem;
pub use space_group_table::{
    ALL_SPACE_GROUP_SYMBOLS, PER_NUMBER_SPACE_GROUP_SYMBOLS, SpaceGroupNumber, SpaceGroupTable,
};
pub(crate) use sym_ops_order::{ORDER_12, ORDER_24, ORDER_48};

/// A typed reference to one space group setting: one row of
/// [`SpaceGroupTable::all`].
///
/// This is a thin wrapper over the typed row index [`SpaceGroupNumber`].
/// It replaces the former 530-variant enum, whose hand-maintained variant
/// order was a parallel index into the reference tables: insert one
/// variant in the wrong place and every index was wrong. The typed index
/// is the row position itself, so there is no variant order to keep in
/// sync with the tables.
///
/// Construction is range-checked: only rows 0..=529 of the all-settings
/// table exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpaceGroupHallSymbol(pub SpaceGroupNumber);

impl From<SpaceGroupNumber> for SpaceGroupHallSymbol {
    fn from(number: SpaceGroupNumber) -> Self {
        Self(number)
    }
}

impl SpaceGroupHallSymbol {
    /// Build a reference from a zero-based row of [`SpaceGroupTable::all`].
    ///
    /// `None` when `row` is not a row of the 530-row all-settings table.
    pub fn new(row: u16) -> Option<Self> {
        SpaceGroupNumber::new(row).map(Self::from)
    }

    /// The typed row index.
    pub fn number(self) -> SpaceGroupNumber {
        self.0
    }

    /// The Hall symbol of the setting, e.g. `"P 2y"`.
    ///
    /// The index is range-checked at construction, so this lookup is
    /// infallible and allocation-free.
    pub fn get_hall_symbol(&self) -> &'static str {
        ALL_SPACE_GROUP_SYMBOLS[2][self.0.row() as usize]
    }

    /// The full Hermann-Mauguin symbol of the setting, e.g. `"P 1 2 1"`.
    pub fn get_hm_symbol(&self) -> &'static str {
        ALL_SPACE_GROUP_SYMBOLS[1][self.0.row() as usize]
    }

    /// The space group number with choice marker of the setting, e.g.
    /// `"3:b"`.
    pub fn get_space_group_number_code(&self) -> &'static str {
        ALL_SPACE_GROUP_SYMBOLS[0][self.0.row() as usize]
    }

    /// The crystal system of this setting, from the standard range of the
    /// space group number.
    pub fn crystal_system(self) -> Option<CrystalSystem> {
        let table = SpaceGroupTable::all();
        let number = table.space_group_number(self.0.row())?;
        CrystalSystem::of_number(number)
    }
}
