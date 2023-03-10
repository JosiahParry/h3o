use crate::{createh3::match_resolution, h3::*};
use extendr_api::prelude::*;
use h3o::CellIndex;
use std::iter::FromIterator;

#[extendr]
fn get_parents_(x: List, resolution: u8) -> Robj {
    let reso = match_resolution(resolution);
    x.into_iter()
        .map(|(_, robj)| Robj::from(H3::from(H3::from(robj).index.parent(reso).unwrap())))
        .collect::<List>()
        .set_class(vctrs_class())
        .unwrap()
}

#[extendr]
fn get_children_(x: List, resolution: u8) -> List {
    let reso = match_resolution(resolution);
    x.into_iter()
        .map(|(_, robj)| {
            let children = H3::from(robj).index.children(reso);
            children
                .map(|child| Robj::from(H3::from(child)))
                .collect::<List>()
                .set_class(vctrs_class())
                .unwrap()
        })
        .collect::<List>()
}

#[extendr]
fn get_children_count_(x: List, resolution: u8) -> Vec<i32> {
    let reso = match_resolution(resolution);
    x.into_iter()
        .map(|(_, robj)| H3::from(robj).index.children_count(reso) as i32)
        .collect::<Vec<i32>>()
}

#[extendr]
fn get_children_center_(x: List, resolution: u8) -> Robj {
    let reso = match_resolution(resolution);
    x.into_iter()
        .map(|(_, robj)| {
            let child = H3::from(robj).index.center_child(reso);

            match child {
                Some(child) => Robj::from(H3::from(child)),
                None => Robj::from(extendr_api::NULL),
            }
        })
        .collect::<List>()
        .set_class(vctrs_class())
        .unwrap()
}

#[extendr]
fn get_children_position_(x: List, resolution: u8) -> Integers {
    let reso = match_resolution(resolution);
    let res = x.into_iter().map(|(_, robj)| {
        let child = H3::from(robj).index.child_position(reso);

        match child {
            Some(child) => Rint::from(child as i32),
            None => Rint::na(),
        }
    });

    Integers::from_iter(res)
}

#[extendr]
fn get_children_at_(x: List, position: i32, resolution: u8) -> Robj {
    let reso = match_resolution(resolution);
    x.into_iter()
        .map(|(_, robj)| {
            let child = H3::from(robj).index.child_at(position as u64, reso);
            match child {
                Some(child) => Robj::from(H3::from(child)),
                None => Robj::from(extendr_api::NULL),
            }
        })
        .collect::<List>()
        .set_class(vctrs_class())
        .unwrap()
}

#[extendr]
fn compact_cells_(x: List) -> Robj {
    let h3_vec = x
        .into_iter()
        .map(|(_, robj)| H3::from(robj).index)
        .collect::<Vec<CellIndex>>();

    CellIndex::compact(h3_vec)
        .unwrap()
        .into_iter()
        .map(|x| Robj::from(H3::from(x)))
        .collect::<List>()
        .set_class(vctrs_class())
        .unwrap()
}

#[extendr]
fn uncompact_cells_(x: List, resolution: u8) -> List {
    let reso = match_resolution(resolution);

    x.into_iter()
        .map(|(_, robj)| {
            let uncompacted = CellIndex::uncompact(std::iter::once(H3::from(robj).index), reso);
            uncompacted
                .map(|x| Robj::from(H3::from(x)))
                .collect::<List>()
                .set_class(vctrs_class())
                .unwrap()
        })
        .collect::<List>()
}

// skipping uncompactCellSize

extendr_module! {
    mod hierarchical;
    fn get_parents_;
    fn get_children_;
    fn get_children_count_;
    fn get_children_center_;
    fn get_children_position_;
    fn get_children_at_;
    fn compact_cells_;
    fn uncompact_cells_;
}
