use nalgebra::Matrix3;

mod basic_cyclic_groups;

pub struct PointGroup {
    operators: Vec<Matrix3<i8>>,
    symbol: String,
}
