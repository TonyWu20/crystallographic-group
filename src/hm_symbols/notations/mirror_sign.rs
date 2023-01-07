use crate::hm_symbols::Notation;
use std::fmt::Display;

#[derive(Default, Debug)]
pub struct M;

impl_notation!(M);

impl Display for M {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m")
    }
}
