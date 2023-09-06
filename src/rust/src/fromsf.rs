use extendr_api::prelude::*;

use sfconversions::fromsf::sfc_to_geometry;

use h3o::geom::ToCells;
use h3o::geom::{PolyfillConfig, ContainmentMode};

// internal deps
use crate::createh3::match_resolution;
use crate::h3::*;

use rayon::prelude::*;

use geo_types::Geometry;

fn geometry_to_cells(x: Geometry, containment: PolyfillConfig) -> Vec<H3> {
    let h3geo = h3o::geom::Geometry::from_degrees(x)
        .unwrap();

    h3geo
        .to_cells(containment)
        .map(H3::from)
        .collect::<Vec<_>>()
}

#[extendr]
fn sfc_to_cells_(x: List, resolution: i32, containment: &str) -> List {

    let resolution = match_resolution(resolution as u8);

    let containment_strategy = match containment {
        "boundary" => ContainmentMode::ContainsBoundary,
        "centroid" => ContainmentMode::ContainsCentroid,
        "intersect" => ContainmentMode::IntersectsBoundary,
        _ => ContainmentMode::ContainsBoundary
    };

    let poly_config = PolyfillConfig::new(resolution)
        .containment_mode(containment_strategy);

    let x = sfc_to_geometry(x);

    let res = x.into_par_iter()
        .map(|xi| {
            match xi {
                Some(xi) => geometry_to_cells(xi, poly_config),
                None => vec![]
            }
        })
        .collect::<Vec<Vec<H3>>>();

    let res = res.into_iter().map(|xi| {
        List::from_values(xi).set_class(vctrs_class()).unwrap()
    })
    .collect::<Vec<Robj>>();

    List::from_values(res)
}

extendr_module! {
    mod fromsf;
    // fn sfg_to_cells;
    fn sfc_to_cells_;
}
