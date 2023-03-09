use extendr_api::prelude::*;
use extendr_api::ExternalPtr;

use h3o::CellIndex;

#[derive(Debug, Clone, Copy)]
pub struct H3 {
    pub index: CellIndex,
}

impl From<CellIndex> for H3 {
    fn from(index: CellIndex) -> Self {
        H3 { index: index }
    }
}

impl From<H3> for Robj {
    fn from(h3: H3) -> Self {
        let pntr = ExternalPtr::new(h3);
        pntr.into_robj()
    }
}

impl From<Robj> for H3 {
    fn from(robj: Robj) -> Self {
        let robj: ExternalPtr<H3> = robj.try_into().unwrap();
        let robj: H3 = *robj;
        robj
    }
}

pub fn vctrs_class() -> [String; 3] {
    [String::from("H3"), String::from("vctrs_vctr"), String::from("list")]
}

#[extendr]
fn h3_to_strings(x: List) -> Strings {
        x
        .into_iter()
        .map(|(_, robj)| H3::from(robj).index.to_string())
        .collect::<Strings>() 
}
extendr_module! { 
    mod h3;
    fn h3_to_strings;
}