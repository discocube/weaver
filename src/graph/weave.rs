use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::{Axis, Slice};
use rayon::prelude::*;

use super::{
    defs::{
        Bobbins, Count, GraphGuide, Loom, LoomSlice, Solution, Spool, Tour, Visited, Warps, Weaver,
        Yarn, YarnEnds, DISP_VECTORS, LAST_ZLEVEL,
    },
    utils::{
        absumv::AbSumV,
        add_vec::AddVec,
        info::{are_adj, get_color_index},
        make_edges_eadjs::{make_eadjs, make_edges},
    },
};

pub fn weave(n: usize) -> Solution {
    let graph = GraphGuide::new(n);
    let mut loom = wrap_and_reflect_loom(&graph);
    let mut weaver: Weaver = Weaver::new(loom[0].split_off(0), true, graph.min_xyz, graph.order);
    let mut loom = loom
        .split_off(1)
        .into_iter()
        .map(|mut data| data.drain(..).collect())
        .collect::<Vec<Vec<_>>>();
    loom.iter_mut().for_each(|warp| {
        let warp_edges = weaver.make_edges_for(warp);
        if let Some((m, n)) = (&weaver.edges()
            & &warp_edges
                .iter()
                .flat_map(|(m, n)| make_edges(*m, *n, graph.min_xyz))
                .collect())
            .into_iter()
            .next()
        {
            if let Some((j, k)) = (&make_eadjs(m, n, graph.min_xyz) & &warp_edges)
                .into_iter()
                .next()
            {
                weaver.rotated_to_edge((m, n));
                Weaver::rotate_to_edge(
                    warp,
                    match are_adj(n, j) {
                        true => (j, k),
                        false => (k, j),
                    },
                );
                weaver.data.append(warp);
            }
        }
    });
    weaver.get_weave()
}

fn wrap_and_reflect_loom(graph: &GraphGuide) -> Loom {
    let spool: Spool = spin_and_color_yarn(graph);
    let mut bobbins: Bobbins = Bobbins::with_capacity(graph.n);
    let mut loom: Loom = Loom::with_capacity(graph.loom_size);
    GraphGuide::get_zlevel_order(graph.n)
        .iter()
        .for_each(|(z, length)| {
            wrap_warps_onto_loom(get_warps(*z, *length, &bobbins, &spool), &mut loom);
            match *z == LAST_ZLEVEL {
                false => bobbins = pin_ends(&mut loom),
                true => (),
            }
        });
    loom.par_iter_mut().for_each(|thread| {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&[x, y, z]| [x, y, -z])
                .collect::<Tour>(),
        )
    });
    loom
}

pub fn spin_and_color_yarn(graph: &GraphGuide) -> Spool {
    let start = [graph.max_xyz, 1];
    let mut spindle: Vec<[i16; 2]> = vec![start; graph.zlen];
    let mut visited: Visited = Visited::with_capacity(graph.zlen);
    visited.insert(start, true);
    let mut yxyx = [1, 0].iter().cycle();
    let mut disp_cycler = DISP_VECTORS.iter().cycle();
    let mut curr_disp = disp_cycler.next().unwrap();
    let mut inside = false;
    (0..graph.zlen - 1).for_each(|i| {
        let yx = *yxyx.next().unwrap();
        let mut new_vect = curr_disp[yx].add_vec(spindle[i]);
        let is_visited = visited.get(&new_vect).is_some();
        inside = !inside && is_visited;
        if is_visited || !inside && new_vect.absumv() > graph.max_absumv {
            curr_disp = disp_cycler.next().unwrap();
            new_vect = curr_disp[yx].add_vec(spindle[i]);
        }
        spindle[i + 1] = new_vect;
        visited.insert(new_vect, true);
    });
    let blue: Yarn = Yarn::from(spindle.drain(..).collect::<Vec<_>>());
    let red: Yarn = blue.dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]]);
    Spool::from([(3, blue), (1, red)])
}

fn get_warps(z: i16, length: Count, bobbins: &Tour, spool: &Spool) -> Warps {
    let mut yarn = spool[&get_color_index(z)].clone();
    let start_pos: isize = (yarn.len_of(ndarray::Axis(0)) - length).try_into().unwrap();
    yarn.slice_axis_inplace(Axis(0), Slice::new(start_pos, None, 1));
    match yarn
        .outer_iter()
        .map(|row| [row[0], row[1], z])
        .collect::<Vec<_>>()
    {
        _yarn if bobbins.is_empty() => vec![_yarn],
        _yarn => cut_yarn(_yarn, bobbins),
    }
}

fn cut_yarn(yarn: Tour, cuts: &Tour) -> Warps {
    let mut subtours: Warps = Vec::with_capacity(cuts.len() + 1);
    let last_ix: usize = yarn.len() - 1;
    let last_idx: usize = cuts.len() - 1;
    let mut prev = -1_i32;
    for (e, idx) in cuts
        .iter()
        .filter_map(|node| yarn.iter().position(|&x| x == *node))
        .sorted()
        .enumerate()
    {
        if e == last_idx && idx != last_ix {
            if let Some(first_slice) = yarn.get(prev as usize + 1..idx) {
                if !first_slice.is_empty() {
                    subtours.push(if cuts.contains(&first_slice[0]) {
                        first_slice.to_vec()
                    } else {
                        first_slice.iter().rev().cloned().collect()
                    });
                }
            }
            if let Some(first_slice) = yarn.get(idx..) {
                if !first_slice.is_empty() {
                    subtours.push(if cuts.contains(&first_slice[0]) {
                        first_slice.to_vec()
                    } else {
                        first_slice.iter().rev().cloned().collect()
                    });
                }
            }
        } else if let Some(first_slice) = yarn.get(prev as usize + 1..=idx) {
            if !first_slice.is_empty() {
                subtours.push(if cuts.contains(&first_slice[0]) {
                    first_slice.to_vec()
                } else {
                    first_slice.iter().rev().cloned().collect()
                });
            }
        }
        prev = idx as i32;
    }
    subtours
}

fn pin_ends(loom: &mut LoomSlice) -> Bobbins {
    loom.iter_mut()
        .flat_map(|thread| {
            let [x, y, z] = thread[0];
            let [i, j, k] = thread[thread.len() - 1];
            let lhs = [x, y, z + 2];
            let rhs = [i, j, k + 2];
            thread.push_front(lhs);
            thread.push_back(rhs);
            [lhs, rhs]
        })
        .collect()
}

fn wrap_warps_onto_loom(mut warps: Warps, loom: &mut Loom) {
    for thread in &mut *loom {
        for warp in warps.iter_mut().filter(|w| !w.is_empty()) {
            match (thread.front(), thread.back()) {
                (Some(front), _) if *front == warp[0] => warp
                    .drain(..)
                    .skip(1)
                    .for_each(|node| thread.push_front(node)),
                (_, Some(back)) if *back == warp[0] => {
                    thread.extend(warp.drain(..).skip(1));
                }
                _ => continue,
            }
        }
    }
    warps.iter_mut().filter(|s| !s.is_empty()).for_each(|seq| {
        loom.append(&mut vec![seq.drain(..).collect::<YarnEnds>()]);
    });
}
