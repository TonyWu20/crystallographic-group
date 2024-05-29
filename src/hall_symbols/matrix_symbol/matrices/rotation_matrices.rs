use nalgebra::{Matrix4, Vector4};

use super::{Rotation, RotationMatrix};

impl<const D: char, const A: char> RotationMatrix for Rotation<1, D, A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        Some(Matrix4::identity())
    }
}

impl<const A: char> RotationMatrix for Rotation<2, '_', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        match A {
            'x' => Some(Matrix4::from_diagonal(&Vector4::new(1, -1, -1, 1))),
            'y' => Some(Matrix4::from_diagonal(&Vector4::new(-1, 1, -1, 1))),
            'z' => Some(Matrix4::from_diagonal(&Vector4::new(-1, -1, 1, 1))),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<3, '_', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [[1, 0, 0, 0], [0, 0, 1, 0], [0, -1, -1, 0], [0, 0, 0, 1]].map(Vector4::from);
        let y = [[-1, 0, -1, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1]].map(Vector4::from);
        let z = [[0, 1, 0, 0], [-1, -1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]].map(Vector4::from);
        match A {
            'x' => Some(Matrix4::from_columns(&x)),
            'y' => Some(Matrix4::from_columns(&y)),
            'z' => Some(Matrix4::from_columns(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<4, '_', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [
            1, 0, 0, 0, //x
            0, 0, 1, 0, //y
            0, -1, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let y = [
            0, 0, -1, 0, //x
            0, 1, 0, 0, //y
            1, 0, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let z = [
            0, 1, 0, 0, //x
            -1, 0, 0, 0, //y
            0, 0, 1, 0, //z
            0, 0, 0, 1, //t
        ];
        match A {
            'x' => Some(Matrix4::from_column_slice(&x)),
            'y' => Some(Matrix4::from_column_slice(&y)),
            'z' => Some(Matrix4::from_column_slice(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<6, '_', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [
            1, 0, 0, 0, //x
            0, 1, 1, 0, //y
            0, -1, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let y = [
            0, 0, -1, 0, //x
            0, 1, 0, 0, //y
            1, 0, 1, 0, //z
            0, 0, 0, 1, //t
        ];
        let z = [
            1, 1, 0, 0, //x
            -1, 0, 0, 0, //y
            0, 0, 1, 0, //z
            0, 0, 0, 1, //t
        ];
        match A {
            'x' => Some(Matrix4::from_column_slice(&x)),
            'y' => Some(Matrix4::from_column_slice(&y)),
            'z' => Some(Matrix4::from_column_slice(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<2, '\'', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [
            -1, 0, 0, 0, //x
            0, 0, -1, 0, //y
            0, -1, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let y = [
            0, 0, -1, 0, //x
            0, -1, 0, 0, //y
            -1, 0, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let z = [
            0, -1, 0, 0, //x
            -1, 0, 0, 0, //y
            0, 0, -1, 0, //z
            0, 0, 0, 1, //t
        ];
        match A {
            'x' => Some(Matrix4::from_column_slice(&x)),
            'y' => Some(Matrix4::from_column_slice(&y)),
            'z' => Some(Matrix4::from_column_slice(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<2, '"', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        let x = [
            -1, 0, 0, 0, //x
            0, 0, 1, 0, //y
            0, 1, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let y = [
            0, 0, 1, 0, //x
            0, -1, 0, 0, //y
            1, 0, 0, 0, //z
            0, 0, 0, 1, //t
        ];
        let z = [
            0, 1, 0, 0, //x
            1, 0, 0, 0, //y
            0, 0, -1, 0, //z
            0, 0, 0, 1, //t
        ];
        match A {
            'x' => Some(Matrix4::from_column_slice(&x)),
            'y' => Some(Matrix4::from_column_slice(&y)),
            'z' => Some(Matrix4::from_column_slice(&z)),
            _ => None,
        }
    }
}

impl<const A: char> RotationMatrix for Rotation<3, '*', A> {
    fn rotation_matrix() -> Option<Matrix4<i32>> {
        Some(Matrix4::from_column_slice(&[
            0, 1, 0, 0, // x
            0, 0, 1, 0, // y
            1, 0, 0, 0, // z
            0, 0, 0, 1, // t
        ]))
    }
}

#[cfg(test)]
mod test {
    use crate::hall_symbols::{
        matrix_symbol::matrices::{Rotation, RotationMatrix},
        origin_shift::OriginShift,
    };

    #[test]
    fn rotation_matrix_gen() {
        let mut r61 = Rotation::<6, '_', 'z'>::rotation_matrix().unwrap();
        let r2a_b = Rotation::<2, '\'', 'z'>::rotation_matrix().unwrap();
        r61.column_mut(3).z = 2;
        println!("{r61:#}");
        let shift = OriginShift::new(0, 0, -1);
        println!("{}", shift.shifted_matrix(r61));
        println!("{}", shift.shifted_matrix(r2a_b));
    }
}
