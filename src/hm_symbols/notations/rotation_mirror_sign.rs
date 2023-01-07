use crate::hm_symbols::Notation;
use std::fmt::Display;

#[derive(Default, Debug)]
pub struct RM<const N: usize>;

impl_notation!(RM<2>, RM<3>, RM<4>, RM<6>);
impl_hm_display!((RM<2>, "2/m"), (RM<3>, "3/m"), (RM<4>, "4/m"), (RM<6>, "6/m"));
