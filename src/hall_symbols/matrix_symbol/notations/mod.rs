use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFold {
    Invalid,
    #[default]
    N1,
    N2,
    N3,
    N4,
    N6,
}

impl Display for NFold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NFold::Invalid => f.write_str("Invalid"),
            NFold::N1 => f.write_str("1"),
            NFold::N2 => f.write_str("2"),
            NFold::N3 => f.write_str("3"),
            NFold::N4 => f.write_str("4"),
            NFold::N6 => f.write_str("6"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFoldSub {
    #[default]
    None,
    N1,
    N2,
    N3,
    N4,
    N5,
}

impl Display for NFoldSub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NFoldSub::None => f.write_str(""),
            NFoldSub::N1 => f.write_str("1"),
            NFoldSub::N2 => f.write_str("2"),
            NFoldSub::N3 => f.write_str("3"),
            NFoldSub::N4 => f.write_str("4"),
            NFoldSub::N5 => f.write_str("5"),
        }
    }
}

impl From<&str> for NFoldSub {
    fn from(value: &str) -> Self {
        match value {
            "1" => Self::N1,
            "2" => Self::N2,
            "3" => Self::N3,
            "4" => Self::N4,
            "5" => Self::N5,
            _ => Self::None,
        }
    }
}

impl From<&char> for NFoldSub {
    fn from(value: &char) -> Self {
        match value {
            '1' => Self::N1,
            '2' => Self::N2,
            '3' => Self::N3,
            '4' => Self::N4,
            '5' => Self::N5,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFoldDiag {
    #[default]
    None,
    SingleQuote,
    DoubleQuote,
    Asterisk,
}

impl Display for NFoldDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NFoldDiag::None => f.write_str(""),
            NFoldDiag::SingleQuote => f.write_str("'"),
            NFoldDiag::DoubleQuote => f.write_str("\""),
            NFoldDiag::Asterisk => f.write_str("*"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum RotationAxis {
    Omitted,
    X,
    Y,
    #[default]
    Z,
}

impl Display for RotationAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RotationAxis::Omitted => f.write_str(""),
            RotationAxis::X => f.write_str("x"),
            RotationAxis::Y => f.write_str("y"),
            RotationAxis::Z => f.write_str(""),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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

impl Ord for RotationType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // abs(order) = order = 1
            (RotationType::E, RotationType::E) => Ordering::Equal,
            (RotationType::E, _) => Ordering::Less,
            (_, RotationType::E) => Ordering::Greater,
            // abs(order) = order = 2
            (RotationType::N2, RotationType::N2) => Ordering::Equal,
            // 2 > -2,  2 > |1|
            (RotationType::N2, RotationType::M) | (RotationType::N2, RotationType::I) => {
                Ordering::Less
            }
            // 2 < 3, 4, 6, |-3|, |-4|, |-6|
            (RotationType::N2, _) => Ordering::Greater,
            // 3 > -1, ±2, -3
            (RotationType::N3, RotationType::N2)
            | (RotationType::N3, RotationType::M)
            | (RotationType::N3, RotationType::I)
            | (RotationType::N3, RotationType::M3) => Ordering::Less,
            (RotationType::N3, RotationType::N3) => Ordering::Equal,
            (RotationType::N3, _) => Ordering::Greater,
            // 4
            (RotationType::N4, RotationType::I)
            | (RotationType::N4, RotationType::N2)
            | (RotationType::N4, RotationType::M)
            | (RotationType::N4, RotationType::M3)
            | (RotationType::N4, RotationType::N3)
            | (RotationType::N4, RotationType::M4) => Ordering::Less,
            (RotationType::N4, RotationType::N4) => Ordering::Equal,
            (RotationType::N4, _) => Ordering::Greater,
            // 6
            (RotationType::N6, RotationType::N6) => Ordering::Equal,
            (RotationType::N6, _) => Ordering::Less,
            // abs(order) = 1 < anything else
            (RotationType::I, RotationType::I) => Ordering::Equal,
            (RotationType::I, _) => Ordering::Greater,
            // abs(order)=2
            (RotationType::M, RotationType::I) => Ordering::Less,
            (RotationType::M, RotationType::M) => Ordering::Equal,
            (RotationType::M, _) => Ordering::Greater,
            (RotationType::M3, RotationType::N2)
            | (RotationType::M3, RotationType::M)
            | (RotationType::M3, RotationType::I) => Ordering::Less,
            (RotationType::M3, RotationType::M3) => Ordering::Equal,
            (RotationType::M3, _) => Ordering::Greater,
            (RotationType::M4, RotationType::I)
            | (RotationType::M4, RotationType::M)
            | (RotationType::M4, RotationType::N2)
            | (RotationType::M4, RotationType::N3)
            | (RotationType::M4, RotationType::M3) => Ordering::Less,
            (RotationType::M4, RotationType::M4) => Ordering::Equal,
            (RotationType::M4, _) => Ordering::Greater,
            (RotationType::M6, RotationType::M6) => Ordering::Equal,
            (RotationType::M6, _) => Ordering::Less,
        }
    }
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

impl PartialOrd for RotationType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
