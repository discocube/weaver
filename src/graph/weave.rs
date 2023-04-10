use super::{defs::*, ops::prelude::*};

/// 🕸️ Weave a Hamiltonian cycle by building chains level by level bottom up until subtours of half is formed. Mirror chains to form cycles for subsequent joining of weft with each warp in the loom until only the weft remains.\
///
///---\
/// `🧭 g`: GraphInfo instance used to get information about the graph.\
/// `🧵 spun`: Spool of yarn to be colored.\
/// `🧶 yarns`: Blue and red yarn as an ndarray.\
/// `📌 pins`: Pins are used cut the finished yarn and as markers to connect the current level to the previous.\
/// `🪜 loom`: Container onto which threads are woven level by level using pins as markers to connect the previous level to the current.\
/// `🪡 weft`: Leading loop into which the warps are incorporated. Weft object containing the solution.\
/// `🧮 warps`: Threads which are built horizontally upwards level by level until half the graph has been built. \
/// ---\
///
///```
///pub fn weave(n: usize) -> Solution {
///
///    // Init pin cushion with capacity provided by g.
///    let mut pins = PinCushion::with_capacity(n);
///
///    // Convert to ndarray & assign to blue. Copy/reflect/translate blue & assign to red.
///    let yarns = Yarns::colorized(Spindle::spin(n.spool_size()));
///
///    // Iterate over the setting for each level: `zrow`, `size`, `color`
///    n.zrow_color_size().iter().for_each(|&((z, color), size)| {
///
///        // Extend each thread end w/ finished yarn. Use pins for cutting.
///        loom.extend_threads(yarns.finish(z, color, size, &pins));
///
///        // Prepare pins for next level. Stop pinning when zrow is the last.
///        pins = loom.mark_next_ends(z);
///    });
///
///    // Mirror loom's threads to form subcycles from subchains for subsequent merging.
///    loom.mirror_threads();
///
///    // Split weft from the loom leaving only the warps.
///    let (mut weft, mut loom) = loom.prepare_merging(n);
///
///    // Iterate over each warp in the loom and incorporate into the weft.
///    loom.iter_mut().for_each(|warp| {
///
///        // Get edges which are also valid bridges
///        let warp_edges = warp.edges(weft.max_sum_z, weft.joined);
///
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
///    }
///```
/// ---\
///
/// Spin and color yarn. From the bottom-up for each level: cut the yarn incorporate into the level using bobbins if necessary to affix to the previous threads. Prepare bobbins for the next level. When we've reached the top, reflect the loom. Separate the loom into a main weft and warps. Incorporate the weft into the warps. Return solution.\
/// For each level, get the requested color and cut. If there are pins in the pin pins, cut accordingly and finish.\
/// Merge subcycles by first calculating the bridge between warp's and weft's edges: Align each sequence to their respective edge such that the two sequences can be placed next to another. Append the warp to the weaver's weft. Continue to incorporate warps into the weft until only the weft remains.
///
/// I've also abstracted away the
pub fn weave(n: usize) -> Solution {
    let mut pins = PinCushion::with_capacity(n);
    let mut loom = Loom::with_capacity(n.loom_size());
    let yarns = Yarns::colorized(Spindle::spin(n.spool_size()));
    n.zrow_color_size().iter().for_each(|&((z, color), size)| {
        loom.extend_threads(yarns.finish(z, color, size, &pins));
        pins = loom.mark_next_ends(z);
    });
    loom.mirror_threads();
    let (mut weft, mut loom) = loom.prepare_merging(n);
    loom.iter_mut().for_each(|warp| {
        let warp_edges = warp.edges(weft.max_sum_z, weft.joined);
        let weft_bridge = weft.edges().bridge(&warp_edges);
        weft.data.align_to(weft_bridge);
        warp.align_to(warp_edges.bridge(&weft_bridge));
        weft.join(warp);
    });
    weft.get_woven()
}
