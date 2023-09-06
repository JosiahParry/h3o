use extendr_api::prelude::*;
use h3o::{geom::ToGeo, CellIndex, LatLng};

use crate::h3::*;

use sfconversions::Geom;

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
    let res = x.into_iter()
        .map(|(_, robj)| {
            let idx = <&H3>::from_robj(&robj);

            let res = match idx {
                Ok(idx) => {
                    let ll = LatLng::try_from(idx.index).unwrap();
                    let arr = [Rfloat::from(ll.lng()), Rfloat::from(ll.lat())];
                    Doubles::from_values(arr)
                }
                Err(_) => Doubles::from_values([Rfloat::na(), Rfloat::na()]),
            };

            res.into_robj().set_class(["XY", "POINT", "sfg"]).unwrap()
        })
        .collect::<Vec<Robj>>();

    List::from_values(res)
}

#[extendr]
fn h3_to_vertexes_(x: List) -> List {
    let res = x.into_iter()
        .map(|(_, robj)| {

            if robj.is_null() {
                Robj::from(RMatrix::new_matrix(0, 2, |_r, _c| 0))
                    .set_class(["XY", "MULTIPOINT", "sfg"])
                    .unwrap()
            } else {
                let vs = <&H3>::from_robj(&robj).unwrap().index.vertexes();

                let ps = vs
                    .map(|p| p.to_geom(true).unwrap())
                    .collect::<Vec<geo_types::Point>>();
    
                let mp = geo_types::MultiPoint::new(ps);
    
                sfconversions::tosf::to_sfg(Geom::from(mp))
            }

        })
        .collect::<Vec<Robj>>();

    List::from_values(res)
}

extendr_module! {
    mod togeo;
    fn h3_to_geo_;
    fn h3_to_points_;
    fn h3_to_vertexes_;
}
