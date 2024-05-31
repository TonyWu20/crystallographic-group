use winnow::PResult;

use self::{
    general_positions::GeneralPositions,
    lattice_symbol::LatticeSymbol,
    matrix_symbol::{MatrixSymbol, SeitzMatrix},
    origin_shift::OriginShift,
    parser::parse_hall_symbol,
};

mod general_positions;
mod lattice_symbol;
mod matrix_symbol;
mod origin_shift;
mod parser;
mod translation_symbol;

pub const SEITZ_TRANSLATE_BASE_NUMBER: i32 = 12;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    pub fn try_from_str(input: &str) -> PResult<Self> {
        let mut input = input;
        parse_hall_symbol(&mut input)
    }
    fn generate_positions(&self) -> Vec<SeitzMatrix> {
        let mut list: Vec<SeitzMatrix> = Vec::new();
        list.push(SeitzMatrix::identity());
        self.matrix_symbols.iter().for_each(|ms| {
            let seitz_mx = ms
                .seitz_matrix()
                .unwrap_or_else(|_| panic!("SeitzMatrix generation failed for {}", ms));
            let shifted = self.origin_shift.shifted_matrix(seitz_mx);
            list.push(shifted);
        });
        loop {
            let mut new_list: Vec<SeitzMatrix> = Vec::new();
            list.iter().skip(1).for_each(|matrix| {
                list.iter().skip(1).for_each(|to_mul| {
                    let new_matrix = *to_mul * *matrix;
                    if list.iter().all(|&m| m != new_matrix)
                        && new_list.iter().all(|&m| m != new_matrix)
                    {
                        new_list.push(new_matrix)
                    }
                })
            });
            if new_list.is_empty() {
                break;
            } else {
                list.append(&mut new_list);
            }
        }
        list.sort();
        list
    }
    pub fn general_positions(&self) -> GeneralPositions {
        GeneralPositions::new(
            self.lattice_symbol.get_translations(),
            self.generate_positions(),
        )
    }
}

#[cfg(test)]
mod test {

    use super::HallSymbolNotation;

    #[test]
    fn test_p178() {
        let symbol_str = "P 61 2 (0 0 -1)";
        let p178 = HallSymbolNotation::try_from_str(symbol_str);
        let general_positions = p178.unwrap().general_positions();
        println!(
            "Number of positions: {}",
            general_positions.num_of_general_pos()
        );
    }
    #[test]
    fn test_p5() {
        let symbol_str: &str = "C 2y";
        let p_5 = HallSymbolNotation::try_from_str(symbol_str).unwrap();
        let general_positions = p_5.general_positions();
        println!(
            "Number of positions: {}",
            general_positions.num_of_general_pos()
        );
        println!("{general_positions}");
    }
}
