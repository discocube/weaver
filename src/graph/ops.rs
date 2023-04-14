/// 🎶 Collection of modules used to build Hamiltonian cycle.
pub mod prelude {
    pub use super::super::types::*;
    pub use super::{
        certify_solution::*, color_yarn::ColorSpunYarn, extend_threads::ExtendLoomThreads,
        graph_info_from_n::InfoN, merge_cycles::*, mirror_loom::MirrorLoomThreads,
        pin_threads::PinThreadEnds, prepare_yarn::PrepYarnExtensions, prepare_yarn::SegmentYarn,
        spin_yarn::Spin,
    };
}

/// ℹ️ Given n calculate values used for building the solution. What happens if you shrink the representation of a graph to almost nothing for the sake of saving memory.
pub mod graph_info_from_n {
    use crate::graph::types::*;
    use itertools::Itertools;
    use std::iter::{repeat, zip};

    /// Given n, get required calculations/settings needed for the weave.
    pub trait InfoN {
        /// Retrieve loom size based on n.
        fn loom_size(self) -> Count;
        /// Get the maximum L1-norm for n.
        fn get_max_absumv(self) -> ScalarXyz;
        /// Get maximum scalar value allowed for x, y or z.
        fn get_max_xyz(self) -> ScalarXyz;
        /// Get maximum scalar value allowed for x, y or z where n is the order.
        fn get_max_xyz_from_order(self) -> SignedIdx;
        /// Minimum scalar value allowed for x, y, or z.
        fn get_min_xyz(self) -> ScalarXyz;
        /// get level from order, or n.
        fn get_n_from_order(self) -> Count;
        /// get order from level/n.
        fn get_order_from_n(self) -> Count;
        /// spool size is the number of verts in the graph whose z-scalar value is -1.
        fn spool_size(self) -> (Count, Count);
        /// Get length of the current level.
        fn zrow_color_idx(self) -> ZrowColorSize;
    }

    impl InfoN for Count {
        fn loom_size(self) -> Count {
            (self / 2) + 1
        }
        fn get_max_absumv(self) -> ScalarXyz {
            (self * 2 + 1) as ScalarXyz
        }

        fn get_max_xyz(self) -> ScalarXyz {
            (self * 2 - 1) as ScalarXyz
        }

        fn get_max_xyz_from_order(self) -> SignedIdx {
            (((3 * self) / 4) as f64).powf(1.0 / 3.0) as i32 * 2 - 1
        }

        fn get_min_xyz(self) -> ScalarXyz {
            (self * 2 - 5) as ScalarXyz
        }

        fn get_n_from_order(self) -> Count {
            (((3 * self) / 4) as f64).powf(1.0 / 3.0) as Count
        }

        fn get_order_from_n(self) -> Count {
            (4 * (self + 2) * (self + 1) * self) / 3
        }

        fn spool_size(self) -> (Count, Count) {
            (self, 2 * self * (self + 1))
        }

        fn zrow_color_idx(self) -> ZrowColorSize {
            let spool_length = 2 * self * (self + 1);
            zip(
                zip(
                    (-((self * 2 - 1) as i16)..=-1).step_by(2),
                    match self % 2 {
                        0 => repeat(1).take(self).interleave(repeat(3).take(self)),
                        _ => repeat(3).take(self).interleave(repeat(1).take(self)),
                    },
                ),
                (1..=self).map(|_n| spool_length - (2 * _n * (_n + 1))),
            )
            .collect()
        }
    }
}

/// 🛞 Spin a zigzagging-inward-spiralling Hamiltonian chain from the outer to innermost vert where the scalar z value of the points equals -1.
mod spin_yarn {
    use super::graph_info_from_n::InfoN;
    use crate::graph::types::*;
    use std;

    /// 2d displacement vectors for walking a zig-zag.
    pub const DISP_VECTORS: [[V2d; 2]; 4] = [
        [[-2, 0], [0, -2]],
        [[-2, 0], [0, 2]],
        [[2, 0], [0, 2]],
        [[2, 0], [0, -2]],
    ];

    /// 🪀 Spin a zigzagging-inward-spiralling Hamiltonian chain.
    pub trait Spin {
        /// 🪀 Spin a zigzagging-inward-spiralling Hamiltonian chain.
        ///
        ///---\
        /// `📏 spool_size`: Spool length and longest hamiltonian chain consisting only of points whose scalar z-value is -1.\
        /// `🧵 spool`: Container upon which the yarn is spun or the tour recorded.\
        /// `🛞 turns`: Indices indicating when to get `next(pair of xy-zigzag displacement vectors)`.\
        /// `🎚️ max_xyz`:  Max scalar value for x, y, z used as the first step and to seed the array: `[max_xyz, 1]`.\
        /// `🔀 switch`:  an infinite switch iterator of indices [[1, 0]] of the y and x axis of the displacement vector.\
        ///---\
        /// Pre-populate the `spool` with start the vector `[max_yxz, 1]` * `spool_size` to avoid reallocation of space. Take steps by looping over the pre-populated elements of `spool` starting from the first index `spool[0]` and calculating the next step by adding either an x or y (switching at each step) displacement vector to the current vector `spool[idx].add_vec(zigzag[y_or_x])` and taking that step by assigning the result to the next index `spool[idx + 1]`. When it's time to turn `turn == idx`, get the new set of zig-zag vectors to change direction (turning inward) and take a step: add that new x or y vector to the current vector and assign to the next index. Each step alternates between x and y.
        ///
        ///
        fn spin(n_spool_size: (usize, usize)) -> Spindle;
    }

    impl Spin for Spindle {
        fn spin((n, spool_size): (usize, usize)) -> Spindle {
            let mut spool = vec![[n.get_max_xyz(), 1]; spool_size];
            let (mut spinner, (mut iturn, mut zigzag)) = Spinner::start(n);
            (0..spool_size - 1).for_each(|idx| {
                let yx = spinner.yxyx.switch();
                if iturn == idx {
                    (iturn, zigzag) = spinner.turn(yx);
                };
                spool[idx + 1] = spool[idx].add(zigzag[yx])
            });
            spool
        }
    }

    /// A struct responsible for providing the right set of displacement vectors so that from an initial vector, an inward-turning zigzag is produced
    pub struct Spinner<'a> {
        /// Value of n.
        pub n: Count,
        /// A list of indices indicating when the displacement vectors should be replaced with a new set.
        pub turns: Counts,
        /// An infinite iterator of displacement vectors.
        pub zigzags: std::iter::Cycle<core::slice::Iter<'static, [V2d; 2]>>,
        /// An infinite iterator of alternating 1 and 0 which indexes to either a y-axis displacement vector or x.
        pub yxyx: std::iter::Cycle<core::slice::Iter<'a, usize>>,
    }

    impl<'a> Spinner<'a> {
        pub fn start(n: Count) -> (Self, (Count, &'static [V2d; 2])) {
            let mut turns = Spinner::get_turns(n);
            let mut zigzags = Spinner::get_zigzags();
            let turn = turns.pop().unwrap();
            let zigzag = zigzags.next().unwrap();
            let spinner = Spinner {
                n,
                turns,
                zigzags,
                yxyx: [1, 0].iter().cycle(),
            };
            (spinner, (turn, zigzag))
        }

        pub fn turn(&mut self, iyx: Count) -> (Count, &'static [V2d; 2]) {
            (
                self.turns.pop().unwrap() - iyx,
                self.zigzags.next().unwrap(),
            )
        }

        fn get_turns(n: Count) -> Counts {
            let mut result = (-(n as i64 * 2)..=-2)
                .step_by(2)
                .flat_map(|turn| [-turn as usize; 2])
                .scan(0, |state, n| {
                    *state += n;
                    Some(*state - 1)
                })
                .collect::<Counts>();
            result.reverse();
            result
        }

        fn get_zigzags() -> std::iter::Cycle<core::slice::Iter<'static, [V2d; 2]>> {
            DISP_VECTORS.iter().cycle()
        }
    }

    /// Conveniece trait for an infinite iterator that doesn't actually need unwrapping, so we'll hide it.
    pub trait Switch<T> {
        /// Convenience fn to avoid the verbosity of self.next().unwrap() at least for an infinite iterator, where it shouldn't be necessary, so we'll hide it.
        fn switch(&mut self) -> T;
    }

    impl<'a, T: Clone + Copy> Switch<T> for std::iter::Cycle<std::slice::Iter<'a, T>> {
        fn switch(&mut self) -> T {
            *self.next().unwrap()
        }
    }

    /// Add another vector to self.
    pub trait AddVec {
        // add two 2-dimensional vectors together.
        fn add(&self, other: V2d) -> V2d;
    }

    impl AddVec for V2d {
        fn add(&self, [a, b]: V2d) -> V2d {
            [self[0] + a, self[1] + b]
        }
    }
}

/// 🌈 Convert spun yarn to a 2-dimensional ndarray. Assign to blue. Copy, reflect and translate blue, assign to red.
mod color_yarn {
    use ndarray::array;

    use crate::graph::types::*;

    pub trait ColorSpunYarn {
        /// 🎨 Color by draining spool into an ndarray & assigning to `blue`. Assign `red` as a mirrored/translated `blue`.
        ///
        ///---\
        /// `🧵 spool`: raw spun yarn to be colored.\
        /// `🔵 blue`: spool as a 2d-array, where each item is a vector of two scalar x and y values.\
        /// `🔴 red`: blue yarn cloned but reflected and shifted one unit in the y+ direction.\
        /// ---\
        ///
        /// The last elevation where the z-value is -1 is always blue and going down, alternates between red, and blue.. Where n = 3, a graph with 80 vertices, there are 3 levels: -5, -3, -1. Each level has a color so in the case of graph_80: -5 ⇒ blue, -3 ⇒ red, -1 ⇒ blue.
        ///
        ///
        fn color_spun(spool: Spindle) -> Yarns;
    }

    impl ColorSpunYarn for Yarns {
        fn color_spun(mut spool: Spindle) -> Yarns {
            let blue = Yarn::from(spool.drain(..).collect::<Spindle>());
            let red = blue.dot(&array![[-1, 0], [0, -1]]) + array![[0, 2]];
            Yarns::from([(3, blue), (1, red)])
        }
    }
}

/// 📌 Mark ends with a pin from which to extend the prepared yarn. Each pin is a point constructed from each end of each thread in the loom where there an edge length is added to the z-scalar value of each end: thread_end [x, y, z] -> pin [x, y, z + 2]
pub mod pin_threads {
    use crate::graph::types::*;

    /// 📌 Pins are used to carry over the values of each end of each thread in the loom from the previous level to the next. For each thread end `[thread[0], thread[thread.len() - 1]]` in the loom make a pin by adding 2 (length of an edge) to the z-scalar value: `[x, y, z + 2]`. Collect the pins in the cushion for cutting later.
    pub trait PinThreadEnds {
        /// 📌 Pins are used to carry over the values of each end of each thread in the loom from the previous level to the next. For each thread end `[thread[0], thread[thread.len() - 1]]` in the loom make a pin by adding 2 (length of an edge) to the z-scalar value: `[x, y, z + 2]`. Collect the pins in the cushion for cutting later.
        ///
        ///---\
        /// `🪜 loom`: Container on which the threads are extended level by levels using pins as markers to connect the levels.\
        /// `🔚 last`: Indicates when we are in the last leg of the loop in which it is no longer required to pin the next level\
        /// ---\
        ///
        /// For each thread end in the loom, add a pin corresponding to the V that is adjacent to that end but one z-level up. Collect the a copy of those inserted pins for cutting the yarn later. The purpose of the pins is to connect the previous level to the next. They are always prepared at the end of level and used for the next. If we have a thread `[1, 2, 3, 4, 5]` and let's say the pin above is 0 for 1 and 6 for 5. We would attach the pins to the ends:
        /// `[0, 1, 2, 3, 4, 5, 6]` and add those pins to the cushion: `[0, 6]` which is later used to cut the sequence from the next level so it can be attached to this thread. let's say we have the yarn for the next level that looks like this: `[0, 10, 20, 30, 40, 6, 16, 26]` we would get the pins from the previous level: `[0, 6]` and cut the finished yarn accordingly such that there is at least two verts in a cut: `[0, 6] ✂️ [0, 10, 20, 30, 40, 6, 16, 26]` would produce `[0, 10, 20, 30, 40] & [6, 16, 26]`: now add that to the thread where the pins match:  `[40, 30, 20, 10, 0][0, 1, 2, 3, 4, 5, 6]+[6, 16, 26]`
        /// resulting in: `[40, 30, 20, 10, 0, 1, 2, 3, 4, 5, 6, 16, 26]`
        /// Insert pins into each end of each thread in the loom. A pin is the vertex adjacent to and directly above an end. Collect the a copy of all inserted pins to be used for cutting the finished yarn from the next level up.
        ///
        fn pin_thread_ends(&mut self) -> PinCushion;
    }

    impl PinThreadEnds for Loom {
        fn pin_thread_ends(&mut self) -> PinCushion {
            self.iter_mut()
                .flat_map(|end| {
                    let [[x, y, z], [i, j, k]] = [end[0], end[end.len() - 1]];
                    let [front, back] = [[x, y, z + 2], [i, j, k + 2]];
                    end.push_front(front);
                    end.push_back(back);
                    [front, back]
                })
                .collect()
        }
    }
}

/// 👨‍🍳 Prepare yarn for extending onto the loom threads. Cut using pins and affix yarn to the current elevation.
pub mod prepare_yarn {
    use crate::graph::types::*;
    use itertools::Itertools;
    use ndarray::s;

    /// 👨‍🍳 Prepare yarn to be extennded onto the individual thread ends.
    pub trait PrepYarnExtensions {
        /// 👨‍🍳 Measure requested color and size as a slice and finish by positioning the yarn to the current elevation by adding a scalar `zpos` to each item in the slice and cutting finished yarn if there are pins in the pins by calling `cut_yarn(_yarn, pins)` or return uncut.
        ///
        ///---\
        /// `🇿 zpos`: Current elevation.\
        /// `🎨 color`: Color for current elevation.\
        /// `📏 size`: Size of yarn to be cut from yarns.\
        /// `📌 pins`: Contains the pins used as markers for cutting yarn.\
        /// `🧶 yarns`: Contains a blue and a red yarn which is accessed by `🎨 color`.\
        ///---\
        /// Get the requested `color`  🔵 or 🔴 from `yarns` and measure requested `size` using `slice(s![size.., ..])`.\
        /// Finish yarn: iterate over each 2d vector & affix to current elevation by appending `zpos`: `[row[0], row[1], zpos]`.\
        /// Cut finished yarn if `pins` is not empty, `cut_yarn(_yarn, &pins)`. Return the finished (cut or not) yarn.
        ///
        ///
        fn prep(&self, zpos: i16, color: u8, len: usize) -> Warp;
    }

    impl PrepYarnExtensions for Yarns {
        fn prep(&self, zpos: i16, color: u8, start_idx: usize) -> Warp {
            self[&color]
                .slice(s![start_idx.., ..])
                .outer_iter()
                .map(|row| [row[0], row[1], zpos])
                .collect_vec()
        }
    }

    /// 🔪 Cut yarn using pins from the pins as cut markers so it can be extended upon the individual threads in the loom.
    pub trait SegmentYarn {
        /// 🔪 Cut yarn using pins from the pins as cut markers.
        ///
        /// ---\
        /// `🎉 warps`: yarn that is cut and prepared to be incorporated.\
        /// `🎨 last_iyarn`: index of last item in yarn.\
        /// `📏 last_ipin`: index of last item in the pin_pins.\
        /// `🖇 i`: index of the previous iyarn.\
        /// `🧶 ipin`: index of the item in the pins.\
        /// `🧶 iyarn`: index of the item in the yarn.\
        /// ---\
        ///
        /// Because there's no previous in the beginning there's no pins to use for cutting the yarn.
        /// Before cutting add the z-value to the 2-dimensional vector, thereby making it 3d and positioning the yarn to the current level on the z-axis.
        ///
        ///
        fn split(self, pins: &PinCushion) -> Warps;
    }

    impl SegmentYarn for Warp {
        fn split(self, pins: &PinCushion) -> Warps {
            match !pins.is_empty() {
                true => {
                    let mut warps = Warps::with_capacity(pins.len() + 1);
                    let last_iyarn = self.len() - 1;
                    let last_ipin = pins.len() - 1;
                    let mut iprev = 0_usize;
                    self.iter()
                        .enumerate()
                        .filter_map(|(idx, point)| (pins.contains(point)).then_some(idx))
                        .enumerate()
                        .for_each(|(ipin, iyarn)| {
                            if ipin == last_ipin && iyarn != last_iyarn {
                                warps.push(self[iyarn..].to_vec());
                                if let Some(s) = self.get(iprev..iyarn).filter(|s| !s.is_empty()) {
                                    warps.push(s.iter().rev().cloned().collect())
                                };
                            } else {
                                warps.push((self[iprev..=iyarn]).iter().rev().cloned().collect());
                                iprev = iyarn + 1;
                            }
                        });
                    warps
                }
                false => vec![self],
            }
        }
    }
}

/// 🧮 Extend each thread in the loom based on the marked ends from the previous level.
mod extend_threads {
    use crate::graph::types::*;

    /// Extend each end of each thread in the loom with the segmented, colored and finished yarn.
    pub trait ExtendLoomThreads {
        /// Extend each end of each thread in the loom with the segmented, colored and finished yarn.
        fn extend_threads(&mut self, warps: Warps);
    }

    impl ExtendLoomThreads for Loom {
        fn extend_threads(&mut self, mut warps: Warps) {
            self.iter_mut().for_each(|thread: &mut LoomThread| {
                for warp in warps.iter_mut().filter(|w| !w.is_empty()) {
                    match (thread[0] == warp[0], thread[thread.len() - 1] == warp[0]) {
                        (true, _) => {
                            thread.pop_front();
                            warp.drain(..).for_each(|item| thread.push_front(item));
                        }
                        (_, true) => {
                            thread.pop_back();
                            thread.extend(warp.drain(..));
                        }
                        _ => continue,
                    }
                }
            });
            warps.iter_mut().filter(|s| !s.is_empty()).for_each(|seq| {
                self.push(seq.drain(..).collect::<LoomThread>());
            });
        }
    }
}

/// 🪞 Reflect the half-solution along the z-axis to create the whole.
mod mirror_loom {
    use crate::graph::types::{Loom, Tour};
    use rayon::prelude::*;

    /// For each thread in the loom all of whose ends are not adjacent, reflect each thread turning chains into cycles.
    /// Imagine reflecting a row of arcs to form a row of ovals.
    pub trait MirrorLoomThreads {
        /// For each thread in the loom all of whose ends are not adjacent, reflect each thread turning chains into cycles.
        /// Imagine reflecting a row of arcs to form a row of ovals.
        fn mirror_threads(&mut self);
    }

    impl MirrorLoomThreads for Loom {
        fn mirror_threads(&mut self) {
            self.par_iter_mut().for_each(|thread| {
                thread.extend(
                    thread
                        .iter()
                        .rev()
                        .map(|&[x, y, z]| [x, y, -z])
                        .collect::<Tour>(),
                )
            });
        }
    }
}

/// 🪢 Merge subcycles into one Hamiltonian cycle by finding their bridges through set intersection.
mod merge_cycles {
    use super::graph_info_from_n::InfoN;
    use crate::graph::types::*;
    use itertools::Itertools;
    use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

    /// Trait responsible for removing the subloops from the Loom and converting each thread from a VecDeque to a Vec and isolating the Weft and the main cycle into which Warps are incorporated.
    pub trait GetWeftWarps {
        /// Remove the threads from the loom and separate into weft and warps. Prepare for cycle merging. Loop over each warp, join with the weft until the end of the loop where there will only be the weft.
        fn prepare_cycle_merging(self, n: usize) -> (Weft, Warps);
    }

    impl GetWeftWarps for Loom {
        fn prepare_cycle_merging(mut self, n: usize) -> (Weft, Warps) {
            (
                Weft::new(self[0].split_off(0), n.get_order_from_n()),
                self.split_off(1)
                    .into_iter()
                    .map(|mut data| data.drain(..).collect())
                    .collect(),
            )
        }
    }

    /// Weft is the main loop into which warps are incorporated.
    #[derive(Clone, Debug)]
    pub struct Weft {
        pub data: Tour,
        pub joined: bool,
        pub max_abs_z: i16,
        pub max_sum_z: i16,
    }

    impl Weft {
        /// Create a new instance of weft which contains methods that are slightly different from the methods of warps.
        /// Weft is the main loop into which the warps are joined.
        pub fn new(mut data: LoomThread, order: Count) -> Weft {
            let mut preallocated = Warp::with_capacity(order);
            preallocated.extend(data.drain(..));
            let max_abs_z = (order.get_max_xyz_from_order() - 4) as i16;
            Weft {
                data: preallocated,
                joined: false,
                max_abs_z,
                max_sum_z: max_abs_z * 2,
            }
        }

        /// Convert data into set of edges using `tuple_windows()` and filter using condition below resulting at most two edges.
        /// We are looking for only one edge so we use find to stop iterating over self.data once that one edge has been found.
        pub fn edges(&mut self) -> Edges {
            self.data
                .iter()
                .tuple_windows()
                .find(|(&[x, _, z], &[a, _, c])| {
                    [x, a] == (if self.joined { [1, 1] } else { [1, 3] })
                        && (z + c).abs() == self.max_sum_z
                })
                .map(|(m, n)| Edges::from([(*m, *n).orient()]))
                .unwrap()
        }

        /// Join the warp with the weft. The bridges change once the warp has been joined once.
        /// Append the warp to the end of the rotated weft.
        /// Once joined a different set of edges to used as bridges is valid.
        /// As cycles are joined level by level we can calculate the predicted location/elevation of the bridge edge
        /// using self.max_sum_z which is incremented each time a sequence is joined.
        pub fn join(&mut self, warp: &mut Warp) {
            self.data.append(warp);
            match self.joined {
                true => self.max_abs_z -= 4,
                false => {
                    self.joined = true;
                    self.max_abs_z -= 2;
                }
            }
            self.max_sum_z = self.max_abs_z * 2 - 2;
        }

        /// Retrieve the finished solution.
        pub fn get_woven(&self) -> Solution {
            self.data.to_vec()
        }
    }

    /// Get a tuple_window of self but filtered so that only the required edges are pass through the filter.
    pub trait GetWarpEdges {
        /// Construct edges from the Vec and filter.
        /// 'vec![1, 2, 3]' -> `hash_set![(1, 2), (2, 3), (3, 1)]` which is then filtered further to avoid memory waste.
        fn edges(&self, joined: bool) -> Edges;
    }

    impl GetWarpEdges for Warp {
        fn edges(&self, joined: bool) -> Edges {
            // The needed edge is located after the first third so let's start there.
            self[(self.len() / 3)..]
                .iter()
                .tuple_windows()
                .filter_map(|(&[x, y, z], &[a, b, c])| {
                    ([b, x, y] == (if joined { [1, 3, 1] } else { [3, 1, 3] }))
                        .then_some(([x, y, z], [a, b, c]).orient())
                })
                .collect()
        }
    }

    /// Orient edge such that LHS < RHS.
    pub trait OrientAscending {
        /// Orient edge such that lhs < rhs.
        fn orient(self) -> Edge;
    }

    impl OrientAscending for Edge {
        fn orient(self) -> Edge {
            match (self.0, self.1) {
                (m, n) if m < n => (m, n),
                (m, n) => (n, m),
            }
        }
    }

    /// Get the bridge edge between self and other.
    pub trait Bridge<T> {
        /// Using the & set operator, find the common bridge i.e., intersection between a set of edges and a set of adjacent edges and return the next() from the set.
        fn bridge(&self, other: &T) -> Edge;
    }

    impl Bridge<WeftEdge> for WarpEdges {
        /// Using the & set operator, find the common bridge i.e., intersection between a set of edges and a set of adjacent edges and return the next() from the set.
        /// This version automatically reverses the edge as it is always the case (removed the check).
        fn bridge(&self, (weft_lhs, weft_rhs): &WeftEdge) -> BridgeEdge {
            let (warp_lhs, warp_rhs) = (&((*weft_lhs, *weft_rhs).eadjs()) & self)
                .into_iter()
                .next()
                .unwrap();
            (warp_rhs, warp_lhs)
        }
    }

    impl Bridge<WarpEdges> for WeftEdges {
        fn bridge(&self, other: &WarpEdges) -> BridgeEdge {
            (self & &other.eadjs()).into_iter().next().unwrap()
        }
    }

    /// Get the adjacent/parallel edge of self either for WarpEdges or for WeftEdge
    pub trait GetEadjs {
        /// Get the adjacent/parallel edges of edges.
        fn eadjs(&self) -> Edges;
    }

    impl GetEadjs for WarpEdges {
        fn eadjs(&self) -> Edges {
            self.par_iter()
                .filter_map(|&([a, b, c], [x, y, z])| {
                    ((x == 3 || x == 1) && (y == 3 || y == 1)).then_some(
                        match (a != x, b != y, c != z) {
                            (true, false, false) => ([a, b - 2, c], [x, y - 2, z]),
                            (false, true, false) => ([a, b, c + 2], [x, y, z + 2]),
                            (false, false, true) => ([a - 2, b, c], [x - 2, y, z]),
                            _ => panic!("NOT A VALID EDGE"),
                        },
                    )
                })
                .collect()
        }
    }

    impl GetEadjs for WeftEdge {
        fn eadjs(&self) -> Edges {
            let ([a, b, c], [x, y, z]) = *self;
            match (a != x, b != y, c != z) {
                (true, false, false) => [([a, b + 2, c], [x, y + 2, z])].into(),
                (false, true, false) => [([a + 2, b, c], [x + 2, y, z])].into(),
                (false, false, true) => [([a + 2, b, c], [x + 2, y, z])].into(),
                _ => panic!("NOT A VALID EDGE"),
            }
        }
    }

    /// Align self to edge such that self.lhs == edge.lhs and self.rhs == edge.rhs
    pub trait AlignToEdge<T: PartialEq + Copy> {
        /// Align self to given edge such that the lhs of edge and self match and the rhs of edge and self match.
        /// Only the cases that occur in the code are covered. It is not to be used generally as not all cases are covered.
        fn align_to(&mut self, edge: (T, T));
    }

    impl<T: PartialEq + Copy> AlignToEdge<T> for Vec<T> {
        fn align_to(&mut self, (lhs, rhs): (T, T)) {
            match (
                self.iter().position(|&x| x == lhs).unwrap(),
                self.iter().position(|&x| x == rhs).unwrap(),
            ) {
                (idx_lhs, idx_rhs) if idx_lhs < idx_rhs => {
                    self.rotate_left(idx_rhs);
                    self.reverse()
                }
                (idx_lhs, _) => self.rotate_left(idx_lhs),
            }
        }
    }
}

/// ✅ Certify if the solution is Hamiltonian.
pub mod certify_solution {
    use crate::graph::types::{Solution, V2d, V3d};
    use itertools::{all, Itertools};

    use std::fmt;

    #[derive(Debug, PartialEq)]
    /// Enum describing possible ids of a solution: Broken, Chain, Cycle.
    pub enum SequenceID {
        Broken,
        HamChain,
        HamCycle,
    }

    /// impl Display to print out SequenceID w/o debug.
    impl fmt::Display for SequenceID {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SequenceID::Broken => write!(f, "Broken"),
                SequenceID::HamChain => write!(f, "HamChain"),
                SequenceID::HamCycle => write!(f, "HamCycle"),
            }
        }
    }

    /// Certify if a sequence is a Hamiltonian cycle by:
    /// Checking for duplicates
    /// Check that solution length is equal to the order of the graph.
    /// The sum of all displacement vectors used to construct the cycle is equal to [0, 0, 0]
    /// Check that the current node is adjacent to the next node.
    /// Check that the last node is adjacent to the first. If not is is a HamChain else HamCycle.
    pub trait Certify<SequenceID> {
        /// Certify if a sequence is a Hamiltonian cycle by:
        /// Checking for duplicates
        /// Check that solution length is equal to the order of the graph.
        /// The sum of all displacement vectors used to construct the cycle is equal to [0, 0, 0]
        /// Check that the current node is adjacent to the next node.
        /// Check that the last node is adjacent to the first. If not is is a HamChain else HamCycle.
        fn certify(&self, order: usize, max_absumv3d: i16) -> SequenceID;
    }

    impl Certify<SequenceID> for Solution {
        fn certify(&self, order: usize, max_absumv3d: i16) -> SequenceID {
            if self.iter().duplicates().count() > 0
                || self.len() != order
                || !all(self.iter(), |vert| vert.l1norm() <= max_absumv3d)
                || self
                    .iter()
                    .fold((0, 0, 0), |acc: (i64, i64, i64), &[x, y, z]| {
                        (acc.0 + x as i64, acc.1 + y as i64, acc.2 + z as i64)
                    })
                    != (0, 0, 0)
            {
                return SequenceID::Broken;
            }
            match self.windows(2).all(|window| window[0].is_adj_to(window[1])) {
                true if self[self.len() - 1].is_adj_to(self[0]) => SequenceID::HamCycle,
                true => SequenceID::HamChain,
                false => SequenceID::Broken,
            }
        }
    }

    /// Calculate the L1-norm for a vector, i.e., the sum of the absolute value of each scalar x, y and or z.
    /// Also known as the L1-Norm- manhattan distance from a to b where b is the origin (0, 0, 0)
    /// The L1 norm is calculated as the sum of the absolute vector values, where the absolute value of a scalar uses the notation |a1|.
    /// In effect, the norm is a calculation of the Manhattan distance from the origin of the vector space.
    pub trait L1Norm {
        /// Calculate the L1-norm for a vector, i.e., the sum of the absolute value of each scalar x, y and or z.
        fn l1norm(&self) -> i16;
    }

    impl L1Norm for V3d {
        fn l1norm(&self) -> i16 {
            self.iter()
                .map(|v| {
                    let mask = v >> 15;
                    (v ^ mask) - mask
                })
                .sum()
        }
    }

    impl L1Norm for V2d {
        fn l1norm(&self) -> i16 {
            self.iter()
                .map(|v| {
                    let mask = v >> 15;
                    (v ^ mask) - mask
                })
                .sum()
        }
    }

    /// Test that input vector is adjacent to self where the points are points in a 3d grid.
    pub trait IsAdjacent {
        /// Check if self is adjacent to another vertex.
        fn is_adj_to(&self, other: V3d) -> bool;
    }

    impl IsAdjacent for V3d {
        fn is_adj_to(&self, [x, y, z]: V3d) -> bool {
            let [a, b, c] = self;
            let n = a - x + b - y + c - z;
            n == 2 || n == -2
        }
    }
}

/// 📤 Module for exporting the solution to a .csv file where each row is x, y, z.
pub mod serialize_csv {
    use crate::graph::types::Solution;
    use serde::Serialize;
    use std::error::Error;

    /// Simple 3-dimensional vector struct for serializing points to csv.
    #[derive(Debug, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub(crate) struct CsvVector {
        pub(crate) x: i16,
        pub(crate) y: i16,
        pub(crate) z: i16,
    }

    /// Save solution to `file_path` as a `.csv` file with the columns headers `x`, `y`, `z` representing the scalar value for each axis.
    pub trait SerializeToCsv<T> {
        /// Save solution to `file_path` as a `.csv` file with the columns `x`, `y`, `z` for each axis.\
        /// A python module using `pandas` and `plotly` to create a 3d line plot is available [here](https://github.com/discocube/plot_solution).
        /// ```
        /// let solution = weave(2);
        /// solution.serialize_to_csv("documents/solutions/csv/solution_2.csv")
        /// ```
        fn serialize_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
    }

    impl SerializeToCsv<Result<(), Box<dyn Error>>> for Solution {
        fn serialize_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
            let file = std::fs::File::create(file_path)?;
            let mut writer = csv::Writer::from_writer(file);
            self.iter().for_each(|[x, y, z]| {
                writer
                    .serialize(CsvVector {
                        x: *x,
                        y: *y,
                        z: *z,
                    })
                    .ok();
            });
            let [x, y, z] = self[0];
            writer.serialize(CsvVector { x, y, z })?;
            writer.flush()?;
            Ok(())
        }
    }
}

///! TESTS
///!
///!
///!
///!
/// 🩺 Test overflow and expected outputs for n.get().
#[cfg(test)]
mod tests_graph_info_from_n {
    use super::prelude::*;

    #[test]
    /// Test behavior if input is max_i16
    fn test_info_n_with_max_i16() {
        let max_i16 = std::i16::MAX as Count;
        assert_eq!(max_i16.loom_size() as i16, 16384);
        assert_eq!(max_i16.get_max_absumv() as i16, -1);
        assert_eq!(max_i16.get_max_xyz(), -3);
        assert_eq!(max_i16.get_max_xyz_from_order(), 57);
        assert_eq!(max_i16.get_min_xyz(), max_i16.get_max_xyz() - 4);
        assert_eq!(max_i16.get_n_from_order(), 29);
        assert_eq!(max_i16.get_order_from_n(), 46912496074752);
        assert_eq!(max_i16.spool_size(), (32767, 2147418112));
    }

    #[test]
    /// Test behavior if input is max_i16
    fn test_info_n_pass() {
        let n = 100;
        assert_eq!(n.loom_size() as i16, 51);
        assert_eq!(n.get_max_absumv(), 201);
        assert_eq!(n.get_max_xyz(), 199);
        assert_eq!((n + 60).get_max_xyz_from_order(), 7);
        assert_eq!(n.get_min_xyz(), n.get_max_xyz() - 4);
        assert_eq!((n - 20).get_n_from_order(), 3);
        assert_eq!(n.get_order_from_n(), 1373600);
        assert_eq!(n.spool_size(), (n, 20200));
    }
}

/// 🩺 Test if result from `spin` is same as expected.
#[cfg(test)]
mod tests_spin_yarn {
    use super::{
        prelude::{InfoN, Spin, V2d},
        spin_yarn::{AddVec, Spinner, Switch},
    };

    #[test]
    /// Test the result of spin whose input is determined by a call to InfoN.
    /// Results provided are zigzag hamiltonian chains from outer to inner vert.
    fn test_spin() {
        // Spin cube.
        let spun = <Vec<V2d> as Spin>::spin(1_usize.spool_size());
        let expected = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
        assert_eq!(spun, expected);
        // Spin order 32 with 12 verts at zlevel -1.
        let spun = <Vec<V2d> as Spin>::spin(2_usize.spool_size());
        let expected = [
            [3, 1],
            [3, -1],
            [1, -1],
            [1, -3],
            [-1, -3],
            [-1, -1],
            [-3, -1],
            [-3, 1],
            [-1, 1],
            [-1, 3],
            [1, 3],
            [1, 1],
        ];
        assert_eq!(spun, expected);
        // Spin order 80 with 24 verts at zlevel -1.
        let spun = <Vec<V2d> as Spin>::spin(3_usize.spool_size());
        let expected = [
            [5, 1],
            [5, -1],
            [3, -1],
            [3, -3],
            [1, -3],
            [1, -5],
            [-1, -5],
            [-1, -3],
            [-3, -3],
            [-3, -1],
            [-5, -1],
            [-5, 1],
            [-3, 1],
            [-3, 3],
            [-1, 3],
            [-1, 5],
            [1, 5],
            [1, 3],
            [3, 3],
            [3, 1],
            [1, 1],
            [1, -1],
            [-1, -1],
            [-1, 1],
        ];
        assert_eq!(spun, expected);
    }

    #[test]
    /// Test that spinner gives the right zigzag that matches the right index.
    fn test_spinner() {
        let (n, spool_size) = 2_usize.spool_size();
        let (mut spinner, (mut iturn, mut zigzag)) = Spinner::start(n);
        (0..spool_size - 1).for_each(|idx| {
            let yx = spinner.yxyx.switch();
            if iturn == idx {
                match idx {
                    3 => assert_eq!(&[[-2, 0], [0, -2]], zigzag),
                    7 => assert_eq!(&[[-2, 0], [0, 2]], zigzag),
                    9 => assert_eq!(&[[2, 0], [0, 2]], zigzag),
                    11 => assert_eq!(&[[2, 0], [0, -2]], zigzag),
                    _ => panic!("Wrong index"),
                }
                (iturn, zigzag) = spinner.turn(yx);
            };
        });
    }

    #[test]
    /// Test that the switch gives alternating outputs of 1 and 0
    fn test_switch() {
        let mut to_cycle = [1, 0].iter().cycle();
        for _ in 0..1000 {
            assert_eq!(1, to_cycle.switch());
            assert_eq!(0, to_cycle.switch());
        }
        let mut to_cycle = ['a', 'b'].iter().cycle();
        for _ in 0..1000 {
            assert_eq!('a', to_cycle.switch());
            assert_eq!('b', to_cycle.switch());
        }
    }

    #[test]
    /// Test the add vec to self
    fn test_add_vec() {
        let result = [0, 1].add([1, 0]);
        assert_eq!(result, [1, 1]);
        let result = [-1, -1].add([1, 1]);
        assert_eq!(result, [0, 0]);
        let result = [-13, 11].add([100, 12]);
        assert_eq!(result, [87, 23]);
    }
}

/// 🩺 Test if colored yarn can be converted by and forth from blue to red and back.
#[cfg(test)]
mod tests_color_yarn {
    use super::color_yarn::ColorSpunYarn;
    use super::prelude::Yarns;
    use ndarray::array;

    #[test]
    /// Test that that we can go back to blue from red by reversing the operations resulting in reverting red back to blue.
    fn test_colorized() {
        let spool = vec![
            [5, 1],
            [5, -1],
            [3, -1],
            [3, -3],
            [1, -3],
            [1, -5],
            [-1, -5],
            [-1, -3],
            [-3, -3],
            [-3, -1],
            [-5, -1],
            [-5, 1],
            [-3, 1],
            [-3, 3],
            [-1, 3],
            [-1, 5],
            [1, 5],
            [1, 3],
            [3, 3],
            [3, 1],
            [1, 1],
            [1, -1],
            [-1, -1],
            [-1, 1],
        ];
        let colored_yarns = <Yarns as ColorSpunYarn>::color_spun(spool);
        let blue = &colored_yarns[&3].clone();
        let red = &colored_yarns[&1].clone();
        let new_shifted_blue = red + array![[0, -2]];
        let expect_blue = new_shifted_blue.dot(&array![[-1, 0], [0, -1]]);
        assert_eq!(blue, expect_blue);
    }
}

/// 🩺 Test `prepare_yarn` and `cut_yarn`
#[cfg(test)]
mod tests_prepare_yarn {
    use super::prelude::*;

    #[test]
    /// Test by getting the requested color and slice of that color and if it has mapped it to a Vec<[i16; 3]>.
    fn test_prepare_yarn() {
        let n = 3_usize;
        let pins = PinCushion::with_capacity(n);
        let yarns = Yarns::color_spun(Spindle::spin(n.spool_size()));
        let prepared = yarns.prep(-1, 3, 0).split(&pins);
        let expected = vec![vec![
            [5, 1, -1],
            [5, -1, -1],
            [3, -1, -1],
            [3, -3, -1],
            [1, -3, -1],
            [1, -5, -1],
            [-1, -5, -1],
            [-1, -3, -1],
            [-3, -3, -1],
            [-3, -1, -1],
            [-5, -1, -1],
            [-5, 1, -1],
            [-3, 1, -1],
            [-3, 3, -1],
            [-1, 3, -1],
            [-1, 5, -1],
            [1, 5, -1],
            [1, 3, -1],
            [3, 3, -1],
            [3, 1, -1],
            [1, 1, -1],
            [1, -1, -1],
            [-1, -1, -1],
            [-1, 1, -1],
        ]];
        assert_eq!(prepared, expected);
    }

    #[test]
    /// Test by cutting a sequence using pins.
    fn test_cut_using() {
        let pins = vec![[1, 1, -1], [-1, 1, -1]];
        let to_cut: Warp = vec![
            [3, 1, -1],
            [3, -1, -1],
            [1, -1, -1],
            [1, -3, -1],
            [-1, -3, -1],
            [-1, -1, -1],
            [-3, -1, -1],
            [-3, 1, -1],
            [-1, 1, -1],
            [-1, 3, -1],
            [1, 3, -1],
            [1, 1, -1],
        ];
        let warps = to_cut.split(&pins);
        let expected = vec![
            vec![
                [-1, 1, -1],
                [-3, 1, -1],
                [-3, -1, -1],
                [-1, -1, -1],
                [-1, -3, -1],
                [1, -3, -1],
                [1, -1, -1],
                [3, -1, -1],
                [3, 1, -1],
            ],
            vec![[1, 1, -1], [1, 3, -1], [-1, 3, -1]],
        ];
        assert_eq!(warps, expected);
    }
}

/// 🩺 Test mark ends by marking ends of a thread.
#[cfg(test)]
mod tests_pin_threads {
    use super::prelude::*;

    #[test]
    /// Test that the output from mark end where zrow == -1 should give me an empty pin cushion.
    fn test_last_row_empty_cushion() {
        let mut loom = Loom::with_capacity(2);
        assert_eq!(PinCushion::with_capacity(0), loom.pin_thread_ends());
    }

    #[test]
    /// Test that the output from mark end where zrow == -1 should give me an empty pin cushion.
    fn test_mark_end() {
        let mut thread = LoomThread::new();
        thread.push_back([0, 0, 0]);
        thread.push_back([2, 0, 0]);
        // Thread should now be [[0, 0, 0], [2, 0, 0]]
        let expected = [[0, 0, 2], [2, 0, 2]];
        let result = {
            let [[x, y, z], [i, j, k]] = [thread[0], thread[thread.len() - 1]];
            let [front, back] = [[x, y, z + 2], [i, j, k + 2]];
            thread.push_front(front);
            thread.push_back(back);
            [front, back]
        };

        assert_eq!(expected, result);
    }
}

/// 🩺 Test extend threads by constructing a loom extending its threads and checking if the result matches the expected output.
#[cfg(test)]
mod tests_extend_threads {
    use super::prelude::*;

    #[test]
    fn test_extend_threads() {
        let n = 2;
        let mut loom = Loom::with_capacity(n.loom_size());
        let yarns = Yarns::color_spun(Spindle::spin(n.spool_size()));
        loom.extend_threads(yarns.prep(-3, 1, 8).split(&vec![]));
        assert_eq!(loom, [[[1, 1, -3], [1, -1, -3], [-1, -1, -3], [-1, 1, -3]]]);
        loom.extend_threads(yarns.prep(-1, 3, 0).split(&vec![[1, 1, -1], [-1, 1, -1]]));
        assert_eq!(
            loom,
            vec![
                vec![[1, 1, -3], [1, -1, -3], [-1, -1, -3], [-1, 1, -3]],
                vec![
                    [-1, 1, -1],
                    [-3, 1, -1],
                    [-3, -1, -1],
                    [-1, -1, -1],
                    [-1, -3, -1],
                    [1, -3, -1],
                    [1, -1, -1],
                    [3, -1, -1],
                    [3, 1, -1]
                ],
                vec![[1, 1, -1], [1, 3, -1], [-1, 3, -1]]
            ]
        );
    }
}

/// 🩺 Test if input loom is properly mirrored.
#[cfg(test)]
mod tests_mirror_loom {
    use super::prelude::*;
    use std::collections::VecDeque;

    #[test]
    /// Create a loom and see if the it one thread is mirrored.
    fn test_mirror_threads() {
        let mut loom = Loom::with_capacity(1);
        let mut expected_loom = Loom::with_capacity(1);
        loom.push(VecDeque::from(vec![
            [1, 1, -1],
            [1, -1, -1],
            [-1, -1, -1],
            [-1, 1, -1],
        ]));
        expected_loom.push(VecDeque::from(vec![
            [1, 1, -1],
            [1, -1, -1],
            [-1, -1, -1],
            [-1, 1, -1],
            [-1, 1, 1],
            [-1, -1, 1],
            [1, -1, 1],
            [1, 1, 1],
        ]));
        loom.mirror_threads();
        assert_eq!(loom, expected_loom);
    }
}

/// 🩺 Test AlignToEdge.
#[cfg(test)]
mod tests_merge_cycles {
    use super::prelude::AlignToEdge;

    #[test]
    fn test_align_to() {
        let mut v = vec![0, 1, 2, 3, 4, 5];
        v.align_to((4, 3));
        assert_eq!(v, vec![4, 5, 0, 1, 2, 3]);
        v.align_to((0, 1));
        assert_eq!(v, vec![0, 5, 4, 3, 2, 1]);
        // Should give a different result as this case was removed as this doesn't occur in this algo.
        v.align_to((1, 0));
        assert_ne!(v, vec![1, 2, 3, 4, 5, 0]);
        assert_eq!(v, vec![1, 0, 5, 4, 3, 2]);
    }
}

/// 🩺 Test if the given sequences are broken.
#[cfg(test)]
mod tests_certify_solution {
    use super::prelude::*;
    #[test]
    fn test_certify_broken() {
        // not long enough.
        let mut sol: Solution = vec![[1, 1, -1], [1, -1, -1], [-1, -1, -1], [-1, 1, -1]];
        assert_eq!(SequenceID::Broken, sol.certify(8, 3));
        // too duplicates.
        sol = vec![[1, 1, -1], [1, -1, -1], [-1, -1, -1], [-1, -1, -1]];
        assert_eq!(SequenceID::Broken, sol.certify(8, 3));
        // right length but duplicates.
        sol = vec![
            [1, 1, -1],
            [-1, -1, -1],
            [-1, -1, -1],
            [-1, -1, -1],
            [-1, -1, -1],
            [-1, -1, -1],
            [1, -1, -1],
            [-1, -1, -1],
        ];
        assert_eq!(SequenceID::Broken, sol.certify(8, 3));
        // absumv too much.
        sol = vec![
            [1, 1, -7],
            [-1, -1, -1],
            [-1, -1, -9],
            [-7, -1, -1],
            [-1, -7, -1],
            [-1, -1, -1],
            [1, -9, -1],
            [-9, -1, -1],
        ];
        assert_eq!(SequenceID::Broken, sol.certify(8, 3));
    }
}

/// 🩺 Run weave to create solution and convert that solution to csv in root/test.csv, then check if file exists in specified location and then delete it.
#[cfg(test)]
mod tests_serialize_csv {
    use crate::graph::{ops::serialize_csv::SerializeToCsv, weave::weave};
    use std::{fs, path::Path};

    #[test]
    fn test_serialize_csvput() {
        let solution = weave(1);
        solution.serialize_to_csv("test.csv").unwrap();
        let path = Path::new("test.csv");
        assert!(path.exists() && path.is_file());
        if path.exists() && path.is_file() {
            fs::remove_file(path).unwrap();
        }
    }
}
