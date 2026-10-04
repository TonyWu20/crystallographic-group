use std::fmt::Display;

/// The crystal system of a space group.
///
/// The system is fixed by the range of the space group number. It is
/// not the lattice letter. See
/// [`CrystalSystem::of_number`] for the standard ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum CrystalSystem {
    /// Space group numbers 1 to 2.
    #[default]
    Triclinic,
    /// Space group numbers 3 to 15.
    Monoclinic,
    /// Space group numbers 16 to 74.
    Orthorhombic,
    /// Space group numbers 75 to 142.
    Tetragonal,
    /// Space group numbers 143 to 167.
    Trigonal,
    /// Space group numbers 168 to 194.
    Hexagonal,
    /// Space group numbers 195 to 230.
    Cubic,
}

impl CrystalSystem {
    /// The crystal system of a space group number in 1..=230.
    ///
    /// The mapping is the standard International Tables range split:
    ///
    /// - 1..=2: triclinic
    /// - 3..=15: monoclinic
    /// - 16..=74: orthorhombic
    /// - 75..=142: tetragonal
    /// - 143..=167: trigonal
    /// - 168..=194: hexagonal
    /// - 195..=230: cubic
    ///
    /// The system is fixed by the number range, not by the lattice letter.
    /// The trigonal numbers 146, 148, 155, 160, 161, 166, and 167 carry
    /// rhombohedral (`R`) lattice settings. `None` for a value outside
    /// 1..=230.
    pub fn of_number(number: u16) -> Option<Self> {
        match number {
            1..=2 => Some(Self::Triclinic),
            3..=15 => Some(Self::Monoclinic),
            16..=74 => Some(Self::Orthorhombic),
            75..=142 => Some(Self::Tetragonal),
            143..=167 => Some(Self::Trigonal),
            168..=194 => Some(Self::Hexagonal),
            195..=230 => Some(Self::Cubic),
            _ => None,
        }
    }
}

impl Display for CrystalSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod test {
    use crate::database::{SpaceGroupTable, CrystalSystem};

    /// The range boundaries of the standard system split.
    #[test]
    fn system_range_boundaries() {
        let cases: &[(u16, Option<CrystalSystem>)] = &[
            (0, None),
            (1, Some(CrystalSystem::Triclinic)),
            (2, Some(CrystalSystem::Triclinic)),
            (3, Some(CrystalSystem::Monoclinic)),
            (15, Some(CrystalSystem::Monoclinic)),
            (16, Some(CrystalSystem::Orthorhombic)),
            (74, Some(CrystalSystem::Orthorhombic)),
            (75, Some(CrystalSystem::Tetragonal)),
            (142, Some(CrystalSystem::Tetragonal)),
            (143, Some(CrystalSystem::Trigonal)),
            (167, Some(CrystalSystem::Trigonal)),
            (168, Some(CrystalSystem::Hexagonal)),
            (194, Some(CrystalSystem::Hexagonal)),
            (195, Some(CrystalSystem::Cubic)),
            (230, Some(CrystalSystem::Cubic)),
            (231, None),
        ];
        for &(number, expected) in cases {
            assert_eq!(CrystalSystem::of_number(number), expected, "number {number}");
        }
    }

    /// The lattice letter of every per-number table row belongs to its
    /// system's allowed lattice set, so the ranges match the table.
    #[test]
    fn ranges_match_table_lattices() {
        let table = SpaceGroupTable::per_number();
        for row in 0..table.len() as u16 {
            let number = table.space_group_number(row).unwrap();
            let system = CrystalSystem::of_number(number).unwrap();
            let lattice = table.hm_full_notation(row).unwrap().chars().next().unwrap();
            let in_system = match system {
                CrystalSystem::Triclinic => lattice == 'P',
                CrystalSystem::Monoclinic => matches!(lattice, 'P' | 'A' | 'B' | 'C' | 'I'),
                CrystalSystem::Orthorhombic => matches!(lattice, 'P' | 'A' | 'B' | 'C' | 'I' | 'F'),
                CrystalSystem::Tetragonal => matches!(lattice, 'P' | 'I'),
                CrystalSystem::Trigonal => matches!(lattice, 'P' | 'R'),
                CrystalSystem::Hexagonal => lattice == 'P',
                CrystalSystem::Cubic => matches!(lattice, 'P' | 'F' | 'I'),
            };
            assert!(in_system, "number {number} lattice {lattice}");
        }
    }
}
