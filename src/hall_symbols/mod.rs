use self::{lattice_symbol::LatticeSymbol, matrix_symbol::MatrixSymbol, origin_shift::OriginShift};

mod lattice_symbol;
mod matrix_symbol;
mod origin_shift;
mod translation_symbol;

pub const SEITZ_TRANSLATE_BASE_NUMBER: i32 = 12;

pub struct HallSymbolNotation {
    lattice_symbol: LatticeSymbol,
    matrix_symbols: Vec<MatrixSymbol>,
    origin_shift: OriginShift,
}

impl HallSymbolNotation {
    pub fn new(
        lattice_symbol: LatticeSymbol,
        matrix_symbols: Vec<MatrixSymbol>,
        origin_shift: OriginShift,
    ) -> Self {
        Self {
            lattice_symbol,
            matrix_symbols,
            origin_shift,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::{
        lattice_symbol::{LatticeSymbol, Lattices},
        matrix_symbol::{MatrixSymbol, NFold, RotationAxis, SeitzMatrix},
        origin_shift::OriginShift,
        HallSymbolNotation,
    };

    impl HallSymbolNotation {
        fn generate_positions(&self) -> Vec<SeitzMatrix> {
            let mut list: Vec<SeitzMatrix> = Vec::new();
            list.push(SeitzMatrix::identity());
            let mut lookup_set: HashSet<SeitzMatrix> = HashSet::new();
            lookup_set.insert(SeitzMatrix::identity());
            self.matrix_symbols.iter().for_each(|ms| {
                let seitz_mx = ms.seitz_matrix().unwrap();
                let shifted = self.origin_shift.shifted_matrix(seitz_mx);
                list.push(shifted);
                lookup_set.insert(shifted);
            });
            loop {
                let mut new_list: Vec<SeitzMatrix> = Vec::new();
                list.iter().skip(1).for_each(|matrix| {
                    list.iter().skip(1).for_each(|to_mul| {
                        let new_matrix = self.origin_shift.shifted_matrix(*to_mul * *matrix);
                        if lookup_set.insert(new_matrix) {
                            new_list.push(new_matrix);
                        }
                    })
                });
                if new_list.is_empty() {
                    break;
                } else {
                    list.append(&mut new_list);
                }
            }
            list
        }
    }
    #[test]
    fn test_generate() {
        let m_p2y = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N2)
            .set_rotation_axis(RotationAxis::Y)
            .build()
            .unwrap();
        let spg_3 = HallSymbolNotation::new(
            LatticeSymbol::new(false, Lattices::P),
            vec![m_p2y],
            OriginShift::new(0, 0, 0),
        );
        let m_2z = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N2)
            .build()
            .unwrap();
        let m_2x = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N2)
            .set_rotation_axis(RotationAxis::X)
            .build()
            .unwrap();
        let general_positions = spg_3.generate_positions();
        general_positions.iter().for_each(|m| println!("{m}"));
        let spg_16 = HallSymbolNotation::new(
            LatticeSymbol::new(false, Lattices::P),
            vec![m_2z, m_2x],
            OriginShift::new(0, 0, 0),
        );
        let general_positions = spg_16.generate_positions();
        println!("P 22");
        general_positions.iter().for_each(|m| println!("{m}"));
    }

    #[test]
    fn test_p178() {
        let mut symbol_str = "P 61 2 (0 0 -1)";
        let symbol = LatticeSymbol::from_str(&mut symbol_str).unwrap();
        println!("{symbol}");
    }
}
