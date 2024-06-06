use std::fmt::Display;

use fraction::GenericFraction;
use nalgebra::Vector3;

use crate::hall_symbols::SEITZ_TRANSLATE_BASE_NUMBER;

use super::matrix_symbol::SeitzMatrix;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct GeneralPositions {
    lattice_translations: Vec<Vector3<i32>>,
    core_position_set: Vec<SeitzMatrix>,
}

impl GeneralPositions {
    pub fn new(
        lattice_translations: Vec<Vector3<i32>>,
        core_position_set: Vec<SeitzMatrix>,
    ) -> Self {
        Self {
            lattice_translations,
            core_position_set,
        }
    }
    pub fn derive_full_sets(&self) -> Vec<Vec<SeitzMatrix>> {
        self.lattice_translations
            .iter()
            .map(|&v| self.core_position_set.iter().map(|&m| m + v).collect())
            .collect()
    }

    pub fn core_position_set(&self) -> &[SeitzMatrix] {
        &self.core_position_set
    }

    pub fn num_of_general_pos(&self) -> usize {
        self.core_position_set.len()
    }

    pub fn text_format(&self) -> String {
        self.derive_full_sets()
            .iter()
            .zip(self.lattice_translations.iter())
            .map(|(set, tr)| {
                let trans = tr.map(|v| GenericFraction::<i32>::new(v, SEITZ_TRANSLATE_BASE_NUMBER));
                let trans_heading = format!("[{}, {}, {}] + set", trans.x, trans.y, trans.z);
                let positions = set
                    .iter()
                    .enumerate()
                    .map(|(i, m)| format!("{}, {}", i + 1, m.jones_faithful_repr()))
                    .collect::<Vec<String>>()
                    .join("\n");
                [trans_heading, positions].join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n")
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
