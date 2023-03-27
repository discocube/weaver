/// see n_order.txt for a list of n and the corresponding order:
/// n: 100 = 1_373_600 vertices
/// ```
/// cargo run --release [N] [N_UPPER_INCLUSIVE]
/// cargo run --release 1 100
/// ```
/// builds binary under hamcycle/target/release/hamcycle
/// runs binary: ./hamcycle/target/release/hamcycle
/// starts with the first order in the sequence with 32 vertices,
/// creates the graph for that order and solves it,
/// continues to the next orders up to the 100th which is an order with 1,373,600 vertices,
/// makes graph, solves it
/// 1 (start with order 8 end at order 1,373,600) 100
/////////////////////////////////////////////////////////////////////////////
extern crate plotters;
extern crate rayon;

use std::{env, time::Instant};

pub mod graph;

use graph::{
    defs::*,
    utils::certify::{self, SequenceID},
    utils::make::{make_adjacency, make_z_graph},
    weave,
};

use crate::graph::utils::{csv_out::vector_to_csv, certify::is_hamiltonian_circuit};

pub fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let n_start: u32 = match args.get(1) {
        Some(arg) => match arg.parse::<u32>() {
            Ok(parsed) => {
                if parsed == 0 {
                    1
                } else {
                    parsed
                }
            }
            Err(_) => 100,
        },
        None => 100,
    };
    let n_end: u32 = match args.get(2) {
        Some(arg) => match arg.parse::<u32>() {
            Ok(parsed) => {
                if parsed < n_start {
                    n_start
                } else {
                    parsed
                }
            }
            Err(_) => n_start,
        },
        None => n_start,
    };
    for level in n_start..=n_end {
        find_solution(level, false)?;
    }
    Ok(())
}

pub fn find_solution(level: u32, _certify: bool) -> Result<Solution, &'static str> {
    let mut start: Instant = Instant::now();
    let (n, order, z_adj, z_order, min_xyz) = make_z_graph(level);
    let dur_make = Instant::now() - start;
    start = Instant::now();
    let solution = weave::weave(n as usize, z_adj, z_order, min_xyz, order);
    let dur_solve = Instant::now() - start;

    println!("{:?}", is_hamiltonian_circuit(&solution, order as usize, min_xyz + 8));

    vector_to_csv(solution.clone(), &format!("/home/rommelo/Repos/easy_dc_rust_vec/src/solution_{order}.csv")).unwrap();
    println!(
        "| 🇳 {n:>4} | ⭕️ {order:>10} | 🕗 {} |",
        dur_solve.as_secs_f32()
    );
    if _certify {
        // Because the adjacency used for solving is a partition, we need to make the whole adjacency to certify the sequence:
        // This incurs a significant memory cost for graphs having over 250 million nodes.
        let adj = make_adjacency(n);
        println!("🇳 {n:>4} FINISHED WEAVING. 🔎 CERTIFYING SOLUTION...");
        start = Instant::now();
        let seq_id = certify::id_seq(&solution, &adj);
        let dur_certify = Instant::now() - start;
        println!(
        "| 🇳 {n:>4} | 🕗 MAKE: {} | ⭕️ {order:>10} | 🕗 SOLVE: {} | 📌 {seq_id:?} | 🕗 CERTIFY: {}",
        dur_make.as_secs_f32(),
        dur_solve.as_secs_f32(),
        dur_certify.as_secs_f32()
        );
        assert_eq!(seq_id, SequenceID::HamCycle);
    }
    Ok(solution)
}
