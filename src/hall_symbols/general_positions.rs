use std::fmt::Display;

use fraction::GenericFraction;
use nalgebra::Vector3;

use crate::hall_symbols::SEITZ_TRANSLATE_BASE_NUMBER;

use super::matrix_symbol::SeitzMatrix;

/// The general positions of one space group setting.
///
/// A general position is a symmetry operation of the group. The type
/// holds the core position set and the lattice translations. It can
/// expand either into the full sets or into one formula string per
/// position.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct GeneralPositions {
    lattice_translations: Vec<Vector3<i32>>,
    core_position_set: Vec<SeitzMatrix>,
}

impl GeneralPositions {
    /// Build a position set from the lattice translations and the
    /// core position set.
    pub fn new(
        lattice_translations: Vec<Vector3<i32>>,
        core_position_set: Vec<SeitzMatrix>,
    ) -> Self {
        Self {
            lattice_translations,
            core_position_set,
        }
    }

    /// Expand the core set into one full set per lattice translation.
    pub fn derive_full_sets(&self) -> Vec<Vec<SeitzMatrix>> {
        self.lattice_translations
            .iter()
            .map(|&v| self.core_position_set.iter().map(|&m| m + v).collect())
            .collect()
    }

    /// The core position set, one entry per group element.
    pub fn core_position_set(&self) -> &[SeitzMatrix] {
        &self.core_position_set
    }

    /// The number of positions in the full set.
    pub fn len(&self) -> usize {
        self.core_position_set.len()
    }

    /// Whether the position set holds no positions.
    pub fn is_empty(&self) -> bool {
        self.core_position_set.is_empty()
    }

    /// One formula string per position of the full set.
    ///
    /// The list holds the positions in set order. It is the compact
    /// view that the reference files compare against.
    pub fn formulas(&self) -> Vec<String> {
        self.derive_full_sets()
            .iter()
            .flat_map(|v| v.iter().map(|m| m.formula()).collect::<Vec<String>>())
            .collect::<Vec<String>>()
    }
}

impl Display for GeneralPositions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_sets = self.derive_full_sets();
        let output = full_sets
            .iter()
            .zip(self.lattice_translations.iter())
            .map(|(set, tr)| {
                let trans = tr.map(|v| GenericFraction::<i32>::new(v, SEITZ_TRANSLATE_BASE_NUMBER));
                let trans_heading = format!("[{}, {}, {}] + set", trans.x, trans.y, trans.z);
                let positions = set
                    .iter()
                    .map(|m| format!("{m}"))
                    .collect::<Vec<String>>()
                    .join("\n");
                [trans_heading, positions].join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{output}")
    }
}
