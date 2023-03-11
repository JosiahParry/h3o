use std::str::FromStr;

use crate::h3::*;
use extendr_api::prelude::*;
use h3o::CellIndex;

#[extendr]
fn h3_resolution(x: List) -> Vec<i32> {
    x.into_iter()
        .map(|(_, x)| {
            if x.is_null() {
                i32::MIN
            } else {
                let idx = <&H3>::from_robj(&x);
                match idx {
                    Ok(idx) => idx.index.resolution() as i32, 
                    Err(_) => i32::MIN
                }
            }            
        })
        .collect::<Vec<i32>>()
}

#[extendr]
fn h3_base_cell(x: List) -> Vec<i32> {
    x.into_iter()
        .map(|(_, x)| {
            if x.is_null() {
                i32::MIN
            } else {
                let cell = <&H3>::from_robj(&x);
                match cell {
                    Ok(cell) => u8::from(cell.index.base_cell()) as i32,
                    Err(_) => i32::MIN
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
fn is_valid_h3(x: Vec<String>) -> Vec<bool> {
    x.into_iter()
        .map(|x| is_valid_h3_(x.as_str()))
        .collect::<Vec<bool>>()
}

#[extendr]
fn is_res_class_iii(x: List) -> Logicals {
    x.into_iter()
        .map(|(_, x)| {
            let cell = <&H3>::from_robj(&x);

            match cell {
                Ok(cell) => Rbool::from_bool(cell.index.resolution().is_class3()),
                Err(_) => Rbool::na()
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
fn get_face_count(x: List) -> Vec<i32> {
    x.into_iter()
        .map(|(_, x)| {
            let cell = <&H3>::from_robj(&x);

            match cell {
                Ok(cell) => cell.index.max_face_count() as i32,
                Err(_) => i32::MIN
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
