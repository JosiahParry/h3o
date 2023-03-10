use extendr_api::prelude::*;
use h3o::{geom::ToGeo, CellIndex};

use crate::h3::*;
use sfconversions::geom::Geom;
#[extendr]
fn h3_to_geo_(x: List) -> Robj {
    let h3_vec = x
        .into_iter()
        .map(|(_, x)| <&H3>::from_robj(&x).unwrap().index)
        .collect::<Vec<CellIndex>>();

    let res = h3_vec.to_geom(true).unwrap();
    let res = Geom::from(res);
    Robj::from(res)
}

extendr_module! {
    mod togeo;
    fn h3_to_geo_;
}
