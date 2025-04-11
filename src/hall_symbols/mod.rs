use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use nalgebra::{Matrix3, Vector3};
use winnow::ModalResult;

use crate::{
    database::{SpaceGroupHallSymbol, ORDER_12, ORDER_24, ORDER_48},
    utils::positive_mod_stbn_i32,
};

use self::{
    lattice_symbol::LatticeSymbol,
    matrix_symbol::{MatrixSymbol, NFold, NFoldDiag},
    origin_shift::OriginShift,
    parser::parse_hall_symbol,
};

mod general_positions;
mod lattice_symbol;
mod matrix_symbol;
mod origin_shift;
mod parser;
mod translation_symbol;

pub use general_positions::GeneralPositions;
pub use matrix_symbol::SeitzMatrix;

pub(crate) const SEITZ_TRANSLATE_BASE_NUMBER: i32 = 12;

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
    pub fn try_from_str(input: &str) -> ModalResult<Self> {
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

    fn get_matrice_order(&self) -> Vec<&str> {
        let first_m = self.matrix_symbols.first().unwrap();
        match first_m.nfold_body() {
            NFold::N6 => ORDER_24.to_vec(),
            NFold::N3 => match first_m.nfold_diag() {
                NFoldDiag::Asterisk => ORDER_12.to_vec(),
                _ => ORDER_24.to_vec(),
            },
            _ => ORDER_48.to_vec(),
        }
    }

    fn sort_general_positions(&self, positions: &[SeitzMatrix]) -> Vec<SeitzMatrix> {
        let mut ret_position: Vec<SeitzMatrix> = positions.to_vec();
        let order_to_use = self.get_matrice_order();
        ret_position.sort_by(|a, b| {
            let a_id = order_to_use
                .iter()
                .position(|&s| s == a.jones_faithful_repr_rot())
                .unwrap_or_else(|| panic!("{} fails to match", a.jones_faithful_repr_rot()));
            let b_id = order_to_use
                .iter()
                .position(|&s| s == b.jones_faithful_repr_rot())
                .unwrap_or_else(|| panic!("{} fails to match", b.jones_faithful_repr_rot()));
            a_id.cmp(&b_id)
        });
        ret_position.to_vec()
    }

    /// Find the minimal positive x,y,z for the translation vector,
    /// by considering the lattice translation vectors.
    fn translation_minimal_repr(&self, new_translation: Vector3<i32>) -> Vector3<i32> {
        let new_translation_pos = new_translation.map(positive_mod_stbn_i32);
        self.lattice_symbol
            .get_translations()
            .iter()
            .map(|tr| {
                let v = new_translation + tr;
                v.map(positive_mod_stbn_i32)
            })
            .fold(new_translation_pos, |curr, next| {
                if curr.map(|v| v as f64).norm_squared() > next.map(|v| v as f64).norm_squared()
                    && curr.iter().filter(|&&v| v <= 0).count()
                        >= next.iter().filter(|&&v| v <= 0).count()
                {
                    next
                } else {
                    curr
                }
            })
    }

    /// Add a `SeitzMatrix` to a `Vec<SeitzMatrix>` if it is unique.
    /// Returns true when the matrix is unique and adding is successful, other wise false.
    fn add_to_list(
        &self,
        list: &mut Vec<SeitzMatrix>,
        map: &mut HashMap<Matrix3<i32>, HashSet<Vector3<i32>>>,
        mut new_matrix: SeitzMatrix,
    ) -> bool {
        match map.get_mut(&new_matrix.rotation_part()) {
            None => {
                let mut translation_set = HashSet::new();
                let tr_with_min_pos_component =
                    self.translation_minimal_repr(new_matrix.translation_part());
                translation_set.insert(tr_with_min_pos_component);
                map.insert(new_matrix.rotation_part(), translation_set);
                new_matrix.set_translation_part(tr_with_min_pos_component);
                list.push(new_matrix);
                true
            }
            Some(tr_set) => {
                if self.lattice_symbol.get_translations().iter().all(|tr| {
                    let t = (new_matrix.translation_part() + tr).map(positive_mod_stbn_i32);
                    tr_set.get(&t).is_none()
                        && (new_matrix.translation_part() + tr)
                            .map(|v| v % SEITZ_TRANSLATE_BASE_NUMBER)
                            .iter()
                            .all(|&v| v != 0)
                }) {
                    let tr_with_min_pos_component =
                        self.translation_minimal_repr(new_matrix.translation_part());
                    if tr_set.insert(tr_with_min_pos_component) {
                        new_matrix.set_translation_part(tr_with_min_pos_component);
                        list.push(new_matrix);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    fn generate_positions(&self) -> Vec<SeitzMatrix> {
        // let num_generators = self.num_generators();
        let mut list: Vec<SeitzMatrix> = Vec::with_capacity(self.max_equiv_pos());
        let mut matrice_map: HashMap<Matrix3<i32>, HashSet<Vector3<i32>>> = HashMap::new();
        self.lattice_symbol.seitz_matrices().iter().for_each(|&m| {
            self.add_to_list(&mut list, &mut matrice_map, m);
        });
        self.matrix_symbols.iter().for_each(|ms| {
            let seitz_mx = ms
                .seitz_matrix()
                .unwrap_or_else(|_| panic!("SeitzMatrix generation failed for {}", ms));
            let shifted = self.origin_shift.shifted_matrix(seitz_mx);
            self.add_to_list(&mut list, &mut matrice_map, shifted);
        });
        loop {
            let mut list_cloned = list.clone();
            for i in list.iter().skip(1) {
                for j in list.iter().skip(1) {
                    let new_m = *i * *j;
                    if self.add_to_list(&mut list_cloned, &mut matrice_map, new_m) {
                        break;
                    }
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

impl Display for HallSymbolNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lattice_symbol = format!("{}", self.lattice_symbol);
        let matrice = self
            .matrix_symbols
            .iter()
            .map(|m| format!("{m}"))
            .collect::<Vec<String>>()
            .join(" ");
        let origin_shift = if self.origin_shift != OriginShift::default() {
            format!(" {}", self.origin_shift)
        } else {
            String::new()
        };
        write!(f, "{} {}{}", lattice_symbol, matrice, origin_shift)
    }
}

#[cfg(test)]
mod test {

    use std::{collections::HashSet, fs::read_to_string, path::Path};

    use indicatif::ProgressIterator;

    use crate::database::DEFAULT_SPACE_GROUP_SYMBOLS;

    use super::{
        matrix_symbol::{MatrixSymbol, NFold, NFoldSub},
        translation_symbol::TranslationSymbol,
        HallSymbolNotation,
    };

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
    fn test_150() {
        let symbol_str = "P 3 2\"";
        let p_150 = HallSymbolNotation::try_from_str(symbol_str).unwrap();
        dbg!(p_150.matrix_symbols);
    }
    #[test]
    fn test_228() {
        test("-F 4ud 2vw 3");
    }
    #[test]
    fn test_221() {
        test("-P 4 2 3")
    }
    #[test]
    fn test_229() {
        test("-I 4 2 3")
    }
    #[test]
    fn test_91() {
        test("P 4w 2c")
    }
    #[test]
    fn test_45() {
        test("I 2 -2c")
    }
    #[test]
    fn test_80() {
        let c1 = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N4)
            .set_nfold_sub(NFoldSub::N1)
            .set_translation_symbols(Some(vec![TranslationSymbol::B]))
            .build()
            .unwrap();
        let c2 = MatrixSymbol::new_builder()
            .set_nfold_body(NFold::N4)
            .set_translation_symbols(Some(vec![TranslationSymbol::B, TranslationSymbol::W]))
            .build()
            .unwrap();
        println!(
            "{}, {}",
            c1.seitz_matrix().unwrap(),
            c2.seitz_matrix().unwrap()
        );
        println!("{}", c1.seitz_matrix().unwrap().powi(2));
        println!("{}", c2.seitz_matrix().unwrap().powi(2));
        test("-I 41b")
    }

    #[test]
    fn test_all() {
        let default_list = DEFAULT_SPACE_GROUP_SYMBOLS.get(2).unwrap();
        default_list
            .iter()
            .progress()
            .map(|&symbol| {
                HashSet::<String>::from_iter(
                    HallSymbolNotation::try_from_str(symbol)
                        .unwrap()
                        .general_positions()
                        .pure_txt(),
                )
            })
            .enumerate()
            .for_each(|(i, xyz_repr)| {
                let ref_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("refs")
                    .join(format!("{}.txt", i + 1));
                let ref_content = read_to_string(ref_path)
                    .unwrap()
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<HashSet<String>>();
                if ref_content != xyz_repr {
                    println!("{}: {}", i + 1, default_list[i]);
                    println!("ref:\n{:?}", ref_content);
                    println!("this:\n{:?}", xyz_repr);
                }
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

    fn read_from_refs() -> Vec<Vec<String>> {
        todo!()
    }
}
