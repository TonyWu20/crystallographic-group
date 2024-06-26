/// For cubic, tetragonal, orthorhombic, monoclinic and triclinic crystal systems
pub(crate) const ORDER_48: [&str; 48] = [
    "x,y,z", "-x,-y,z", "z,x,y", "z,-x,-y", "-z,-x,y", "-z,x,-y", "y,z,x", "-y,z,-x", "y,-z,-x",
    "-y,-z,x", "-y,x,z", "y,-x,z", "x,z,-y", "-x,z,y", "y,-x,-z", "-y,x,-z", "-x,y,-z", "x,-y,-z",
    "x,-y,z", "-x,y,z", "y,x,-z", "-y,-x,-z", "-x,-z,-y", "x,-z,y", "z,y,-x", "z,-y,x", "-z,y,x",
    "-z,-y,-x", "-x,-y,-z", "x,y,-z", "-z,-x,-y", "-z,x,y", "z,x,-y", "z,-x,y", "-y,-z,-x",
    "y,-z,x", "-y,z,x", "y,z,-x", "-y,-x,z", "y,x,z", "-x,-z,y", "x,-z,-y", "x,z,y", "-x,z,-y",
    "-z,-y,x", "-z,y,-x", "z,-y,-x", "z,y,x",
];

/// For hexagonal and trigonal crystal systems
pub(crate) const ORDER_24: [&str; 24] = [
    "x,y,z",
    "-y,x-y,z",
    "-x+y,-x,z",
    "-x,-y,z",
    "y,-x+y,z",
    "x-y,x,z",
    "y,x,-z",
    "x-y,-y,-z",
    "-x,-x+y,-z",
    "-y,-x,-z",
    "-x+y,y,-z",
    "x,x-y,-z",
    "-x,-y,-z",
    "y,-x+y,-z",
    "x-y,x,-z",
    "x,y,-z",
    "-y,x-y,-z",
    "-x+y,-x,-z",
    "-y,-x,z",
    "-x+y,y,z",
    "x,x-y,z",
    "y,x,z",
    "x-y,-y,z",
    "-x,-x+y,z",
];

/// For rhombohedral system
pub(crate) const ORDER_12: [&str; 12] = [
    "x,y,z", "z,x,y", "y,z,x", "-z,-y,-x", "-y,-x,-z", "-x,-z,-y", "-x,-y,-z", "-z,-x,-y",
    "-y,-z,-x", "z,y,x", "y,x,z", "x,z,y",
];
