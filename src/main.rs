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
/// GOAL IS 8011618152. 8 BILLION FOR EACH PERSON ON THE EARTH. N = 1817.
///                                                   
///                                                   
///                                                   
///                      ◉―――――――◉                    
///                     ╱│      ╱│                    
///                    ◉―――⊖―――◉―│―◉                  
///                    │ │╱│   │ │╱│                  
///              ◉―――――│―◉―――――│―◉――――――――◉           
///             ╱│     │╱│ │   │╱│ │     ╱│           
///            ◉―――――――◉―――⊖―――◉―――⊖――――◉ │           
///            │ │    ╱│ │╱   ╱│ │╱     │ │           
///            │ ◉―――◉―――⊖―――◉―│―◉――――――│―◉           
///            │╱    │ │╱│   │ │╱│      │╱            
///            ◉―――――│―◉―――――│―◉――――――――◉             
///                  │╱│ │   │╱│ │                    
///                  ◉―――⊖―――◉―│―◉                    
///                    │╱      │╱                     
///                    ◉―――――――◉                      
///                                                   
///                                                   
///                                         
///
///
///                                               
/////////////////////////////////////////////////////////////////////////////
extern crate blas_src;
extern crate chrono;
extern crate rayon;

pub mod graph;

use graph::{defs::*, utils::certify::SequenceID, utils::debug::get_current_date_time, weave};
use std::{env, time::Instant};

pub fn main() -> Result<(), &'static str> {
    std::env::set_var("RUST_BACKTRACE", "1");
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
        find_solutions(level, false)?;
    }
    Ok(())
}

// Solve on one or many by step or by steps. Time it and certify.
pub fn find_solutions(n: usize, _certify: bool) -> Result<Solution, &'static str> {
    let order = n.get_order_from_n();
    if order > 1000000000 {
        println!("{} | SOLVING ORDER ⭕️ {order}", get_current_date_time());
    }
    let mut start: Instant = Instant::now();
    let solution = weave::weave(n);
    let dur_solve = Instant::now() - start;
    if order > 10000000 {
        println!(
            "| 🇳 {n:>4} | ⭕️ {order:>10} | 🕗 {:.10} |",
            dur_solve.as_secs_f32(),
        );
    } else {
        start = Instant::now();
        let seq_id = solution.certify(order, n.get_max_absumv());
        let dur_certify = Instant::now() - start;
        println!(
            "| 🇳 {n:>4} | ⭕️ {order:>10} | 🕗 SOLVE: {:.10} | 📌 {seq_id:?} | 🕗 CERTIFY: {:.10}",
            dur_solve.as_secs_f32(),
            dur_certify.as_secs_f32()
        );
        assert_eq!(seq_id, SequenceID::HamCycle);
    }
    Ok(solution)
}
