/// see n_order.txt for a list of n and the corresponding order:
/// n: 100 = 1_373_600 vertices
/// ```
/// cargo run --release [N] [N_UPPER_INCLUSIVE] [STEPS]
/// cargo run --release 1 100 2
/// cargo run --release [N] [N_UPPER_INCLUSIVE] [STEPS]
/// cargo run --release 1 100 2
/// ```
/// builds binary under hamcycle/target/release/hamcycle
/// runs binary: ./hamcycle/target/release/hamcycle
/// starts with the first order in the sequence with 32 vertices,
/// creates the graph for that order and solves it,
/// continues to the next orders up to the 100th which is an order with 1,373,600 vertices,
/// makes graph, solves it
/// 1 (start with order 8 end at order 1,373,600) 100 in steps of two: [1, 3, 5, 7...]
/////////////////////////////////////////////////////////////////////////////
extern crate chrono;
extern crate rayon;

pub mod graph;

use graph::{
    defs::*,
    utils::certify::{is_hamiltonian_circuit, SequenceID},
    utils::make::make_xs_graph,
    weave,
};
use std::{env, time::Instant};

pub fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let n_start = args
        .get(1)
        .and_then(|arg| arg.parse().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(100);
    let n_end = args
        .get(2)
        .and_then(|arg| arg.parse().ok())
        .filter(|&parsed| parsed >= n_start)
        .unwrap_or(n_start);
    let steps = args.get(3).and_then(|arg| arg.parse().ok()).unwrap_or(1);
    for level in (n_start..=n_end).step_by(steps) {
        find_solution(level, false)?;
    }
    Ok(())
}

pub fn find_solution(level: u32, _certify: bool) -> Result<Solution, &'static str> {
    let mut start: Instant = Instant::now();
    let (n, order, z_order, min_xyz) = make_xs_graph(level);
    let dur_make = Instant::now() - start;
    start = Instant::now();
    let solution = weave::weave(n as usize, z_order, min_xyz, order);
    let dur_solve = Instant::now() - start;
    println!(
        "| 🇳 {n:>4} | 🕗 MAKE: {} | ⭕️ {order:>10} | 🕗 SOLVE: {} | 📌 HamCycle",
        dur_make.as_secs_f32(),
        dur_solve.as_secs_f32(),
    );
    start = Instant::now();
    let seq_id = is_hamiltonian_circuit(&solution, order as usize, min_xyz + 8);
    let dur_certify = Instant::now() - start;
    println!(
        "| 🇳 {n:>4} | 🕗 MAKE: {} | ⭕️ {order:>10} | 🕗 SOLVE: {} | 📌 {seq_id:?} | 🕗 CERTIFY: {}",
        dur_make.as_secs_f32(),
        dur_solve.as_secs_f32(),
        dur_certify.as_secs_f32()
    );
    assert_eq!(seq_id, SequenceID::HamCycle);
    Ok(solution)
}
