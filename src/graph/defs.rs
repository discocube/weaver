use itertools::Itertools;
use ndarray::Array2;
use std::collections::{HashMap, HashSet, VecDeque};

use super::utils::{check_edge::is_valid_edge, modify::orient};

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
use std::error::Error;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Vector {
    x: i16,
    y: i16,
    z: i16
}

#[derive(Clone, Debug)]
pub struct Weaver {
    pub data: Tour,
    lead: bool,
    min_xyz: Point,
    order: u32, 
}

impl Weaver {
    pub fn new(mut data: YarnEnds, lead: bool, min_xyz: Point, order: u32) -> Weaver {
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
        } else if !(lhs == self.data[0] && rhs == self.data[self.data.len() - 1]){
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
        } else if !(lhs == cycle[0] && rhs == cycle[cycle.len() - 1]){
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

    pub fn create_3d_line_plot(&self) -> Result<(), Box<dyn std::error::Error>> {
        use plotters::prelude::*;
        let root = BitMapBackend::new("3d_plot.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
    
        let mut xx: Vec<i16> = Vec::new();
        let mut yy: Vec<i16> = Vec::new();
        let mut zz: Vec<i16> = Vec::new();
    
        for [x, y, z] in self.data.iter() {
            xx.push(*x);
            yy.push(*y);
            zz.push(*z);
        }
    
        let mut chart = ChartBuilder::on(&root)
            .caption("3D Line Plot", ("Arial", 30).into_font())
            .build_cartesian_3d(-100..100, -100..100, -100..100)?;
    
            chart.draw_series(LineSeries::new(
                xx
                    .iter()
                    .zip(yy.iter())
                    .zip(zz.iter())
                    .map(|((x, y), z)| (*x as i32, *y as i32, *z as i32)),
                &BLACK,
            ))?;
    
        Ok(())
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
        let file = std::fs::File::create(&file_path)?;
        let mut writer = csv::Writer::from_writer(file);
        self.data.iter().for_each(|[x, y, z]| {
            writer.serialize(Vector{x:*x, y:*y, z:*z}).ok();
        });
        writer.flush()?;
        Ok(())
    }
}
