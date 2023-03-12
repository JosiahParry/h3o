use extendr_api::prelude::*;

use crate::h3::*;

#[extendr]
fn grid_disk_fast_(x: List, k: u32) -> List {
    x.into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                list!().set_class(vctrs_class()).unwrap()
            } else {
                let ind = <&H3>::from_robj(&robj).unwrap().index;
                ind.grid_disk_fast(k)
                    .map(|x| 
                        // can be null sometimes 
                        // if it messed up catch it and return null
                        match x {
                            Some(x) => Robj::from(H3::from(x)),
                            None => Robj::from(extendr_api::NULL)
                        })
                    .collect::<List>()
                    .set_class(vctrs_class())
                    .unwrap()
            }
        })
        .collect::<List>()
}

#[extendr]
fn grid_disk_safe_(x: List, k: u32) -> List {
    x.into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                list!().set_class(vctrs_class()).unwrap()
            } else {
                let ind = <&H3>::from_robj(&robj).unwrap().index;
                ind.grid_disk_safe(k)
                    .map(|x| Robj::from(H3::from(x)))
                    .collect::<List>()
                    .set_class(vctrs_class())
                    .unwrap()
            }
        })
        .collect::<List>()
}

#[extendr]
fn grid_distances_(x: List, k: u32) -> List {
    x.into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                let vc: Vec<u32> = Vec::with_capacity(0);
                vc
            } else {
                let ind = <&H3>::from_robj(&robj).unwrap().index;
                ind.grid_disk_distances::<Vec<_>>(k)
                    .into_iter()
                    .map(|(_, dist)| dist)
                    .collect::<Vec<u32>>()
            }
        })
        .collect::<List>()
}

#[extendr]
fn grid_ring_(x: List, k: u32) -> List {
    x.into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                list!().set_class(vctrs_class()).unwrap()
            } else {
                <&H3>::from_robj(&robj)
                    .unwrap()
                    .index
                    .grid_ring_fast(k)
                    .map(|x| {
                        // can be null sometimes
                        // if it messed up catch it and return null
                        match x {
                            Some(x) => Robj::from(H3::from(x)),
                            None => Robj::from(extendr_api::NULL),
                        }
                    })
                    .collect::<List>()
                    .set_class(vctrs_class())
                    .unwrap()
            }
        })
        .collect::<List>()
}

#[extendr]
fn grid_path_cells_(x: List, y: List) -> List {
    x.into_iter()
        .zip(y.into_iter())
        .map(|((_, x), (_, y))| {
            if x.is_null() || y.is_null() {
                list!().set_class(vctrs_class()).unwrap()
            } else {
                let x = <&H3>::from_robj(&x).unwrap().index;
                let y = <&H3>::from_robj(&y).unwrap().index;
                let path = x.grid_path_cells(y);
                let path = match path {
                    Ok(path) => path
                        .into_iter()
                        .map(|x| match x {
                            Ok(x) => Robj::from(H3::from(x)),
                            Err(_x) => Robj::from(extendr_api::NULL),
                        })
                        .collect::<List>(),
                    // idk if this is the right way to handle it
                    Err(_path) => list!(),
                };

                path.set_class(vctrs_class()).unwrap()
            }
        })
        .collect::<List>()
}

#[extendr]
fn grid_path_cells_size_(x: List, y: List) -> Integers {
    x.into_iter()
        .zip(y.into_iter())
        .map(|((_, x), (_, y))| {
            if x.is_null() || y.is_null() {
                Rint::na()
            } else {
                let x = <&H3>::from_robj(&x).unwrap().index;
                let y = <&H3>::from_robj(&y).unwrap().index;
                let size = x.grid_path_cells_size(y);

                match size {
                    Ok(size) => Rint::from(size),
                    Err(_size) => Rint::na(),
                }
            }
        })
        .collect::<Integers>()
}

#[extendr]
fn grid_distance_(x: List, y: List) -> Integers {
    x.into_iter()
        .zip(y.into_iter())
        .map(|((_, x), (_, y))| {
            if x.is_null() || y.is_null() {
                Rint::na()
            } else {
                let d = <&H3>::from_robj(&x)
                    .unwrap()
                    .index
                    .grid_distance(<&H3>::from_robj(&y).unwrap().index);

                match d {
                    Ok(d) => Rint::from(d),
                    Err(_) => Rint::na(),
                }
            }
        })
        .collect::<Integers>()
}

#[extendr]
// x is anchor
// y is origin
fn local_ij_(x: List, y: List) -> List {
    let (i, j): (Vec<i32>, Vec<i32>) = x
        .into_iter()
        .zip(y.into_iter())
        .map(|((_, x), (_, y))| {
            if x.is_null() || y.is_null() {
                (i32::MIN, i32::MIN)
            } else {
                let x = <&H3>::from_robj(&x).unwrap().index;
                let y = <&H3>::from_robj(&y).unwrap().index;

                let res = x.to_local_ij(y);
                match res {
                    Ok(res) => (res.i(), res.j()),
                    Err(_) => (i32::MIN, i32::MIN),
                }
            }
        })
        .unzip();
    list!(i = i, j = j)
}

extendr_module! {
    mod grid_traversal;
    fn grid_disk_fast_;
    fn grid_disk_safe_;
    fn grid_distances_;
    fn grid_ring_;
    fn grid_path_cells_;
    fn grid_path_cells_size_;
    fn grid_distance_;
    fn local_ij_;
}
