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
