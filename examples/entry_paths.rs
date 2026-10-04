//! Both entry paths to a space group.
//!
//! Path 1 parses a Hall symbol string. Path 2 assembles the typed
//! parts with the builder. Both resolve to the same table row. The
//! program prints the group number, the HM symbol, the crystal
//! system, and the general positions of one group.

use crystallographic_group::database::SpaceGroupTable;
use crystallographic_group::hall_symbols::{
    LatticeSymbol, Lattices, MatrixSymbol, NFold, OriginShift,
};
use crystallographic_group::SpaceGroup;

fn main() {
    // Entry path 1: parse a Hall symbol string.
    let parsed = SpaceGroup::try_from_str("P 6").expect("P 6 is a table row");
    println!("entry path 1: parse the Hall symbol \"P 6\"");
    print_group(&parsed);

    // Entry path 2: assemble the typed parts.
    let six = MatrixSymbol::new_builder().nfold_body(NFold::N6).build();
    let assembled = SpaceGroup::new(
        LatticeSymbol::new(false, Lattices::P),
        vec![six],
        OriginShift::default(),
    );
    println!("\nentry path 2: assemble the typed parts");
    print_group(&assembled);

    println!(
        "\nboth entry paths resolve to the same table row: {}",
        assembled.number() == parsed.number()
    );
}

/// Print the identity of one group and its general positions.
fn print_group(group: &SpaceGroup) {
    let row = group.number().expect("a table row resolves at construction");
    let table = SpaceGroupTable::all();
    let number = table.space_group_number(row.row());
    let positions = group.general_positions();
    println!("space group number: {number:?}");
    println!("HM symbol          : {:?}", group.hm_symbol());
    println!("crystal system     : {:?}", group.crystal_system());
    println!("general positions  : {} total", positions.len());
    for formula in positions.formulas() {
        println!("  {formula}");
    }
}
