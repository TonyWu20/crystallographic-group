use crate::hm_symbols::{Nil, None, SpaceGroup, R, RM};

use super::P;

pub type P1 = SpaceGroup<P, Nil, R<1>, Nil, None, None, None, 1>;
pub type Pm1 = SpaceGroup<P, Nil, RM<1>, Nil, None, None, None, 2>;
