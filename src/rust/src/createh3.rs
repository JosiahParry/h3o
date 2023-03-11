use std::str::FromStr;

use extendr_api::prelude::*;
use extendr_api::Strings;
use h3o::Resolution;
use h3o::{CellIndex, LatLng};

use crate::h3::{vctrs_class, H3};

#[extendr]
fn h3_from_string_(x: Strings) -> Robj {
    x.into_iter()
        .map(|strng| {
            if strng.is_na() {
                Robj::from(extendr_api::NULL)
            } else {
                Robj::from(H3::from(CellIndex::from_str(strng.as_str()).unwrap()))
            }
        })
        .collect::<List>()
        .set_class(vctrs_class())
        .unwrap()
}

#[extendr]
fn h3_from_points_(x: List, resolution: u8) -> Robj {
    let reso = match_resolution(resolution);

    x.into_iter()
        .map(|(_, robj)| {
            let dbls = Doubles::try_from(robj).unwrap();
            let ll = LatLng::new(dbls[1].0, dbls[0].0);
            
            match ll {
                Ok(ll) => {
                    Robj::from(H3::from(ll.to_cell(reso)))
                },
                Err(_) => Robj::from(extendr_api::NULL)
            }
        })
        .collect::<List>()
        .set_class(vctrs_class())
        .unwrap()
}

#[extendr]
fn h3_from_xy_(x: Doubles, y: Doubles, resolution: u8) -> Robj {
    let reso = match_resolution(resolution);

    x
        .into_iter()
        .zip(y.iter())
        .map(|(x, y)| {
            if x.is_na() || y.is_na() {
                Robj::from(extendr_api::NULL)
            } else {
                Robj::from(H3::from(LatLng::new(x.0, y.0).unwrap().to_cell(reso)))
            }
        })
        .collect::<List>()
        .set_class(vctrs_class())
        .unwrap()
        
}

// boundary for a single hex
fn h3_boundary_(x: Robj) -> Robj {
    //let h3 = <&H3>::from_robj(&x).unwrap();
    let h3 = <&H3>::from_robj(&x).unwrap();
    let boundary = h3.index.boundary();

    let mut coords = boundary
        .into_iter()
        .map(|x| [x.lng(), x.lat()])
        .collect::<Vec<[f64; 2]>>();

    coords.push(coords[0].clone());

    let m = RMatrix::new_matrix(coords.len(), 2, |r, c| coords[r][c]);

    list![m].set_class(["XY", "POLYGON", "sfg"]).unwrap()
}

// vectorized but prettier
#[extendr]
fn h3_boundaries_(x: List) -> List {
    x.into_iter()
        .map(|(_, robj)| h3_boundary_(robj))
        .collect::<List>()
}

pub fn match_resolution(resolution: u8) -> Resolution {
    let tmp = h3o::Resolution::try_from(resolution).unwrap();
    tmp
}

extendr_module! {
    mod createh3;
    fn h3_from_string_;
    fn h3_from_points_;
    fn h3_from_xy_;
    fn h3_boundaries_;
}
