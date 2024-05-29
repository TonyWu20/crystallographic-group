use std::ops::Neg;

use nalgebra::{Matrix4, Vector4};

use super::SEITZ_TRANSLATE_BASE_NUMBER;

#[derive(Debug, Clone, Copy)]
pub struct OriginShift {
    va: i32,
    vb: i32,
    vc: i32,
}

impl OriginShift {
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

    pub fn shifted_matrix(&self, matrix: Matrix4<i32>) -> Matrix4<i32> {
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
        let mut result = mvi * matrix * mv;
        // Output convention is positive
        result.column_mut(3).iter_mut().for_each(|v| {
            if !(0..SEITZ_TRANSLATE_BASE_NUMBER).contains(v) {
                *v = *v % SEITZ_TRANSLATE_BASE_NUMBER + SEITZ_TRANSLATE_BASE_NUMBER
            }
        });
        result
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Matrix4;

    use crate::hall_symbols::SEITZ_TRANSLATE_BASE_NUMBER;

    use super::OriginShift;

    #[test]
    fn origin_shift_test() {
        let shift = OriginShift::new(0, 0, -11);
        let shifted_matrix = shift.shifted_matrix(Matrix4::identity());
        let mat = Matrix4::identity();
        assert_eq!(shifted_matrix, mat);
        println!(
            "{}",
            -13 % SEITZ_TRANSLATE_BASE_NUMBER + SEITZ_TRANSLATE_BASE_NUMBER
        );
    }
}
