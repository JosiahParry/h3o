use extendr_api::prelude::*;
use h3o::{geom::ToGeo, CellIndex, LatLng};

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


#[extendr]
fn h3_to_points_(x: List) -> List {
    x
        .into_iter()
        .map(|(_, robj)| {
            let idx = <&H3>::from_robj(&robj);

            let res = match idx {
                Ok(idx) => {
                    let ll = LatLng::try_from(idx.index).unwrap();
                    let arr = [Rfloat::from(ll.lng()), Rfloat::from(ll.lat())];
                    Doubles::from_values(arr)
                },
                Err(_) => Doubles::from_values([Rfloat::na(), Rfloat::na()])
            };

            res.into_robj().set_class(["XY", "POINT", "sfg"])
        })
        .collect::<List>()
}

extendr_module! {
    mod togeo;
    fn h3_to_geo_;
    fn h3_to_points_;
}
