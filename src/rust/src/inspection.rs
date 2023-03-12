use std::str::FromStr;

use crate::h3::*;
use extendr_api::prelude::*;
use h3o::CellIndex;

#[extendr]
/// H3 Inspection Functions
///
/// Functions that provide metadata about H3 indexes.
///
/// @param x an `H3` vector.
///
/// @details
/// - `h3_resolution()`: returns the resolution of each H3 cell.
/// - `h3_base_cell()`: returns the base cell integer.
/// - `is_valid_h3()`: given a vector of H3 index string IDs, determine if they are valid.
/// - `is_res_class_iii()`: determines if an H3 cell has Class III orientation.
/// - `is_pentagon()`: determines if an H3 cell is one of the rare few pentagons.
/// - `get_face_count()`: returns the number of faces that intersect with the H3 index.
///
/// @export
/// @rdname inspection
fn h3_resolution(x: List) -> Vec<i32> {
    x.into_iter()
        .map(|(_, x)| {
            if x.is_null() {
                i32::MIN
            } else {
                let idx = <&H3>::from_robj(&x);
                match idx {
                    Ok(idx) => idx.index.resolution() as i32,
                    Err(_) => i32::MIN,
                }
            }
        })
        .collect::<Vec<i32>>()
}

#[extendr]
/// @export
/// @rdname inspection
fn h3_base_cell(x: List) -> Vec<i32> {
    x.into_iter()
        .map(|(_, x)| {
            if x.is_null() {
                i32::MIN
            } else {
                let cell = <&H3>::from_robj(&x);
                match cell {
                    Ok(cell) => u8::from(cell.index.base_cell()) as i32,
                    Err(_) => i32::MIN,
                }
            }
        })
        .collect::<Vec<i32>>()
}

// scalar implementation
fn is_valid_h3_(x: &str) -> bool {
    let mb = CellIndex::from_str(x);
    match mb {
        Ok(_mb) => true,
        Err(_mb) => false,
    }
}

#[extendr]
/// @export
/// @rdname inspection
fn is_valid_h3(x: Vec<String>) -> Vec<bool> {
    x.into_iter()
        .map(|x| is_valid_h3_(x.as_str()))
        .collect::<Vec<bool>>()
}

#[extendr]
/// @export
/// @rdname inspection
fn is_res_class_iii(x: List) -> Logicals {
    x.into_iter()
        .map(|(_, x)| {
            let cell = <&H3>::from_robj(&x);

            match cell {
                Ok(cell) => Rbool::from_bool(cell.index.resolution().is_class3()),
                Err(_) => Rbool::na(),
            }
        })
        .collect::<Logicals>()
}

#[extendr]
fn is_hexagon(x: List) -> Logicals {
    x.into_iter()
        .map(|(_, x)| {
            let cell = <&H3>::from_robj(&x);

            match cell {
                Ok(cell) => Rbool::from_bool(cell.index.is_pentagon()),
                Err(_) => Rbool::na(),
            }
        })
        .collect::<Logicals>()
}

// skip CellIndex::icosahedron_faces
#[extendr]
/// @export
/// @rdname inspection
fn get_face_count(x: List) -> Vec<i32> {
    x.into_iter()
        .map(|(_, x)| {
            let cell = <&H3>::from_robj(&x);

            match cell {
                Ok(cell) => cell.index.max_face_count() as i32,
                Err(_) => i32::MIN,
            }
        })
        .collect::<Vec<i32>>()
}

extendr_module! {
    mod inspection;
    fn h3_resolution;
    fn h3_base_cell;
    fn is_valid_h3;
    fn is_res_class_iii;
    fn is_hexagon;
    fn get_face_count;
}
