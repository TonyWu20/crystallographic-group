use std::{fmt::Display, ops::Neg};

use nalgebra::{Matrix4, Vector4};
use winnow::ModalResult;

use crate::utils::positive_mod_stbn_i32;

use self::parser::parse_origin_shift;

use super::{matrix_symbol::SeitzMatrix, SEITZ_TRANSLATE_BASE_NUMBER};

mod parser;

pub const CHANGE_OF_BASIS_BASE_NUMBER: i32 = 72;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OriginShift {
    va: i32,
    vb: i32,
    vc: i32,
}

impl Default for OriginShift {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl OriginShift {
    pub fn try_from_str(input: &mut &str) -> ModalResult<Self> {
        parse_origin_shift(input)
    }

    pub fn new(va: i32, vb: i32, vc: i32) -> Self {
        // Ensure value is between (-12, 12)
        let closed = [va, vb, vc].map(|v| {
            if !(-(SEITZ_TRANSLATE_BASE_NUMBER - 1)..SEITZ_TRANSLATE_BASE_NUMBER).contains(&v) {
                v % SEITZ_TRANSLATE_BASE_NUMBER + SEITZ_TRANSLATE_BASE_NUMBER
            } else {
                v
            }
        });
        let [va, vb, vc] = closed;
        Self { va, vb, vc }
    }

    pub fn shifted_matrix(&self, seitz_matrix: SeitzMatrix) -> SeitzMatrix {
        let mut v = [self.va, self.vb, self.vc].map(|v| v.neg()).to_vec();
        v.push(1);
        let vt = Vector4::from_column_slice(&v);
        let mut mv: Matrix4<i32> = Matrix4::identity();
        mv.set_column(3, &vt);
        let mut v_i = [self.va, self.vb, self.vc].to_vec();
        v_i.push(1);
        let v_it = Vector4::from_column_slice(&v_i);
        let mut mvi = Matrix4::identity();
        mvi.set_column(3, &v_it);
        let mut result = mvi * seitz_matrix.matrix() * mv;
        // Output convention is positive
        result.column_mut(3).iter_mut().for_each(|v| {
            *v = positive_mod_stbn_i32(*v);
        });
        SeitzMatrix::new(result)
    }
}

impl Display for OriginShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.va, self.vb, self.vc)
    }
}

#[cfg(test)]
mod test {
    use crate::hall_symbols::{matrix_symbol::SeitzMatrix, SEITZ_TRANSLATE_BASE_NUMBER};

    use super::OriginShift;

    #[test]
    fn origin_shift_test() {
        let shift = OriginShift::new(0, 0, -11);
        let shifted_matrix = shift.shifted_matrix(SeitzMatrix::identity());
        assert_eq!(shifted_matrix, SeitzMatrix::identity());
        println!(
            "{}",
            -13 % SEITZ_TRANSLATE_BASE_NUMBER + SEITZ_TRANSLATE_BASE_NUMBER
        );
    }
}
