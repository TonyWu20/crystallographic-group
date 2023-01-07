use crate::hm_symbols::Notation;
use std::fmt::Display;

#[derive(Default, Debug)]
pub struct RI<const N: usize>;
impl_notation!(RI<1>, RI<3>, RI<4>, RI<6>);
impl_hm_display!((RI<1>, "-1"), (RI<3>, "-3"), (RI<4>, "-4"), (RI<6>, "-6"));
