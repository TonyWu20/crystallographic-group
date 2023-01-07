use crate::hm_symbols::Notation;
use std::fmt::Display;

#[derive(Default, Debug)]
pub struct R<const N: usize>;
impl_notation!(R<1>, R<2>, R<3>, R<4>, R<6>);
impl_hm_display!((R<1>, "1"), (R<2>, "2"), (R<3>, "3"), (R<4>, "4"), (R<6>, "6"));
