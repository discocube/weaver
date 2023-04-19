use super::ops::prelude::*;

/// 🪡 Weave a Hamiltonian cycle by building chains level by level bottom up halfway up the graph. Mirror chains to form cycles for subsequent joining of weft with each warp in the loom until only the weft remains. A construction algorithm for describing the discocube, a techno-upgrade of the discoball 🪩 that's still a discoball but reflects the intertwined complexity of our algorithmically connected world. A disco ball fits well in a 1920's ballroom, but what fits well in a techno hall? \
///
///---\
/// `🧭 n`: NInfo instance used to get parameters from the graph.\
/// `🧵 spun`: Spool of yarn to be spun and colored.\
/// `🧶 yarns`: Blue and red yarn as an ndarray.\
/// `📌 pins`: Pins are used cut the finished yarn and as markers to connect the current level to the previous.\
/// `🪜 loom`: Where threads are woven level by level using pins as markers to connect the prev to the current level.\
/// `🧣 weft`: Leading loop into which the warps are incorporated. Weft object containing the solution.\
/// `🧮 warps`: Threads which are built horizontally upwards level by level until half the graph has been built. \
/// ---\
///
///```
///pub fn weave(n: usize) -> Solution {
///
///    //  Create a loom instance with a specific size from InfoN.
///    let mut loom = Loom::with_capacity(n.loom_size());
///
///    // Convert to ndarray & assign to blue. Copy/reflect/translate blue & assign to red.
///    let yarns = Yarns::colorized(Spindle::spin(n.spool_size()));
///
///    // Iterate over the setting for each level: `zrow`, `size`, `color`
///    n.zrow_color_size().iter().for_each(|&((z, color), size)| {
///
///        // Get pins for cutting the yarn to match each thread in the loom.
///        let pins = loom.pin_thread_ends();
///
///        // Extend each thread end w/ segmented yarn using pins for cutting.
///        // prep: get requested color and cut from yarns from idx. Add the zrow to the 2d vector to position the yarn to the elevation.
///        // split: using pins, split the finished yarn so that the lhs of each sequence matches to a thread end in the loom.
///        loom.extend_threads(yarns.prep(zrow, color, idx).chop(&pins));
///    });
///
///    // Mirror each thread in loom to form cycles from chains for subsequent cycle merging.
///    // ◡ + ◠ = ◯
///    // Loom before:  ◡ ◡ ◡ ◡ ◡ ◡ ◡ ◡ + ◠ ◠ ◠ ◠ ◠ ◠ ◠ ◠
///    // Loom after:   ◯ ◯ ◯ ◯ ◯ ◯ ◯ ◯
///    loom.mirror_threads();
///
///    // Split weft from the loom leaving only the warps.
///    // solution = ((((weft + warp) + warp) + warp) + warp)
///    let (mut weft, mut loom) = loom.prepare_merging(n);
///
///    // Iterate over each warp in the loom and incorporate into the weft.
///    loom.iter_mut().for_each(|warp| {
///
///        // Get edges which are also valid bridges
///        let warp_edges = warp.edges(weft.max_sum_z, weft.joined);
///
///        // A bridge is an edge that shares an adjacent edge with another sequence and used
///        // as a bridge to join two cycles.
///        // Get the bridge edge on the weft to join to with the bridge edge of the warp.
///        let weft_bridge = weft.edges().bridge(&warp_edges);
///
///        // Align/Rotate weft so the ends match weft's bridge.
///        weft.align_to(weft_bridge);
///
///        // Align/Rotate weft so the ends match warp's bridge.
///        warp.align_to(warp_edges.bridge(&weft_bridge));
///
///        // Now that both are aligned weft joins with warp by appending.
///        weft.join(warp);
///    });
///
///    // After weaving there's is only the weft. Retrieve the finished weave.
///    weft.get_woven()
///
///    // Output results to a csv file:
///    weft.export_csv()
///}
///```
/// ---\
///
/// Spin and color yarn. From the bottom-up for each level: cut the yarn incorporate into the level using pins if necessary to affix to the previous threads. Prepare pins for the next level. When we've reached the top, reflect the loom. Separate the loom into a main weft and warps. Incorporate the weft into the warps. Return solution.\
/// For each level, pin each end of each thread in the loom. Get the requested color and cut yarn using pins.
/// Merge subcycles by first calculating the bridge between warp's and weft's edges: Align each sequence to their respective edge such that the two sequences can be placed next to another. Append the warp to the weaver's weft. Continue to incorporate warps into the weft until only the weft remains.\
///---\
/// ---\
/// I've placed most of the implementations in the ops.rs file to avoid cluttering the structure of the actual algorithm. The ops.rs file is structured so that it follows the order of the weave algorithm, where each function is encapsulated in a separate module and imported using `prelude::*`. Here is a list of the modules with the corresponding call to that module in weave.\
///```
///graph_info_from_n::InfoN ────────────➤  n.loom_size()
///                                        n.spool_size()
///                                        n.zrow_color_len()
///
///spin_yarn::Spin ─────────────────────➤  Spindle::spin()
///
///color_yarn::ColorSpunYarn ───────────➤  Yarns::color_spun()
///
///pin_threads::PinThreadEnds ──────────➤  loom.pin_thread_ends()
///
///prep_yarn::PrepYarnExtensions ───────➤  yarns.prep()
///
///extend_threads::ExtendThreads ───────➤  loom.extend_threads()
///
///mirror_loom::MirrorLoomThreads ──────➤  loom.mirror_threads()
///
///merge_cycles::* ─────────────────────➤  loom.prepare_cycle_merging()
///                                        warp.edges()
///                                        weft.edges().bridge(&warp_edges)
///                                        weft.data.align_to(weft_bridge)
///                                        warp.align_to(warp_edges.bridge(&weft_bridge))
///                                        weft.join(warp)
///
///```
///
///
pub fn weave(n: usize) -> Solution {
    let mut loom = Loom::with_capacity(n.loom_size());
    let yarns = Yarns::color_spun(Spindle::spin(n.spool_size()));
    n.zrow_color_idx().iter().for_each(|&((zrow, color), idx)| {
        let mut pins = loom.pin_thread_ends(zrow);
        loom.extend_threads(yarns.prep(zrow, color, idx).chop(&mut pins));
    });
    loom.mirror_threads();
    let (mut weft, mut loom) = loom.prepare_cycle_merging(n);
    loom.iter_mut().for_each(|warp| {
        let warp_edges = warp.edges(weft.joined);
        let weft_bridge = weft.edges().bridge(&warp_edges);
        weft.data.align_to(weft_bridge);
        warp.align_to(warp_edges.bridge(&weft_bridge));
        weft.join(warp);
    });
    weft.get_woven()
}

///! 🩺 TEST
///!
///!
///!
///!
///!
///! 🩺 Test to check that the results from the first 50 orders are Hamiltonian cycles.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::ops::certify_solution::{Certify, SequenceID};

    /// 🩺 Run weave on the first 50 instances and test if the results are Hamiltonian cycles.
    #[test]
    fn test_weave() {
        for n in 1..=50 {
            let order = n.get_order_from_n();
            let max_absumv = n.get_max_absumv();
            let solution = weave(n);
            let seq_id = solution.certify(order, max_absumv);
            assert_eq!(seq_id, SequenceID::HamCycle);
        }
    }
}
