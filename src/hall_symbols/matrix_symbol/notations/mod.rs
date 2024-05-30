use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub enum NFold {
    #[default]
    N1,
    N2,
    N3,
    N4,
    N6,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum NFoldSub {
    #[default]
    None,
    N1,
    N2,
    N3,
    N4,
    N5,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum NFoldDiag {
    #[default]
    None,
    SingleQuote,
    DoubleQuote,
    Asterisk,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RotationAxis {
    Omitted,
    X,
    Y,
    #[default]
    Z,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RotationType {
    #[default]
    E,
    N2,
    N3,
    N4,
    N6,
    I,
    M,
    M3,
    M4,
    M6,
}

impl Display for RotationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RotationType::E => f.write_str("1"),
            RotationType::N2 => f.write_str("2"),
            RotationType::N3 => f.write_str("3"),
            RotationType::N4 => f.write_str("4"),
            RotationType::N6 => f.write_str("6"),
            RotationType::I => f.write_str("1\u{0305}"),
            RotationType::M => f.write_str("2\u{0305}"),
            RotationType::M3 => f.write_str("3\u{0305}"),
            RotationType::M4 => f.write_str("4\u{0305}"),
            RotationType::M6 => f.write_str("6\u{0305}"),
        }
    }
}
