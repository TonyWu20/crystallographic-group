use crate::hm_symbols::Notation;
use std::fmt::Display;

#[derive(Default, Debug)]
pub struct Glide<const N: usize>;
pub type GlideA = Glide<0>;
pub type GlideB = Glide<1>;
pub type GlideC = Glide<2>;
pub type GlideN = Glide<3>;
pub type GlideD = Glide<4>;

impl_notation!(GlideA, GlideB, GlideC, GlideD, GlideN);
impl_hm_display!(
    (GlideA, "a"),
    (GlideB, "b"),
    (GlideC, "c"),
    (GlideD, "d"),
    (GlideN, "n")
);

#[derive(Default, Debug)]
pub struct Screw<const N: usize, const T: usize>;

impl_notation!(Screw<2,1>, Screw<3,1>, Screw<3,2>, Screw<4,1>, Screw<4,2>, Screw<4,3>,
Screw<6,1>,Screw<6,2>,Screw<6,3>,Screw<6,4>,Screw<6,5>
);

impl_hm_display!(
(Screw<2,1>, "2_1"), (Screw<3,1>, "3_1"),(Screw<3,2>,"3_2"),
(Screw<4,1>, "4_1"), (Screw<4,2>,"4_2"), (Screw<4,3>, "4_3"),
(Screw<6,1>, "6_1"), (Screw<6,2>, "6_2"), (Screw<6,3>, "6_3"), (Screw<6,4>, "6_4"), (Screw<6,5>, "6_5")
    );
