//! The public center of the crate: one space group.
//!
//! A [`SpaceGroup`] holds the lattice symbol, the generator matrix
//! symbols, and the origin shift of one space group setting. It
//! resolves its row of the all-settings space group table at
//! construction, so the number, the full Hermann-Mauguin symbol, and
//! the crystal system come from the table without a string round-trip.

use std::fmt;

use crate::database::{CrystalSystem, SpaceGroupNumber, SpaceGroupTable};
use crate::hall_symbols::{
    restore_information_in_matrix_symbols, GeneralPositions, HallParseError, HallSymbolNotation,
    LatticeSymbol, MatrixSymbol, OriginShift,
};

/// One space group setting: a lattice symbol, the generator matrix
/// symbols, and the origin shift.
///
/// Both entry paths resolve the all-settings table row at construction.
/// [`try_from_str`](Self::try_from_str) parses a Hall symbol and fails
/// with [`SpaceGroupError::NotInTable`] when no row matches.
/// [`new`](Self::new) and [`From<HallSymbolNotation>`] take structured
/// parts and store `None` from [`number`](Self::number) when the parts
/// match no row. No entry path panics on a missing row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpaceGroup {
    lattice: LatticeSymbol,
    generators: Vec<MatrixSymbol>,
    origin_shift: OriginShift,
    row: Option<SpaceGroupNumber>,
}

impl SpaceGroup {
    /// Build a group from structured parts.
    ///
    /// The implied-axis rules of
    /// [`restore_information_in_matrix_symbols`](crate::hall_symbols::restore_information_in_matrix_symbols)
    /// apply to `generators`, as the parse path applies them. A part set
    /// that matches no table row gives `None` from the table methods.
    pub fn new(lattice: LatticeSymbol, generators: Vec<MatrixSymbol>, origin_shift: OriginShift) -> Self {
        Self::from(HallSymbolNotation::new(lattice, generators, origin_shift))
    }

    /// Parse a Hall symbol string and build the group.
    ///
    /// A parse failure is [`SpaceGroupError::Parse`]. A symbol that
    /// parses but matches no row of the all-settings table is
    /// [`SpaceGroupError::NotInTable`].
    pub fn try_from_str(input: &str) -> Result<Self, SpaceGroupError> {
        let notation = HallSymbolNotation::try_from_str(input).map_err(SpaceGroupError::Parse)?;
        let group = Self::from(notation);
        group.number().ok_or_else(|| SpaceGroupError::NotInTable(input.to_string()))?;
        Ok(group)
    }

    /// The typed row of the all-settings table for this group.
    ///
    /// Set at construction. `None` when the parts match no table row.
    pub fn number(&self) -> Option<SpaceGroupNumber> {
        self.row
    }

    /// The full Hermann-Mauguin symbol of the group's table row.
    pub fn hm_symbol(&self) -> Option<&'static str> {
        self.row.and_then(|row| SpaceGroupTable::all().hm_full_notation(row.row()))
    }

    /// The crystal system of the group, from the standard range of the
    /// space group number of its table row.
    pub fn crystal_system(&self) -> Option<CrystalSystem> {
        self.row.and_then(|row| {
            let table = SpaceGroupTable::all();
            let number = table.space_group_number(row.row())?;
            CrystalSystem::of_number(number)
        })
    }

    /// The general positions of the group.
    pub fn general_positions(&self) -> GeneralPositions {
        HallSymbolNotation::new(self.lattice, self.generators.clone(), self.origin_shift)
            .general_positions()
    }
}

impl From<HallSymbolNotation> for SpaceGroup {
    fn from(notation: HallSymbolNotation) -> Self {
        let lattice = notation.lattice_symbol();
        let mut generators = notation.matrix_symbols().to_vec();
        let origin_shift = notation.origin_shift();
        restore_information_in_matrix_symbols(&mut generators);
        let row = table_row_of(&HallSymbolNotation::new(lattice, generators.clone(), origin_shift));
        Self {
            lattice,
            generators,
            origin_shift,
            row,
        }
    }
}

/// The all-settings table row whose parsed Hall symbol equals
/// `notation`, in row order. A row that does not parse is skipped.
fn table_row_of(notation: &HallSymbolNotation) -> Option<SpaceGroupNumber> {
    let row = SpaceGroupTable::all().hall_symbols().iter().position(|&symbol| {
        matches!(
            HallSymbolNotation::try_from_str(symbol),
            Ok(parsed) if parsed == *notation
        )
    })?;
    SpaceGroupNumber::new(u16::try_from(row).ok()?)
}

/// Error of [`SpaceGroup::try_from_str`].
#[derive(Debug, Clone)]
pub enum SpaceGroupError {
    /// The input did not parse as a Hall symbol.
    Parse(HallParseError),
    /// The input parsed, but no row of the all-settings table matches it.
    NotInTable(String),
}

impl fmt::Display for SpaceGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(f, "Hall symbol parse failure: {error}"),
            Self::NotInTable(symbol) => {
                write!(f, "Hall symbol {symbol:?} matches no space group table row")
            }
        }
    }
}

impl std::error::Error for SpaceGroupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::NotInTable(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::database::{CrystalSystem, SpaceGroupHallSymbol, SpaceGroupTable};
    use crate::hall_symbols::{Lattices, LatticeSymbol, MatrixSymbol, NFold, OriginShift};

    use super::{SpaceGroup, SpaceGroupError};

    /// One group per crystal system, each through the string entry.
    #[test]
    fn one_group_per_system() {
        let cases: &[(u16, &str, CrystalSystem)] = &[
            (2, "-P 1", CrystalSystem::Triclinic),
            (5, "C 2y", CrystalSystem::Monoclinic),
            (16, "P 2 2", CrystalSystem::Orthorhombic),
            (75, "P 4", CrystalSystem::Tetragonal),
            (143, "P 3", CrystalSystem::Trigonal),
            (168, "P 6", CrystalSystem::Hexagonal),
            (195, "P 2 2 3", CrystalSystem::Cubic),
        ];
        let table = SpaceGroupTable::all();
        for &(number, hall, system) in cases {
            let group = SpaceGroup::try_from_str(hall).unwrap();
            assert_eq!(group.crystal_system(), Some(system), "{hall}");
            let row = group.number().unwrap();
            assert_eq!(table.space_group_number(row.row()), Some(number), "{hall}");
            assert_eq!(table.hall_symbol(row.row()), Some(hall), "{hall}");
            assert_eq!(group.hm_symbol(), table.hm_full_notation(row.row()), "{hall}");
        }
    }

    /// The structured entry path: assemble the parts, no string involved.
    #[test]
    fn structured_construction() {
        let two = || MatrixSymbol::new_builder().nfold_body(NFold::N2).build();
        let group =
            SpaceGroup::new(LatticeSymbol::new(false, Lattices::P), vec![two(), two()], OriginShift::default());
        let parsed = SpaceGroup::try_from_str("P 2 2").unwrap();
        assert_eq!(group.number(), parsed.number());
        assert_eq!(group.crystal_system(), Some(CrystalSystem::Orthorhombic));
        assert_eq!(group.hm_symbol(), Some("P 2 2 2"));
    }

    /// The error cases of the string entry. No panic on a missing row.
    #[test]
    fn string_entry_errors() {
        let parse = SpaceGroup::try_from_str("not a symbol");
        assert!(matches!(parse, Err(SpaceGroupError::Parse(_))));
        // A per-number spelling that is not a row of the all-settings table.
        let missing = SpaceGroup::try_from_str("P 41");
        assert!(matches!(missing, Err(SpaceGroupError::NotInTable(_))));
    }

    /// The system filter on the table and the reference type method.
    #[test]
    fn rows_of_filters_by_system() {
        let table = SpaceGroupTable::per_number();
        assert_eq!(table.rows_of(CrystalSystem::Cubic).len(), 36);
        assert_eq!(table.rows_of(CrystalSystem::Triclinic).len(), 2);
        assert_eq!(table.rows_of(CrystalSystem::Hexagonal).len(), 27);
        assert_eq!(table.rows_of(CrystalSystem::Trigonal).len(), 25);
        assert_eq!(table.rows_of(CrystalSystem::Hexagonal)[0].row(), 167);
        assert_eq!(table.rows_of(CrystalSystem::Cubic)[0].row(), 194);
        let first = SpaceGroupHallSymbol::new(0).unwrap();
        assert_eq!(first.crystal_system(), Some(CrystalSystem::Triclinic));
        assert_eq!(SpaceGroupHallSymbol::new(3).unwrap().crystal_system(), Some(CrystalSystem::Monoclinic));
    }
}
