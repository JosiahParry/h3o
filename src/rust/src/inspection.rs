use std::str::FromStr;

use crate::h3::*;
use extendr_api::prelude::*;
use h3o::CellIndex;

#[extendr]
fn h3_resolution(x: List) -> Vec<i8> {
    x.into_iter()
        .map(|(_, x)| {
            let y = <&H3>::from_robj(&x).unwrap().index.resolution();
            y as i8
        })
        .collect::<Vec<i8>>()
}

#[extendr]
fn h3_base_cell(x: List) -> Vec<i8> {
    x.into_iter()
        .map(|(_, x)| {
            let cell = <&H3>::from_robj(&x).unwrap().index.base_cell();
            let res = u8::from(cell);
            res as i8
        })
        .collect::<Vec<i8>>()
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
fn is_res_class_iii(x: List) -> Vec<bool> {
    x.into_iter()
        .map(|(_, x)| <&H3>::from_robj(&x).unwrap().index.resolution().is_class3())
        .collect::<Vec<bool>>()
}

#[extendr]
fn is_hexagon(x: List) -> Vec<bool> {
    x.into_iter()
        .map(|(_, x)| <&H3>::from_robj(&x).unwrap().index.is_pentagon())
        .collect::<Vec<bool>>()
}

// skip CellIndex::icosahedron_faces
#[extendr]
fn get_face_count(x: List) -> Vec<i32> {
    x.into_iter()
        .map(|(_, x)| <&H3>::from_robj(&x).unwrap().index.max_face_count() as i32)
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
