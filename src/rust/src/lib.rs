use extendr_api::prelude::*;

use h3o;
use h3o::CellIndex;

mod h3;
mod createh3;
mod fromsf;

// every input is going to be a str from R, no pointer bisniss
fn parse_h3str(x: &str) -> CellIndex {
    let x: CellIndex = str::parse(x).unwrap();
    x
}

#[extendr]
fn h3_string_to_u64(x: &str) -> u64 {
    let x = parse_h3str(x);
    let res: u64 = x.try_into().unwrap();
    res
}




#[extendr]
fn h3_int_to_string(x: f64) -> String {
    let x = x as u64;
    let res = CellIndex::try_from(x).unwrap();
    res.to_string()
}


#[extendr]
fn h3_base_cell(x: &str) -> String {
    let res = parse_h3str(x).base_cell();
    res.to_string()
}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod h3o;
    use createh3;
    use h3;
    use fromsf;
    fn h3_string_to_u64;
    fn h3_int_to_string;
    fn h3_base_cell;
    
}
