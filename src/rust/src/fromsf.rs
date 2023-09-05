use extendr_api::prelude::*;

// personal dep
//use sfconversions::sfg_to_geometry;
use sfconversions::sfg_to_geometry;

use h3o::geom::ToCells;

// internal deps
use crate::createh3::match_resolution;
use crate::h3::*;

#[extendr]
fn sfg_to_cells(x: Robj, resolution: u8) -> Robj {
    let resolution = match_resolution(resolution);

    let geo = sfg_to_geometry(x).geom;
    let h3geo = h3o::geom::Geometry::from_degrees(geo).unwrap();

    let res = h3geo
        .to_cells(resolution)
        .map(|x| H3::from(x))
        .collect::<Vec<H3>>();

    List::from_values(res)
        .set_class(vctrs_class())
        .unwrap()
}

#[extendr]
/// Convert an sfc object to cells
/// @export
fn sfc_to_cells(x: List, resolution: i32) -> List {
    let resolution = resolution as u8;

    let res = x.into_iter()
        .map(|(_, robj)| {
            if robj.len() == 0 || robj.is_null() || robj.is_na() {
                list!().set_class(vctrs_class()).unwrap()
            } else {
                sfg_to_cells(robj, resolution)
            }
        })
        .collect::<Vec<Robj>>();

    List::from_values(res)
}

extendr_module! {
    mod fromsf;
    fn sfg_to_cells;
    fn sfc_to_cells;
}
