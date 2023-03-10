use extendr_api::prelude::*;

use crate::h3::H3;
use h3o::{CellIndex, Direction, DirectedEdgeIndex, geom::ToGeo};

#[extendr]
fn is_nb_pairwise_(x: List, y: List) -> Logicals {
    x.into_iter()
        .zip(y.into_iter())
        .map(|((_, x), (_, y))| {
            let y = H3::from(y).index;
            let x = H3::from(x).index;
            let is_nb = x.is_neighbor_with(y);

            match is_nb {
                Ok(is_nb) => Rbool::from_bool(is_nb),
                Err(_is_nb) => Rbool::na_value(),
            }
        })
        .collect::<Logicals>()
}

#[extendr]
fn is_nb_sparse_(x: List, y: List) -> List {
    x
        .into_iter()
        .map(|(_, x)| {
            let xh3 = H3::from(x).index;
            y.iter().map(|(_, y)| {
                let xi_yj_nbs = xh3.is_neighbor_with(H3::from(y).index);
                match xi_yj_nbs {
                    Ok(xi_yj_nbs) => Rbool::from_bool(xi_yj_nbs),
                    Err(_xi_yj_nbs) => Rbool::na_value(),
                }
            })
            .collect::<Logicals>()
        })
        .collect::<List>()
}


#[extendr]
fn h3_edges(x: List, y: List) -> List {
    x
        .into_iter()
        .zip(y.into_iter())
        .map(|((_, x), (_, y))| {
            let x = H3::from(x).index;
            let y = H3::from(y).index;

            H3DEdge { edge:  x.edge(y).unwrap() }
            // match res {
            //     Some(res) => res,

            // }
        })
        .collect::<List>()


}

#[extendr]
fn make_edge(x: Robj, y: Robj) -> H3DEdge {
    let ed = H3::from(x).index.edge(H3::from(y).index).unwrap();
    H3DEdge { edge: ed }
}
extendr_module! {
    mod edgefns;
    fn is_nb_pairwise_;
    fn is_nb_sparse_;
    fn h3_edges;
    fn make_edge;
    impl H3DEdge;
}


#[extendr]
pub struct H3DEdge {
    pub edge: DirectedEdgeIndex,
}

#[extendr]
impl H3DEdge {

    fn new(x: Robj, y: Robj) -> Self {
        let ed = H3::from(x).index.edge(H3::from(y).index).unwrap();
        Self { edge: ed }
    }

    fn to_id(&self) -> String {
        self.edge.to_string()
    }

    fn length(&self, unit: &str) -> f64 {
        match unit {
            "km" => self.edge.length_km(),
            "m" => self.edge.length_m(),
            "rad" => self.edge.length_rads(),
            &_ => self.edge.length_m()
        }
    }
}

