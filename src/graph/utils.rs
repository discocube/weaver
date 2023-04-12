/// Types used for the Graph and its instances.
pub mod graph_types {
    use std::collections::{HashMap, HashSet};

    pub type Adjacency = HashMap<V3, Neighbors>;
    pub type Neighbors = HashSet<V3>;
    pub type ScalarXyz = i16;
    pub type V2 = [ScalarXyz; 2];
    pub type V3 = [ScalarXyz; 3];
    pub type Vert = (ScalarXyz, ScalarXyz, ScalarXyz);
    pub type Verts = [V3];
    pub type Vs = HashSet<V3>;
    pub type VIMap = HashMap<Vert, V3>;
    pub type Visited = HashMap<V2, bool>;
    pub type ZAdjacency = HashMap<[ScalarXyz; 2], Vec<[ScalarXyz; 2]>>;
    pub type ZOrder = Vec<(ScalarXyz, usize)>;
    pub type ZlevelVsMap = HashMap<ScalarXyz, Vs>;
}

///  Make Graph from n. No longer used to construct cycle owing to heavy memory use.
pub mod make {
    use itertools::{iproduct, Itertools};
    use ndarray::{arr2, Array2};
    use rayon::prelude::*;
    use std::iter::zip;

    use super::graph_types::*;
    use crate::graph::ops::{certify_solution::L1Norm, graph_info_from_n::*};

    pub fn make_z_graph(n: usize) -> (usize, ZAdjacency, ZOrder, i16) {
        let order = n.get_order_from_n();
        let max_xyz = order.get_max_xyz_from_order() as i16;
        let (z_adj, z_order) = make_xs_adjacency(n, max_xyz);
        (order, z_adj, z_order, max_xyz - 4)
    }

    pub fn make_xs_graph(n: usize) -> (usize, ZOrder, i16) {
        let order = n.get_order_from_n();
        let max_xyz = n.get_max_xyz_from_order() as i16;
        let z_order = get_zlevel_order(n);
        (order, z_order, max_xyz - 4)
    }

    fn make_xs_adjacency(n: usize, max_xyz: i16) -> (ZAdjacency, ZOrder) {
        let adj = make_z_adjacency_map(max_xyz);
        (adj, get_zlevel_order(n))
    }

    pub fn make_z_adjacency_map(max_xyz: ScalarXyz) -> ZAdjacency {
        let max_xyz_plus_1 = max_xyz + 1;
        let verts = vertices_for_z_adjacency(max_xyz);
        verts
            .par_iter()
            .map(|vert| {
                (
                    *vert,
                    shift_xy(arr2(&[*vert]))
                        .into_iter()
                        .filter(|neigh| *neigh != *vert && neigh.l1norm() <= max_xyz_plus_1)
                        .collect_vec(),
                )
            })
            .collect()
    }

    fn vertices_for_z_adjacency(max_xyz: ScalarXyz) -> Vec<[i16; 2]> {
        let max_xyz_plus_1 = max_xyz + 1;
        iproduct!(
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2)
        )
        .filter(|&(x, y)| [x, y].l1norm() <= max_xyz_plus_1)
        .sorted_by_key(|&(x, y)| ([x, y].l1norm(), x, y))
        .map(|(x, y)| [x, y])
        .collect::<Vec<_>>()
    }

    fn get_zlevel_order(n: usize) -> Vec<(i16, usize)> {
        zip(
            (-((n * 2 - 1) as i16)..=-1).step_by(2),
            (1..=n).map(|_n| 2 * _n * (_n + 1)),
        )
        .collect()
    }

    pub fn make_adjacency(n: usize) -> Adjacency {
        let order = n.get_order_from_n();
        let max_xyz = order.get_max_xyz_from_order() as i16;
        let verts: Vec<[i16; 3]> = vertices(max_xyz);
        adjacency_map(&verts, max_xyz + 2)
    }

    fn vertices(max_xyz: ScalarXyz) -> Vec<[i16; 3]> {
        let max_xyz_plus_4 = max_xyz + 4;
        iproduct!(
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2)
        )
        .filter_map(|(x, y, z)| ([x, y, z].l1norm() < max_xyz_plus_4).then_some([x, y, z]))
        .sorted_by_key(|&vert| (vert.l1norm(), vert[0], vert[1]))
        .collect::<Vec<_>>()
    }

    fn adjacency_map(verts: &Verts, max_xyz_plus_2: ScalarXyz) -> Adjacency {
        verts
            .par_iter()
            .map(|vert| {
                (
                    *vert,
                    shift_xyz(arr2(&[*vert]))
                        .into_iter()
                        .filter(|new_neighbor_vert| {
                            *vert != *new_neighbor_vert
                                && new_neighbor_vert.l1norm() <= max_xyz_plus_2
                        })
                        .collect::<Neighbors>(),
                )
            })
            .collect()
    }

    pub fn shift_xyz(vert: Array2<ScalarXyz>) -> Vec<[i16; 3]> {
        (vert
            + arr2(&[
                [2, 0, 0],
                [-2, 0, 0],
                [0, 2, 0],
                [0, -2, 0],
                [0, 0, 2],
                [0, 0, -2],
            ]))
        .outer_iter()
        .map(|point| [point[0], point[1], point[2]])
        .collect()
    }

    pub fn shift_xy(vert: Array2<ScalarXyz>) -> Vec<[i16; 2]> {
        (vert + arr2(&[[2, 0], [-2, 0], [0, 2], [0, -2]]))
            .outer_iter()
            .map(|point| [point[0], point[1]])
            .collect()
    }
}

pub mod iters {
    pub fn uon(start: usize, end: usize) -> impl Iterator<Item = usize> {
        (0..2000 + 2).filter_map(move |i| {
            let _uon = (0..2000 * 2 + 2)
                .step_by(2)
                .take(i)
                .map(|n| n * (n + 2))
                .sum();
            if _uon >= start && _uon <= end {
                Some(_uon)
            } else {
                None
            }
        })
    }
}

/// Helper methods for debugging, timing, refactoring.
pub mod debug {
    use chrono::{Datelike, Local, Timelike};
    use std::fmt;

    struct DateTimeString(String);

    impl fmt::Display for DateTimeString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    pub fn get_current_date_time() -> String {
        let now = Local::now();
        format!(
            "📅 {:02}/{:02}/{:02}  ⌚ {:02}:{:02}:{:02}",
            now.day(),
            now.month(),
            now.year() % 100,
            now.hour(),
            now.minute(),
            now.second()
        )
    }

    pub fn calculate_sizes(start: u64, end: u64, step: u64) -> Vec<(u64, f64)> {
        let size_i16x3 = 6;
        let mut order = start;
        let mut sizes = Vec::new();
        while order <= end {
            let num_elements = order as usize;
            let size = size_i16x3 as f64 * num_elements as f64 / 1024.0 / 1024.0 / 1024.0;
            sizes.push((order, size));
            order += step;
        }
        println!("| Order | Size (GB) |");
        println!("|-------|-----------|");
        for (order, size) in &sizes {
            println!("| {order:<5} | {size:<9.2} |");
        }
        sizes
    }
}
