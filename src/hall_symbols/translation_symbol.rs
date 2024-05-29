use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub enum TranslationSymbol {
    A,
    B,
    C,
    N,
    U,
    V,
    W,
    D,
}

impl TranslationSymbol {
    pub fn translation_vector(&self) -> Vector3<i32> {
        match self {
            TranslationSymbol::A => Vector3::new(6, 0, 0),
            TranslationSymbol::B => Vector3::new(0, 6, 0),
            TranslationSymbol::C => Vector3::new(0, 0, 6),
            TranslationSymbol::N => Vector3::new(6, 6, 6),
            TranslationSymbol::U => Vector3::new(3, 0, 0),
            TranslationSymbol::V => Vector3::new(0, 3, 0),
            TranslationSymbol::W => Vector3::new(0, 0, 3),
            TranslationSymbol::D => Vector3::new(3, 3, 3),
        }
    }
}
