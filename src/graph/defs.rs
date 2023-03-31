use itertools::Itertools;
use ndarray::Array2;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::zip,
};

use super::utils::{check_edge::is_valid_edge, modify::orient};

pub const DISP_VECTORS: [[[i16; 2]; 2]; 4] = [
    [[-2, 0], [0, -2]],
    [[-2, 0], [0, 2]],
    [[2, 0], [0, 2]],
    [[2, 0], [0, -2]],
];

pub type Adjacency = HashMap<Node, Neighbors>;
pub type ZAdjacency = HashMap<[Point; 2], Vec<[Point; 2]>>;
pub type Bobbins = Vec<Node>;
pub type Count = usize;
pub type Edge = (Node, Node);
pub type Edges = HashSet<Edge>;
pub type Loom = Vec<VecDeque<[i16; 3]>>;
pub type LoomSlice = [VecDeque<[i16; 3]>];
pub type Neighbors = HashSet<[i16; 3]>;
pub type Node = [i16; 3];
pub type Nodes = HashSet<Node>;
pub type Order = u32;
pub type Point = i16;
pub type Points = HashSet<Point>;
pub type Solution = Tour;
pub type Spindle = Vec<[i16; 2]>;
pub type Spool = HashMap<u32, Yarn>;
pub type Spun = HashMap<[i16; 2], bool>;
pub type Subtours = Vec<Tour>;
pub type Tour = Vec<[i16; 3]>;
pub type TourSlice<'a> = &'a [[i16; 2]];
pub type YarnEnds = VecDeque<Node>;
pub type Var2 = [i16; 2];
pub type Vert = (Point, Point, Point);
pub type Verts = [[i16; 3]];
pub type VecVert = Vec<Vert>;
pub type VIMap = HashMap<Vert, Node>;
pub type Warps = Vec<Vec<[i16; 3]>>;
pub type Weights = HashMap<Node, Point>;
pub type SignedIdx = i32;
pub type Yarn = Array2<Point>;
pub type ZlevelNodesMap = HashMap<Point, Nodes>;
pub type ZOrder = Vec<(Point, usize)>;
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum XY {
    X,
    Y,
}

pub struct GraphGuide {
    pub n: usize,
    pub order: usize,
    pub zlen: usize,
    pub min_xyz: i16,
    pub max_xyz: i16,
    pub max_absumv: i16,
    pub z_order: Vec<(i16, usize)>
}

impl GraphGuide {
    pub fn new(n: usize) -> GraphGuide {
        let max_xyz = GraphGuide::get_max_xyz_from_n16(n);
        GraphGuide { 
            n, 
            order: GraphGuide::get_order_from_n(n), 
            zlen: GraphGuide::get_zlen(n), 
            max_xyz,
            min_xyz: max_xyz - 4, 
            max_absumv: max_xyz + 1,
            z_order: GraphGuide::get_zlevel_order(n)
        }
    }

    pub fn get_max_xyz(order: usize) -> i32 {
        (GraphGuide::get_n_from_order(order) * 2 - 1) as i32
    }

    pub fn get_max_xyz_i16(order: usize) -> i16 {
        (GraphGuide::get_n_from_order(order) * 2 - 1) as i16
    }

    pub fn min_xyz_from_max(max_xyz: i16) -> i16 {
        max_xyz - 4
    }

    pub fn get_max_xyz_from_n(n: u32) -> i32 {
        (n * 2 - 1) as i32
    }

    pub fn get_max_xyz_from_n16(n: usize) -> i16 {
        (n * 2 - 1) as i16
    }

    pub fn get_max_absumv_from_n16(n: usize) -> i16 {
        (n * 2 + 1) as i16
    }

    pub fn get_min_xyz_from_n16(n: usize) -> i16 {
        (n * 2 - 5) as i16
    }

    pub fn get_order_from_n(n: usize) -> usize {
        ((4.0 / 3.0) * ((n + 2) * (n + 1) * n) as f64).round() as usize
    }

    pub fn get_order_from_n_u64(n: u64) -> u64 {
        ((4.0 / 3.0) * ((n + 2) * (n + 1) * n) as f64).round() as u64
    }

    pub fn get_n_from_order(order: usize) -> u32 {
        (((3.0 / 4.0) * order as f64).powf(1.0 / 3.0) - 2.0 / 3.0).round() as u32
    }

    pub fn get_color_index(z: i16) -> u32 {
        (z % 4 + 4).try_into().unwrap()
    }

    pub fn get_zlen(n: usize) -> usize {
        2 * n * (n + 1)
    }

    fn get_zlevel_order(n: usize) -> Vec<(i16, usize)> {
        zip(
            (-((n * 2 - 1) as i16)..=-1).step_by(2),
            (1..=n).map(|_n| 2 * _n * (_n + 1)),
        )
        .collect()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Vector {
    x: i16,
    y: i16,
    z: i16,
}

#[derive(Clone, Debug)]
pub struct Weaver {
    pub data: Tour,
    lead: bool,
    min_xyz: Point,
    order: usize,
}

impl Weaver {
    pub fn new(mut data: YarnEnds, lead: bool, min_xyz: Point, order: usize) -> Weaver {
        let mut preallocated = Vec::with_capacity(order as usize);
        preallocated.extend(data.drain(..));
        Weaver {
            data: preallocated,
            lead,
            min_xyz,
            order,
        }
    }

    pub fn make_edges_for(&self, other_data: &Tour) -> Edges {
        other_data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(m, n)| is_valid_edge(m, n, self.min_xyz, self.order, false))
            .collect()
    }

    pub fn rotated_to_edge(&mut self, (lhs, rhs): ([i16; 3], [i16; 3])) {
        // more prudent than using splice() which has preferences for optimality:
        // The tail (elements in the vector after range) is empty,
        // or replace_with yields fewer or equal elements than range’s length
        // or the lower bound of its size_hint() is exact. blah blah blah.
        // More steps to fulfill optimality than just doing what's correct.
        if lhs == self.data[self.data.len() - 1] && rhs == self.data[0] {
            self.data.reverse();
        } else if !(lhs == self.data[0] && rhs == self.data[self.data.len() - 1]) {
            match (
                self.data.iter().position(|&x| x == lhs).unwrap(),
                self.data.iter().position(|&x| x == rhs).unwrap(),
            ) {
                (idx_lhs, idx_rhs) if idx_lhs < idx_rhs => {
                    self.data.rotate_left(idx_rhs);
                    self.data.reverse()
                }
                (idx_lhs, _) => self.data.rotate_left(idx_lhs),
            }
        }
    }

    pub fn rotate_to_edge(cycle: &mut Tour, (lhs, rhs): ([i16; 3], [i16; 3])) {
        if lhs == cycle[cycle.len() - 1] && rhs == cycle[0] {
            cycle.reverse();
        } else if !(lhs == cycle[0] && rhs == cycle[cycle.len() - 1]) {
            match (
                cycle.iter().position(|&x| x == lhs).unwrap(),
                cycle.iter().position(|&x| x == rhs).unwrap(),
            ) {
                (idx_lhs, idx_rhs) if idx_lhs < idx_rhs => {
                    cycle.rotate_left(idx_rhs);
                    cycle.reverse()
                }
                (idx_lhs, _) => cycle.rotate_left(idx_lhs),
            }
        }
    }

    pub fn edges(&mut self) -> Edges {
        self.data
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| orient(*a, *b))
            .filter(|&(m, n)| is_valid_edge(m, n, self.min_xyz, self.order, self.lead))
            .collect()
    }

    pub fn get_weave(&self) -> Solution {
        self.data.to_vec()
    }

    /// Python script using plotly and pandas to display the solution from the .csv file produced by the function below:
    ///```
    /// import pandas as pd
    /// import plotly.express as px
    ///
    /// def create_3d_line_plot(file_path):
    ///     df = pd.read_csv(file_path)
    ///     fig = px.line_3d(df, x='X', y='Y', z='Z')
    ///     fig.show()
    ///
    ///```
    /// Save solution to file_path as a csv file where each axis X, Y, Z is a separate column.
    /// Structured for easy read and plotting using python and plotly (see above).
    pub fn save_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = std::fs::File::create(file_path)?;
        let mut writer = csv::Writer::from_writer(file);
        self.data.iter().for_each(|[x, y, z]| {
            writer
                .serialize(Vector {
                    x: *x,
                    y: *y,
                    z: *z,
                })
                .ok();
        });
        // Add first value to last to form a loop:
        let [x, y, z] = self.data[0];
        writer.serialize(Vector { x, y, z })?;
        writer.flush()?;
        Ok(())
    }
}
