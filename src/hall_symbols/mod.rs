use winnow::PResult;

use crate::{
    database::SpaceGroupHallSymbol,
    hall_symbols::matrix_symbol::{ORDER_12, ORDER_48},
};

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

pub trait SymmetryElement {
    fn equiv_num(&self) -> usize;
}

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

    fn num_generators(&self) -> usize {
        self.lattice_symbol.equiv_num() + self.matrix_symbols.len()
    }

    fn max_equiv_pos(&self) -> usize {
        self.matrix_symbols
            .iter()
            .map(|m| m.seitz_matrix().expect("Invalid Seitz Matrix").equiv_num())
            .fold(self.lattice_symbol.equiv_num(), |acc, x| acc * x)
    }

    fn sort_general_positions(&self, positions: &[SeitzMatrix]) -> Vec<SeitzMatrix> {
        let order_to_use = match self.lattice_symbol.char() {
            lattice_symbol::Lattices::R => ORDER_12.to_vec(),
            _ => ORDER_48.to_vec(),
        };
        let mut ret_position: Vec<SeitzMatrix> = positions.to_vec();
        ret_position.sort_by(|a, b| {
            let a_id = order_to_use
                .iter()
                .position(|&s| s == a.jones_faithful_repr_rot())
                .expect("Jones faithful representation does not match.");
            let b_id = order_to_use
                .iter()
                .position(|&s| s == b.jones_faithful_repr_rot())
                .expect("Jones faithful representation does not match.");
            a_id.cmp(&b_id)
        });
        ret_position.to_vec()
    }

    fn generate_positions(&self) -> Vec<SeitzMatrix> {
        let max_equiv_pos = self.max_equiv_pos();
        dbg!(max_equiv_pos);
        // let num_generators = self.num_generators();
        let mut list: Vec<SeitzMatrix> = Vec::with_capacity(self.max_equiv_pos());
        list.append(&mut self.lattice_symbol.seitz_matrices());
        self.matrix_symbols.iter().for_each(|ms| {
            let seitz_mx = ms
                .seitz_matrix()
                .unwrap_or_else(|_| panic!("SeitzMatrix generation failed for {}", ms));
            let shifted = self.origin_shift.shifted_matrix(seitz_mx);
            list.push(shifted);
        });
        loop {
            let mut list_cloned = list.clone();
            for i in list.iter().skip(1) {
                let mut new_matrix = SeitzMatrix::identity();
                for j in list.iter().skip(1) {
                    let new_m = *i * *j;
                    if list_cloned.iter().all(|m| new_m != *m) {
                        new_matrix = new_m;
                        break;
                    }
                }
                if new_matrix != SeitzMatrix::identity() {
                    list_cloned.push(new_matrix);
                    break;
                }
            }
            if list_cloned.len() > list.len() {
                list = list_cloned;
            } else {
                break;
            }
        }
        self.sort_general_positions(&list)
    }
    pub fn general_positions(&self) -> GeneralPositions {
        GeneralPositions::new(
            self.lattice_symbol.get_translations(),
            self.generate_positions(),
        )
    }
}

impl From<SpaceGroupHallSymbol> for HallSymbolNotation {
    fn from(value: SpaceGroupHallSymbol) -> Self {
        Self::try_from_str(&value.get_hall_symbol()).unwrap()
    }
}

#[cfg(test)]
mod test {

    use std::{
        fs::{self, create_dir},
        path::Path,
    };

    use crate::database::DEFAULT_SPACE_GROUP_SYMBOLS;

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
    #[test]
    fn test_228() {
        test("-F 4cvw 2vw 3");
    }
    #[test]
    fn test_221() {
        test("-P 4 2 3")
    }
    #[test]
    fn test_91() {
        test("P 4w 2c")
    }

    #[test]
    fn test_all() {
        let default_list = DEFAULT_SPACE_GROUP_SYMBOLS.get(2).unwrap();
        default_list
            .iter()
            .map(|&symbol| (symbol, xyz_repr(symbol)))
            .for_each(|(symbol, xyz)| {
                let test_dir = Path::new("tests");
                if !test_dir.exists() {
                    create_dir(test_dir).expect("Failed to create dir");
                }
                let filename = test_dir.join(symbol).with_extension("txt");
                fs::write(filename, xyz).expect("Writing out  general positions error!")
            })
    }

    fn test(symbol_str: &str) {
        let g = HallSymbolNotation::try_from_str(symbol_str).unwrap();
        let positions = g.general_positions();
        println!("Number of positions: {}", positions.num_of_general_pos());
        println!("{}", positions.text_format());
    }

    fn xyz_repr(symbol_str: &str) -> String {
        let g = HallSymbolNotation::try_from_str(symbol_str).unwrap();
        let positions = g.general_positions();
        positions.text_format()
    }
}
