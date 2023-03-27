use itertools;
use itertools::Itertools;
use ndarray::{arr2, Array2};
use rayon;
use std;

use super::defs::{
    Adjacency, Edge, Edges, Neighbors, Point, SignedIdx, Solution, Vert, Verts, ZAdjacency, ZOrder,
};

pub mod make {
    use super::{
        arr2,
        info::{absumv, absumv2dc, get_max_xyz, get_order_from_n},
        itertools::{iproduct, Itertools},
        modify::{shift_xy, shift_xyz},
        rayon::prelude::*,
        std::iter::zip,
        Adjacency, Neighbors, Point, Verts, ZAdjacency, ZOrder,
    };

    pub fn make_z_graph(n: u32) -> (u32, u32, ZAdjacency, ZOrder, i16) {
        let order = get_order_from_n(n);
        let max_xyz = get_max_xyz(order) as i16;
        let (z_adj, z_order) = make_xs_adjacency(n as usize, max_xyz);
        (n, order, z_adj, z_order, max_xyz - 4)
    }

    fn make_xs_adjacency(n: usize, max_xyz: i16) -> (ZAdjacency, ZOrder) {
        let adj = make_z_adjacency_map(max_xyz);
        (adj, get_zlevel_order(n))
    }

    fn make_z_adjacency_map(max_xyz: Point) -> ZAdjacency {
        let max_xyz_plus_1 = max_xyz + 1;
        let verts = vertices_for_z_adjacency(max_xyz);
        verts
            .par_iter()
            .map(|vert| {
                (
                    *vert,
                    shift_xy(arr2(&[*vert]))
                        .into_iter()
                        .filter(|neigh| *neigh != *vert && absumv2dc(*neigh) <= max_xyz_plus_1)
                        .collect_vec(),
                )
            })
            .collect()
    }

    fn vertices_for_z_adjacency(max_xyz: Point) -> Vec<[i16; 2]> {
        let max_xyz_plus_1 = max_xyz + 1;
        iproduct!(
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2)
        )
        .filter(|&(x, y)| absumv2dc([x, y]) <= max_xyz_plus_1)
        .sorted_by_key(|&(x, y)| (absumv2dc([x, y]), x, y))
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

    pub fn make_adjacency(n: u32) -> Adjacency {
        let order = get_order_from_n(n);
        let max_xyz = get_max_xyz(order) as i16;
        let verts: Vec<[i16; 3]> = vertices(max_xyz);
        adjacency_map(&verts, max_xyz + 2)
    }

    fn vertices(max_xyz: Point) -> Vec<[i16; 3]> {
        let max_xyz_plus_4 = max_xyz + 4;
        iproduct!(
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2),
            (-max_xyz..=max_xyz).step_by(2)
        )
        .filter_map(|(x, y, z)| {
            if absumv([x, y, z]) < max_xyz_plus_4 {
                Some([x, y, z])
            } else {
                None
            }
        })
        .sorted_by_key(|&vert| (absumv(vert), vert[0], vert[1]))
        .collect::<Vec<_>>()
    }

    fn adjacency_map(verts: &Verts, max_xyz_plus_2: Point) -> Adjacency {
        verts
            .par_iter()
            .map(|vert| {
                (
                    *vert,
                    shift_xyz(arr2(&[*vert]))
                        .into_iter()
                        .filter(|new_neighbor_vert| {
                            *vert != *new_neighbor_vert
                                && absumv(*new_neighbor_vert) <= max_xyz_plus_2
                        })
                        .collect::<Neighbors>(),
                )
            })
            .collect()
    }
}

pub mod modify {
    use super::{arr2, Array2, Point};

    pub fn orient(m: [i16; 3], n: [i16; 3]) -> ([i16; 3], [i16; 3]) {
        match m < n {
            true => (m, n),
            false => (n, m),
        }
    }

    pub fn shift_xyz(vert: Array2<Point>) -> Vec<[i16; 3]> {
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

    pub fn shift_xy(vert: Array2<Point>) -> Vec<[i16; 2]> {
        (vert + arr2(&[[2, 0], [-2, 0], [0, 2], [0, -2]]))
            .outer_iter()
            .map(|point| [point[0], point[1]])
            .collect()
    }
}

pub mod info {
    use super::{Point, SignedIdx, Vert};

    pub fn are_adj([a, b, c]: [i16; 3], [x, y, z]: [i16; 3]) -> bool {
        let n = a - x + b - y + c - z;
        let mask = n >> 15;
        (n + mask) ^ mask == 2
    }

    pub fn axis2d((x, y, _): &Vert, (a, b, _): &Vert) -> usize {
        (0..2)
            .find(|&i| [x, y][i] != [a, b][i])
            .expect("Something's wrong, the same verts are being compared.")
    }

    pub fn absumv2d((x, y, _): Vert) -> i16 {
        let abs_sum = [x, y].iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn absumv3d((x, y, z): Vert) -> i16 {
        let abs_sum = [x, y, z].iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn absumv2dc(vert: [i16; 2]) -> i16 {
        let abs_sum = vert.iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn absumv(vert: [i16; 3]) -> Point {
        let abs_sum = vert.iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn get_max_xyz(order: u32) -> SignedIdx {
        (get_n_from_order(order) * 2 - 1) as i32
    }

    pub fn get_order_from_n(n: u32) -> u32 {
        ((4.0 / 3.0) * ((n + 2) * (n + 1) * n) as f64).round() as u32
    }

    pub fn get_n_from_order(order: u32) -> u32 {
        (((3.0 / 4.0) * order as f64).powf(1.0 / 3.0) - 2.0 / 3.0).round() as u32
    }

    pub fn get_color_index(z: i16) -> u32 {
        (z % 4 + 4).try_into().unwrap()
    }
}

pub mod iters {
    pub fn uon(start: usize, end: usize, max_n: usize) -> impl Iterator<Item = usize> {
        (0..max_n + 2).filter_map(move |i| {
            let _uon = (0..max_n * 2 + 2)
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

pub mod check_edge {
    use super::Point;

    pub fn is_valid_edge(
        v1: [i16; 3],
        v2: [i16; 3],
        min_xyz: Point,
        order: u32,
        lead: bool,
    ) -> bool {
        if order < 160 {
            return valid_edge(v1, v2);
        }
        match lead {
            true => valid_main_edge(v1, v2, min_xyz),
            false => valid_other_edge(v1, v2, min_xyz),
        }
    }

    pub fn valid_edge([x1, y1, _]: [i16; 3], [x2, y2, _]: [i16; 3]) -> bool {
        matches!(x1 + y1 + x2 + y2, 4..=10)
    }

    pub fn valid_main_edge([x, y, z]: [i16; 3], [x2, y2, z2]: [i16; 3], min_xyz: Point) -> bool {
        if z.abs() == min_xyz && min_xyz == z2.abs() {
            (x == 1 || x == 3) && y == y2 && y2 == 1 && (x2 == 1 || x2 == 3)
        } else {
            x == x2 && x2 == 1 && y == y2 && y2 == 1
        }
    }

    pub fn valid_other_edge([x, y, z]: [i16; 3], [x2, y2, z2]: [i16; 3], min_xyz: Point) -> bool {
        if z.abs() == min_xyz && min_xyz == z2.abs() {
            (x == 1 || x == 3) && y == y2 && y2 == 3 && (x2 == 1 || x2 == 3)
        } else {
            x == x2 && x2 == 3 && y == y2 && y2 == 1
        }
    }
}

pub mod make_edges_eadjs {
    use super::{Edge, Edges};
    use rayon::prelude::*;

    pub fn make_eadjs([a, b, c]: [i16; 3], [x, y, z]: [i16; 3], min_xyz: i16) -> Edges {
        match (a != x, b != y, c != z) {
            (true, false, false) => [[0, 2, 0], [0, -2, 0], [0, 0, 2], [0, 0, -2]],
            (false, true, false) => [[2, 0, 0], [-2, 0, 0], [0, 0, 2], [0, 0, -2]],
            (false, false, true) => [[2, 0, 0], [-2, 0, 0], [0, 2, 0], [0, -2, 0]],
            _ => panic!("NOT A VALID EDGE"),
        }
        .par_iter()
        .filter_map(|[i, j, k]| {
            get_valid_eadj([a + i, b + j, c + k], [x + i, y + j, z + k], min_xyz)
        })
        .collect()
    }

    pub fn make_edges([a, b, c]: [i16; 3], [x, y, z]: [i16; 3], min_xyz: i16) -> Edges {
        match (a != x, b != y, c != z) {
            (true, false, false) => [[0, 2, 0], [0, -2, 0], [0, 0, 2], [0, 0, -2]],
            (false, true, false) => [[2, 0, 0], [-2, 0, 0], [0, 0, 2], [0, 0, -2]],
            (false, false, true) => [[2, 0, 0], [-2, 0, 0], [0, 2, 0], [0, -2, 0]],
            _ => panic!("NOT A VALID EDGE"),
        }
        .par_iter()
        .filter_map(|[i, j, k]| {
            get_valid_edge([a + i, b + j, c + k], [x + i, y + j, z + k], min_xyz)
        })
        .collect()
    }

    pub fn get_valid_edge([x, y, z]: [i16; 3], [a, b, c]: [i16; 3], min_xyz: i16) -> Option<Edge> {
        match z.abs() == min_xyz
            && min_xyz == c.abs()
            && (x == 1 || x == 3)
            && y == b
            && b == 1
            && (a == 1 || a == 3)
            || x == a && a == 1 && y == b && b == 1
        {
            true => Some(([x, y, z], [a, b, c])),
            false => None,
        }
    }

    pub fn get_valid_eadj([x, y, z]: [i16; 3], [a, b, c]: [i16; 3], min_xyz: i16) -> Option<Edge> {
        match z.abs() == min_xyz
            && min_xyz == c.abs()
            && (x == 1 || x == 3)
            && y == b
            && b == 3
            && (a == 1 || a == 3)
            || x == a && a == 3 && y == b && b == 1
        {
            true => Some(([x, y, z], [a, b, c])),
            false => None,
        }
    }
}

pub mod certify {
    use itertools::all;

    use super::{std::fmt, Adjacency, Itertools, Solution, info::absumv};

    #[derive(Debug, PartialEq)]
    pub enum SequenceID {
        Broken,
        HamChain,
        HamCycle,
    }

    impl fmt::Display for SequenceID {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SequenceID::Broken => write!(f, "Broken"),
                SequenceID::HamChain => write!(f, "HamChain"),
                SequenceID::HamCycle => write!(f, "HamCycle"),
            }
        }
    }

    pub fn id_seq(seq: &Solution, adj: &Adjacency) -> SequenceID {
        if seq.iter().duplicates().count() > 0 || seq.len() != adj.len() {
            return SequenceID::Broken;
        }
        match seq
            .windows(2)
            .all(|window| adj[&window[0]].contains(&window[1]))
        {
            true if adj[&seq[seq.len() - 1]].contains(&seq[0]) => SequenceID::HamCycle,
            true => SequenceID::HamChain,
            false => SequenceID::Broken,
        }
    }

    pub fn is_hamiltonian_circuit(seq: &Solution, order: usize, max_xyz_plus_4: i16) -> SequenceID {
        if seq.iter().duplicates().count() > 0 
            || seq.len() != order 
            || !all(seq.iter(), |[x, y, z]| (x & 1) + (y & 1) + (z & 1) == 3)
            || !all(seq.iter(), |vert| absumv(*vert) < max_xyz_plus_4)
            || seq.iter().fold((0, 0, 0), |acc: (i16, i16, i16), &[x, y, z]| (acc.0 + x, acc.1 + y, acc.2 + z)) != (0, 0, 0)
        {
            return SequenceID::Broken;
        }
        SequenceID::HamCycle
    }
}

pub mod csv_out {
    use std::error::Error;
    use serde::Serialize;
    use plotters::prelude::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    
    
    #[derive(Debug, Serialize, serde::Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Vector {
        x: i16,
        y: i16,
        z: i16
    }

    
    pub fn create_3d_line_plot(file_path: &str) -> Result<(), Box<dyn Error>> {
        let path = Path::new(file_path);
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        let mut rdr = csv::Reader::from_reader(contents.as_bytes());
        let mut x_data = Vec::new();
        let mut y_data = Vec::new();
        let mut z_data = Vec::new();
    
        for result in rdr.deserialize() {
            let vector: Vector = result?;
            x_data.push(vector.x);
            y_data.push(vector.y);
            z_data.push(vector.z);
        }
    
        let root = BitMapBackend::new("3d_line_plot.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
    
        let mut chart = ChartBuilder::on(&root)
            .caption("3D Line Plot", ("sans-serif", 20))
            .build_cartesian_3d(-100..100, -100..100, -100..100)?;
    
        chart.configure_axes().draw()?;
    
        chart.draw_series(LineSeries::new(
            x_data
                .iter()
                .zip(y_data.iter())
                .zip(z_data.iter())
                .map(|((x, y), z)| (*x as i32, *y as i32, *z as i32)),
            &BLACK,
        ))?;
    
        Ok(())
    }

    pub fn vector_to_csv(data: Vec<[i16;3]>, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = std::fs::File::create(&file_path)?;
        let mut writer = csv::Writer::from_writer(file);
        data.iter().for_each(|[x, y, z]| {
            writer.serialize(Vector{x:*x, y:*y, z:*z}).ok();
        });
        writer.flush()?;
        Ok(())
    }
}
