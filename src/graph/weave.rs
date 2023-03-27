use itertools::Itertools;
use ndarray::{arr2, Axis, Slice};
use rayon::prelude::*;

use super::{
    defs::{
        Bobbins, Count, Loom, LoomSlice, Point, Solution, Spindle, Spool, Spun, Tour, TourSlice,
        Var2, Warps, Weaver, Yarn, YarnEnds, ZAdjacency, ZOrder,
    },
    utils::{
        info::{absumv2dc, are_adj, get_color_index},
        make_edges_eadjs::{make_eadjs, make_edges},
    },
};

pub fn weave(n: usize, z_adj: ZAdjacency, z_order: ZOrder, min_xyz: Point, order: u32) -> Solution {
    let mut loom = wrap_and_reflect_loom(n, z_adj, z_order);
    let mut weaver: Weaver = Weaver::new(loom[0].split_off(0), true, min_xyz, order);
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
                .flat_map(|(m, n)| make_edges(*m, *n, min_xyz))
                .collect())
            .into_iter()
            .next()
        {
            if let Some((j, k)) = (&make_eadjs(m, n, min_xyz) & &warp_edges)
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

fn wrap_and_reflect_loom(n: usize, z_adj: ZAdjacency, z_order: ZOrder) -> Loom {
    let spool: Spool = spin_and_color_yarn(z_adj);
    let mut bobbins: Bobbins = Bobbins::with_capacity(n);
    let mut loom: Loom = Loom::with_capacity((n / 2) + 1);
    for (z, length) in z_order {
        wrap_warps_onto_loom(get_warps(z, length, &bobbins, &spool), &mut loom);
        if z != -1 {
            bobbins = pin_ends(&mut loom);
        }
    }
    loom.par_iter_mut().for_each(|thread| {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| {
                    let [x, y, z] = node;
                    [x, y, -z]
                })
                .collect::<Tour>(),
        )
    });
    loom
}

fn spin_and_color_yarn(z_adj: ZAdjacency) -> Spool {
    let order_z = z_adj.len();
    let spindle: &mut Spindle = &mut Spindle::with_capacity(order_z);
    let start: Var2 = *z_adj.keys().max().unwrap();
    let mut spun: Spun = Spun::with_capacity(order_z);
    spindle.push(start);
    spun.insert(start, true);
    let tail = order_z - 5;
    (1..order_z).for_each(|ix| {
        let unspun = get_unspun(spindle, &z_adj, ix, tail, &mut spun);
        spindle.push(unspun);
        spun.insert(unspun, true);
    });
    let blue: Yarn = Yarn::from(spindle.drain(..).collect::<Vec<_>>());
    let red: Yarn = blue.dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]]);
    Spool::from([(3, blue), (1, red)])
}

fn get_unspun(
    spindle: TourSlice,
    z_adj: &ZAdjacency,
    ix: usize,
    tail: usize,
    spun: &mut Spun,
) -> [i16; 2] {
    let [x, y] = *spindle.last().unwrap();
    *z_adj[&[x, y]]
        .iter()
        .filter_map(|node| match (spun.get(node), *node) {
            (Some(true), _) => None,
            (None, fiber)
                if ix < tail || (spindle[spindle.len() - 2][0] == x) != (x == fiber[0]) =>
            {
                Some((node, absumv2dc(fiber)))
            }
            _ => None,
        })
        .max_by_key(|&(_, absumv)| absumv)
        .unwrap()
        .0
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
        node_yarn if bobbins.is_empty() => vec![node_yarn],
        node_yarn => cut_yarn(node_yarn, bobbins),
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
        } else {
            if let Some(first_slice) = yarn.get(prev as usize + 1..=idx) {
                if !first_slice.is_empty() {
                    subtours.push(if cuts.contains(&first_slice[0]) {
                        first_slice.to_vec()
                    } else {
                        first_slice.iter().rev().cloned().collect()
                    });
                }
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
