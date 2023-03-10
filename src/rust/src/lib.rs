use extendr_api::prelude::*;

use h3o;
use h3o::CellIndex;

mod createh3;
mod edgefns;
mod fromsf;
mod h3;
mod hierarchical;
mod inspection;
mod togeo;

// every input is going to be a str from R, no pointer bisniss
fn parse_h3str(x: &str) -> CellIndex {
    let x: CellIndex = str::parse(x).unwrap();
    x
}

#[extendr]
fn h3_string_to_u64(x: &str) -> Robj {
    let x = parse_h3str(x);
    let res: u64 = x.try_into().unwrap();
    rprintln!("{res}");
    //let res = res as f64;
    //rprintln!("{res}");
    Robj::from(res).set_class(["integer64"]).unwrap()
}

#[extendr]
fn h3_int_to_string(x: u64) -> () {
    rprintln!("{:?}", x);
}

// #[extendr]
// fn h3_base_cell(x: &str) -> String {
//     let res = parse_h3str(x).base_cell();
//     res.to_string()
// }

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod h3o;
    use createh3;
    use h3;
    use fromsf;
    use inspection;
    use hierarchical;
    use togeo;
    use edgefns;
    fn h3_string_to_u64;
    fn h3_int_to_string;
    //fn h3_base_cell;

}
