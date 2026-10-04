use std::fmt::Display;

use nalgebra::Vector3;

/// A translation symbol of a Hall notation.
///
/// Each symbol adds a fixed half-lattice or quarter-lattice
/// translation. The letters follow the Hall convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TranslationSymbol {
    /// No translation, or an unrecognized letter.
    Invalid,
    /// A half translation along the `a` axis.
    A,
    /// A half translation along the `b` axis.
    B,
    /// A half translation along the `c` axis.
    C,
    /// A half translation along the body diagonal.
    N,
    /// A quarter translation along the `a` axis.
    U,
    /// A quarter translation along the `b` axis.
    V,
    /// A quarter translation along the `c` axis.
    W,
    /// A quarter translation along the body diagonal.
    D,
}

impl TranslationSymbol {
    /// The translation vector of this symbol, in units of the 12-fold
    /// base.
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
            TranslationSymbol::Invalid => Vector3::zeros(),
        }
    }
}

impl From<&char> for TranslationSymbol {
    fn from(value: &char) -> Self {
        match value {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'n' => Self::N,
            'u' => Self::U,
            'v' => Self::V,
            'w' => Self::W,
            'd' => Self::D,
            _ => Self::Invalid,
        }
    }
}

impl Display for TranslationSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationSymbol::Invalid => f.write_str("Invalid"),
            TranslationSymbol::A => f.write_str("a"),
            TranslationSymbol::B => f.write_str("b"),
            TranslationSymbol::C => f.write_str("c"),
            TranslationSymbol::N => f.write_str("n"),
            TranslationSymbol::U => f.write_str("u"),
            TranslationSymbol::V => f.write_str("v"),
            TranslationSymbol::W => f.write_str("w"),
            TranslationSymbol::D => f.write_str("d"),
        }
    }
}
