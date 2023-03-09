use extendr_api::prelude::*;

// personal dep
use sfconversions::sfg_to_geometry;

use h3o::{geom::{ToCells}};

// internal deps
use crate::h3::*;
use crate::createh3::match_resolution;

#[extendr]
fn sfg_to_cells(x: Robj, resolution: u8) -> Robj {

    let resolution = match_resolution(resolution);

    let geo = sfg_to_geometry(x).geom;
    let h3geo = h3o::geom::Geometry::from_degrees(geo).unwrap();
    let h3s = h3geo.to_cells(resolution).collect::<Vec<_>>();

    h3s
        .into_iter()
        .map(|x| Robj::from(H3::from(x)))
        .collect::<List>()
        .set_class(vctrs_class())
        .unwrap()

}

#[extendr]
fn sfc_to_cells(x: List, resolution: u8) -> List {
    x
        .into_iter()
        .map(|(_, robj)| sfg_to_cells(robj, resolution))
        .collect::<List>()
}

extendr_module! {
    mod fromsf;
    fn sfg_to_cells;
    fn sfc_to_cells;
}

