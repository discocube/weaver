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
        fn get_radius(self) -> ScalarXyz;
        /// Get maximum scalar value allowed for x, y or z where n is the order.
        fn get_radius_usize(self) -> usize;
        /// Get maximum scalar value allowed for x, y or z where n is the order.
        fn get_radius_from_order(self) -> ScalarXyz;
        /// get level from order, or n.
        fn get_n_from_order(self) -> Count;
        /// get order from level/n.
        fn get_order_from_n(self) -> Count;
        /// spool size is the number of verts in the graph whose z-scalar value is -1.
        fn get_spool_size(self) -> Count;
        /// Get length of the current level.
        fn z_color_len(self) -> ZrowColorSize;
    }

    impl InfoN for Count {
        fn loom_size(self) -> Count {
            (self / 2) + 1
        }

        fn get_max_absumv(self) -> ScalarXyz {
            (self * 2 + 1) as ScalarXyz
        }

        fn get_radius(self) -> ScalarXyz {
            (self * 2 - 1) as ScalarXyz
        }

        fn get_radius_usize(self) -> usize {
            self * 2 - 1
        }

        fn get_radius_from_order(self) -> ScalarXyz {
            (((3 * self) / 4) as f64).powf(1.0 / 3.0) as i16 * 2 - 1
        }

        fn get_n_from_order(self) -> Count {
            (((3 * self) / 4) as f64).powf(1.0 / 3.0) as Count
        }

        fn get_order_from_n(self) -> Count {
            (4 * (self + 2) * (self + 1) * self) / 3
        }

        fn get_spool_size(self) -> Count {
            2 * self * (self + 1)
        }

        fn z_color_len(self) -> ZrowColorSize {
            zip(
                zip(
                    (-((self * 2 - 1) as i16)..=-1).step_by(2),
                    match self % 2 {
                        0 => repeat(1).take(self).interleave(repeat(3).take(self)),
                        _ => repeat(3).take(self).interleave(repeat(1).take(self)),
                    },
                ),
                (1..=self).map(|f| 2 * f * (f + 1)),
            )
            .collect()
        }
    }
}

/// 🛞 Spin a zigzagging-inward-spiralling Hamiltonian chain from the outer to innermost vert where the scalar z value of the points equals -1.
mod spin_yarn {
    use super::{graph_info_from_n::InfoN, prepost_iter::{PostfacedIter, PrefacedIter}};
    use crate::graph::types::*;

    /// 2d displacement vectors for walking a zig-zag inwards.
    pub const DPYX_EVEN: [[[i16; 2]; 2]; 4] = [
        [[0, 2], [2, 0]],
        [[0, 2], [-2, 0]],
        [[0, -2], [-2, 0]],
        [[0, -2], [2, 0]],
    ];

    /// 2d displacement vectors for walking a zig-zag inwards.
    pub const DPYX_ODD: [[[i16; 2]; 2]; 4] = [
        [[0, -2], [-2, 0]],
        [[0, -2], [2, 0]],
        [[0, 2], [2, 0]],
        [[0, 2], [-2, 0]],
    ];

    /// 🪀 Spin a zigzagging-inward-spiralling Hamiltonian chain using displacement vectors to determine the next step. No backtracking, just a switching of displacement vectors at specific cuts to assure the path form is turning always inwards.
    pub trait Spin {
        /// 🪀 Spin a zigzagging-inward-spiralling Hamiltonian chain.
        ///
        ///---\
        /// `📏 spool_size`: Spool length and longest hamiltonian chain consisting only of points whose scalar z-value is -1.\
        /// `🧵 spool`: Container upon which the yarn is spun or the tour recorded.\
        /// `🛞 turns`: cuts indicating when to get `next(pair of xy-zigzag displacement vectors)`.\
        /// `🎚️ radius`:  Max scalar value for x, y, z used as the first step and to seed the array: `[radius, 1]`.\
        /// `🔀 switch`:  an infinite switch iterator of cuts [[1, 0]] of the y and x axis of the displacement vector.\
        ///---\
        /// Pre-populate the `spool` with start the vector `[max_yxz, 1]` * `spool_size` to avoid reallocation of space. Take steps by looping over the pre-populated elements of `spool` starting from the first index `spool[0]` and calculating the next step by adding either an x or y (switching at each step) displacement vector to the current vector `spool[idx].add_vec(zigzag[y_or_x])` and taking that step by assigning the result to the next index `spool[idx + 1]`. When it's time to turn `turn == idx`, get the new set of zig-zag vectors to change direction (turning inward) and take a step: add that new x or y vector to the current vector and assign to the next index. Each step alternates between x and y.
        ///
        ///
        fn spin_out(n: usize) -> Spindle;
    }

    impl Spin for Spindle {
        fn spin_out(n: usize) -> Spindle {
            // if n == 4:
            // r = n * 2 - 1
            // r = (4 * 2) - 1
            // r = 7
            let r = n.get_radius_usize();
            let (start, disp_vects) = if n % 2 == 0 {
                ([1, 1], DPYX_EVEN)
            } else {
                ([-1, 1], DPYX_ODD)
            };
            (1..r + 1)
                // (1, 2, 3, 4, 5, 6, 7)
                .step_by(2)
                // (1, 3, 5, 7)
                .flat_map(|x| [x; 2])
                // .flat_map(|x| if x < r {vec![x; 2]} else {vec![x; 3]})
                // (1, 1, 3, 3, 5, 7, 7)
                .postfaced_with(r)
                // (1, 1, 3, 3, 5, 7, 7, 7)
                .enumerate()
                // idx is used to cycle over the disp_vectors, the first `idx % 4` cycles through the disp_vects [0, 1, 2, 3]
                // and the second: `(idx + 1) % 2` results in [0, 1, 0, 1, ...] which alternates between the y and x displacement vector.
                // [(0, 1), (1, 1), (2, 3), (3, 3), (4, 5), (5, 5), (6, 7), (7, 7), (8, 7)]
                .flat_map(|(idx, len)| {
                    (0..len)
                        .map(move |i| disp_vects[idx % 4][(idx + i) % 2])
                        // `[idx % 4]` => 0, 1, 2, 3
                        // Cycles through the zigzag vectors.
                        // `[(idx + i) % 2]` => 0, 1, 0, 1
                        // replaces the infinite iterator of [0, 1] to access y or x displacement vector in the zigzag
                        // displacement vector
                        .into_iter()
                })
                // preface with start of path: [radius, 1]
                .prefaced_with(start)
                // Apply the displacement vectors to a start state of [0, 0] to the following results.
                .scan([0, 0], |state, [x, y]| {
                    *state = [state[0] + x, state[1] + y];
                    Some(*state)
                })
                .collect()
        }
    }

    /// 🩺 Test `prepare_yarn` and `cut_yarn`
    #[cfg(test)]
    mod tests_spin_easy {
        use std::collections::HashSet;

        use itertools::all;
        use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

        use super::super::prelude::*;

        #[test]
        /// Run and test first 200 orders if output is whatexpected.
        /// Test:
        ///     l1-norm within max_l1-norm,
        ///     first and last item matches expected,
        ///     length of spun yarn,
        ///     no duplicates.
        fn test_spin_outward() {
            (1..100).for_each(|n| {
                let result = Spindle::spin_out(n);
                let radius = n.get_radius();
                // spool size: `2 * n * (n + 1)`
                let spool_size = n.get_spool_size();
                assert_eq!(result.len(), spool_size);
                assert_eq!(result[0], [if n % 2 == 0 { 1 } else { -1 }, 1]);
                assert_eq!(*result.last().unwrap(), [radius, 1]);
                let [first_x, first_y] = result.first().unwrap();
                assert_eq!(*first_y, 1);
                assert!(*first_x == -1 || *first_x == 1);
                assert!(all(result, |vert| vert.l1norm() <= n.get_max_absumv()));
                let result = Spindle::spin_out(n);
                let set_result = result.par_iter().collect::<HashSet<_>>();
                assert_eq!(spool_size, set_result.len());
                let first = result.first().unwrap();
                match n % 2 == 0 {
                    true => assert_eq!(*first, [1, 1]),
                    false => assert_eq!(*first, [-1, 1]),
                }
            });
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

/// Append prepend to iterator
pub mod prepost_iter {

    /// Trait for types that can be prepended with an element of the same type.
    pub trait PrefacedWith<T, U> {
        /// Returns a new instance of the type that has `item` prepended to it.
        ///
        /// ## Arguments
        ///
        /// * `item` - The element to prepend to the container.
        ///
        /// ## Returns
        ///
        /// A new instance of the type with `item` prepended to it.
        fn prefaced_with(self, item: T) -> U;
    }

    use std::iter::{once, Chain, FromIterator, Once};

    impl<T, U> PrefacedWith<T, U> for U
    where
        U: IntoIterator<Item = T> + FromIterator<T>,
    {
        /// Returns a new instance of the type that has `item` prepended to it.
        ///
        /// ## Arguments
        ///
        /// * `item` - The element to prepend to the container.
        ///
        /// ## Returns
        ///
        /// A new instance of the type with `item` prepended to it.
        fn prefaced_with(self, item: T) -> U {
            once(item).chain(self.into_iter()).collect()
        }
    }

    /// Trait for iterators that can be prepended with an element of the same type.
    pub trait PostfacedIter<T>: Iterator<Item = T> {
        /// Returns a new iterator that has `item` prepended to it.
        ///
        /// ## Arguments
        ///
        /// * `item` - The element to prepend to the iterator.
        ///
        /// ## Returns
        ///
        /// A new iterator that has `item` prepended to it.
        fn postfaced_with(self, item: T) -> Chain<Self, Once<T>>
        where
            Self: Sized;
    }

    impl<T, I> PostfacedIter<T> for I
    where
        I: Iterator<Item = T>,
    {
        /// Returns a new iterator that has `item` prepended to it.
        ///
        /// ## Arguments
        ///
        /// * `item` - The element to prepend to the iterator.
        ///
        /// ## Returns
        ///
        /// A new iterator that has `item` prepended to it.
        /// same as:
        /// ```
        /// self.chain(once(item))
        /// ```
        fn postfaced_with(self, item: T) -> Chain<Self, Once<T>> {
            self.chain(once(item))
        }
    }

    /// Trait for iterators that can be prepended with an element of the same type.
    pub trait PrefacedIter<T>: Iterator<Item = T> {
        /// Returns a new iterator that has `item` prepended to it.
        ///
        /// ## Arguments
        ///
        /// * `item` - The element to prepend to the iterator.
        ///
        /// ## Returns
        ///
        /// A new iterator that has `item` prepended to it.
        fn prefaced_with(self, item: T) -> std::iter::Chain<std::iter::Once<T>, Self>
        where
            Self: Sized;
    }

    impl<T, I> PrefacedIter<T> for I
    where
        I: Iterator<Item = T>,
    {
        /// Returns a new iterator that has `item` prepended to it.
        ///
        /// ## Arguments
        ///
        /// * `item` - The element to prepend to the iterator.
        ///
        /// ## Returns
        ///
        /// A new iterator that has `item` prepended to it.
        /// same as:
        /// ```
        /// once(item).chain(self)
        /// ```
        fn prefaced_with(self, item: T) -> Chain<Once<T>, Self> {
            once(item).chain(self)
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
        fn pin_thread_ends(&mut self, zrow: i16) -> PinCushion;
    }

    impl PinThreadEnds for Loom {
        fn pin_thread_ends(&mut self, zrow: i16) -> PinCushion {
            self.iter_mut()
                .flat_map(|t| {
                    let [[x, y, _], [i, j, _]] = [t[0], t[t.len() - 1]];
                    t.add_pins([x, y, zrow], [i, j, zrow])
                })
                .collect()
        }
    }

    /// Add two pins, one for each end of the thread. Collect copy of each as a guide for cutting/segmenting and placing the yarn.
    pub trait AddPinsFrontBack {
        fn add_pins(&mut self, front: [i16; 3], back: [i16; 3]) -> [[i16; 3]; 2];
    }

    impl AddPinsFrontBack for LoomThread {
        fn add_pins(&mut self, front: [i16; 3], back: [i16; 3]) -> [[i16; 3]; 2] {
            self.push_front(front);
            self.push_back(back);
            [front, back]
        }
    }
}

/// 👨‍🍳 Prepare yarn for extending onto the loom threads. Cut using pins and affix yarn to the current elevation.
pub mod prepare_yarn {
    use crate::graph::types::*;
    use itertools::Itertools;
    use ndarray::s;
    use rayon::prelude::*;

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
        fn prep(&self, zpos: i16, color: u8, len: usize) -> Warp {
            self[&color]
                .slice(s![..len, ..])
                .outer_iter()
                .map(|row| [row[0], row[1], zpos])
                .collect_vec()
        }
    }

    /// 🔪 Cut yarn using pins from the pins as cut markers so it can be extended upon the individual threads in the loom.
    pub trait SegmentYarn {
        /// 🔪 Chop yarn using pins from the pins as cut markers.
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
        fn chop(self, pins: &mut PinCushion) -> Warps;
    }

    impl SegmentYarn for Warp {
        fn chop(mut self: Warp, pins: &mut PinCushion) -> Warps {
            // This version would probably be faster if I constructed the yarn so it's reversed to begin with.
            // This involves, changing prep(), and changing the whole spin() module to spin outward.
            // the displacement vectors also have to be changed.
            match !pins.is_empty() {
                true => {
                    let mut warps = Warps::with_capacity(pins.len() + 1);
                    self.par_iter()
                        .enumerate()
                        .filter_map(|(i, p)| pins.contains(p).then_some(i))
                        .collect::<Vec<_>>()
                        .into_iter()
                        .enumerate()
                        .rev()
                        .for_each(|(idx, i)| match idx == 0 {
                            true if !pins.contains(&self[0]) => {
                                warps.push(self.drain(i + 1..).collect::<Vec<_>>());
                                warps.push(self.drain(..).rev().collect::<Vec<_>>())
                            }
                            _ => warps.push(self.drain(i..).collect::<Vec<_>>()),
                        });
                    warps
                }
                false => {
                    self.reverse();
                    vec![self]
                }
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

/// 🪞 Reflect the half-solution along the z-axis to create the whole: ◡ + ◠ = ◯.
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
                        .par_iter()
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
    use super::{graph_info_from_n::InfoN, serialize_csv::SerializeToCsv};
    use crate::graph::types::*;
    use itertools::Itertools;
    use rayon::prelude::*;

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
                    .into_par_iter()
                    .map(|mut data| data.drain(..).collect())
                    .collect(),
            )
        }
    }

    /// Weft is the main loop into which warps are incorporated.
    #[derive(Clone, Debug)]
    pub struct Weft {
        // Lead subcycle with which others are joined.
        pub data: Tour,
        // Indicates whether weft has joined which changes the conditions for valid bridge edges.
        pub joined: bool,
        // The maximum absolute scalar value for z.
        pub max_abs_z: i16,
        // Max_abs_z * 2 to represent two z scalar values of an edge which is added together and then the absolute value compared to this value to determine current elevation.
        pub max_sum_z: i16,
    }

    impl Weft {
        /// Create a new instance of weft which contains methods that are slightly different from those of warps.
        /// Weft is the main loop into which the warps are joined.
        /// The final size of the solution is preallocated to the weft to avoid reallocations.
        pub fn new(mut data: LoomThread, order: Count) -> Weft {
            let mut preallocated = Warp::with_capacity(order);
            preallocated.extend(data.drain(..));
            let max_abs_z = (order.get_radius_from_order() - 4) as i16;
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
                // find just one edge matching condition use find instead of iterating over entire tupled windows.
                .find(|(&[x, _, z], &[a, _, c])| {
                    [x, a] == if self.joined { [1, 1] } else { [1, 3] }
                        // Matching to the current merge elevation which always increases. 
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

        /// Save the finished solution to a csv file.
        pub fn export_csv(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
            self.data.serialize_to_csv(filepath)?;
            Ok(())
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
                    ([b, x, y] == if joined { [1, 3, 1] } else { [3, 1, 3] })
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
                (m, n) if m < n => self,
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
        fn bridge(&self, (weft_lhs, weft_rhs): &WeftEdge) -> BridgeEdge {
            let &(warp_lhs, warp_rhs) = ((*weft_lhs, *weft_rhs).eadjs())
                .intersection(self)
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
                        // Determine along which axis this edge lies to get parallel edge.
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
            // Determine along which axis this edge lies to get parallel edge.
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

/// Get information about solution like non-turn count and a count of the axes of edges.
pub mod grade_solution {
    use itertools::Itertools;

    use super::prelude::{Edge, Solution};

    /// Grade Solution for edges' axes count and count of nonturns in solution.
    pub trait Grade {
        /// Calculate the axis on which each edge lies.
        fn axes(&self) -> [usize; 3];
        /// get axes count and divide each axis by order
        fn axis_percent(&self) -> [f64; 3];
        /// Count the number of nonturns of solution.
        fn nonturns(&self) -> usize;
    }

    impl Grade for Solution {
        fn axes(&self) -> [usize; 3] {
            self.iter()
                .circular_tuple_windows()
                .fold([0; 3], |mut acc, (m, n)| {
                    acc[(*m, *n).axis()] += 1;
                    acc
                })
        }

        fn axis_percent(&self) -> [f64; 3] {
            let [x_count, y_count, z_count] = self.axes();
            let [x_count, y_count, z_count] = [x_count as f64, y_count as f64, z_count as f64];
            let order = x_count + y_count + z_count;
            [
                (x_count / order) * 100.0,
                (y_count / order) * 100.0,
                (z_count / order) * 100.0,
            ]
        }

        fn nonturns(&self) -> usize {
            let mut nonturns: usize = 0;
            let mut prev_axis = (self[self.len() - 1], self[0]).axis();
            self.iter()
                .circular_tuple_windows()
                .for_each(|(m, n): (&[i16; 3], &[i16; 3])| {
                    let curr_axis = match (*m, *n).axis() {
                        curr_axis if curr_axis == prev_axis => {
                            nonturns += 1;
                            curr_axis
                        }
                        curr_axis => curr_axis,
                    };
                    prev_axis = curr_axis;
                });
            nonturns
        }
    }

    /// Get the given axis of a self and test if the length is the edge's unit length of 2.
    pub trait GetEdgeAxis {
        /// Get axis of edge.
        fn axis(&self) -> usize;
    }

    impl GetEdgeAxis for Edge {
        fn axis(&self) -> usize {
            let ([a, b, c], [x, y, z]) = *self;
            match [a != x, b != y, c != z] {
                [true, false, false] if ((a - x).abs() == 2) => 0,
                [false, true, false] if ((b - y).abs() == 2) => 1,
                [false, false, true] if ((c - z).abs() == 2) => 2,
                _ => panic!("NOT A VALID EDGE"),
            }
        }
    }

    /// 🩺 Test `axis()`, `axes()`, `nonturns()` by solving smaller instances and checking the expected with the actual output.
    #[cfg(test)]
    mod tests_grade_solution {
        use crate::graph::{ops::prelude::InfoN, weave::weave};

        use super::*;

        #[test]
        /// Solve for n and count edges axes which is around 50% for each x and y and 0.1% for z.
        fn test_count_axes() {
            // Test cube (the first instance) which has a different count ratio than the proceeding orders.
            let n_1 = 1_usize;
            let mut order = n_1.get_order_from_n();
            let [mut x_count, mut y_count, mut z_count] = weave(n_1).axes();
            // Test that x and z count is 2 and y count == 4.
            assert!(z_count == x_count && x_count == 2 && y_count == 4);
            // sum of the xyz counts should equal the order.
            assert_eq!(order, [x_count, y_count, z_count].iter().sum());
            // Test the rest of the orders which have the same count proportions.
            for n in 2..=100 {
                order = n.get_order_from_n();
                let solution = weave(n);
                [x_count, y_count, z_count] = solution.axes();
                let [x_part, y_part, z_part] = solution.axis_percent();
                // check that z < x < y;
                assert!(z_count < x_count && x_count < y_count);
                // sum of the xyz counts should equal the order.
                assert_eq!(order, [x_count, y_count, z_count].iter().sum());
                // check that x and y are around about 50% and z at 1% when rounded. Closer to 50/50/1 when n increases.
                let [xc, yc, zc] = [x_part.round(), y_part.round(), z_part.round()];
                assert!(if n > 48 && n < 70 {
                    xc == 49.0 && yc == 49.0 && zc == 1.0
                } else if n < 49 {
                    xc <= 49.0 && yc <= 49.0 && zc <= 19.0
                } else {
                    xc <= 50.0 && yc == 50.0 && zc == 1.0
                });
            }
        }

        #[test]
        /// Solve for n and count nonturns if n < 3 the result is zero else 2.
        fn test_count_nonturns() {
            for n in 1..=100 {
                assert_eq!(if n < 3 { 0 } else { 2 }, weave(n).nonturns());
            }
        }

        #[test]
        /// Test if given edge.axis() is expected.
        fn test_edge_axis() {
            assert_eq!(([0, 2, 0], [0, 0, 0]).axis(), 1);
            assert_eq!(([0, 2, 0], [0, 2, 2]).axis(), 2);
            assert_eq!(([0, 2, 2], [2, 2, 2]).axis(), 0);
        }

        #[test]
        #[should_panic(expected = "NOT A VALID EDGE")]
        /// Test expected panic of same edge.
        fn test_invalid_same_edge() {
            ([0, 2, 0], [0, 2, 0]).axis();
        }

        #[test]
        #[should_panic(expected = "NOT A VALID EDGE")]
        /// Test expected panic of invalid Edge Length.
        fn test_invalid_same_edge_bad_length() {
            ([0, 2, 0], [0, 2, 3]).axis();
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
            match self[0] - x + self[1] - y + self[2] - z {
                n if n == 2 || n == -2 => true,
                _ => false,
            }
        }
    }
}

/// ✅ Certify if the solution is Hamiltonian.
pub mod certify_solution_node {
    use crate::graph::types::{V2d, V3d};
    use itertools::Itertools;

    use std::{
        collections::{HashMap, HashSet},
        fmt,
    };

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
    pub trait CertifyNode<SequenceID> {
        /// Certify if a sequence is a Hamiltonian cycle by:
        /// Checking for duplicates
        /// Check that solution length is equal to the order of the graph.
        /// The sum of all displacement vectors used to construct the cycle is equal to [0, 0, 0]
        /// Check that the current node is adjacent to the next node.
        /// Check that the last node is adjacent to the first. If not is is a HamChain else HamCycle.
        fn certify(&mut self, adj: &HashMap<u32, HashSet<u32>>) -> SequenceID;
    }

    impl CertifyNode<SequenceID> for &mut Vec<u32> {
        fn certify(&mut self, adj: &HashMap<u32, HashSet<u32>>) -> SequenceID {
            if self.iter().duplicates().count() > 0 || self.len() != adj.len() {
                return SequenceID::Broken;
            }
            match self
                .windows(2)
                .all(|window| adj[&window[0]].contains(&window[1]))
            {
                true if adj[&self[self.len() - 1]].contains(&self[0]) => SequenceID::HamCycle,
                true => SequenceID::HamChain,
                false => SequenceID::Broken,
            }
        }
    }

    impl CertifyNode<SequenceID> for &mut [u32; 12320] {
        fn certify(&mut self, adj: &HashMap<u32, HashSet<u32>>) -> SequenceID {
            if self.iter().duplicates().count() > 0 || self.len() != adj.len() {
                return SequenceID::Broken;
            }
            match self
                .windows(2)
                .all(|window| adj[&window[0]].contains(&window[1]))
            {
                true if adj[&self[self.len() - 1]].contains(&self[0]) => SequenceID::HamCycle,
                true => SequenceID::HamChain,
                false => SequenceID::Broken,
            }
        }
    }

    impl CertifyNode<SequenceID> for Vec<u32> {
        fn certify(&mut self, adj: &HashMap<u32, HashSet<u32>>) -> SequenceID {
            if self.iter().duplicates().count() > 0 || self.len() != adj.len() {
                return SequenceID::Broken;
            }
            match self
                .windows(2)
                .all(|window| adj[&window[0]].contains(&window[1]))
            {
                true if adj[&self[self.len() - 1]].contains(&self[0]) => SequenceID::HamCycle,
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
            self.iter().map(|v| v.abs()).sum()
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
            match self[0] - x + self[1] - y + self[2] - z {
                n if n == 2 || n == -2 => true,
                _ => false,
            }
        }
    }
}

/// Module for translating the solution into a string by using a list of 30 unique characters
/// Goal: Either choose your own unique 30 chars or
pub mod translate {
    use itertools::{Itertools, iproduct};
    use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

    use super::prelude::{L1Norm, Node, Nodes, Vectors, InfoN};
    
    /// Makes vertices based on radius.
    pub fn make_vertices(order: usize) -> Vec<[i16; 3]> {
        let radius = order.get_radius_from_order();
        let radius_plus_4 = radius + 4;
        iproduct!(
            (-radius..=radius).step_by(2),
            (-radius..=radius).step_by(2),
            (-radius..=radius).step_by(2)
        )
        .filter_map(|(x, y, z)| ([x, y, z].l1norm() < radius_plus_4).then_some([x, y, z]))
        .sorted_by_key(|&vert| (vert.l1norm(), vert[0], vert[1]))
        .collect::<Vec<_>>()
    }

    /// Convert a vector of indices to vectors to those vectors.
    pub trait NodesToVectors {
        fn to_vectors(&self) -> Vectors;
    }

    impl NodesToVectors for Nodes {
        fn to_vectors(&self) -> Vectors {
            let verts = make_vertices(self.len());
            self.par_iter().map(|&node| verts[node as usize]).collect()
        }
    }

    /// Convert a vector of points to a vector of their indices in the vertices list.
    pub trait VectorsToNodes {
        // Check which is faster?
        fn to_nodes(&self) -> Nodes;
    }

    impl VectorsToNodes for Vectors {
        fn to_nodes(&self) -> Nodes {
            self.iter()
                .enumerate()
                .sorted_by_key(|&(_, v)| (v.l1norm(), v[0], v[1], v[2]))
                .enumerate()
                .sorted_by_key(|&(_, (i, _))| i)
                .map(|(idx, _)| idx as Node)
                .collect()
        }
    }
}

pub mod serialize_chars {
    use std::{
        collections::{HashMap, HashSet},
        ops::Sub,
    };

    use lazy_static::lazy_static;

    use common_macros::hash_map;
    use itertools::Itertools;
    use rayon::prelude::*;

    use super::{
        prelude::{Solution, Tour},
        translate::VectorsToNodes,
    };

    pub fn md([a, b, c]: [i16; 3], [x, y, z]: [i16; 3]) -> usize {
        ((a - x).abs() + (b - y).abs() + (c - z).abs())
            .try_into()
            .unwrap()
    }

    /// maps to and from a single-byte character. SBCS
    static CORNERS: [&str; 30] = [
        "XY", "XZ", "Xy", "Xz", "YX", "YZ", "Yx", "Yz", "ZX", "ZY", "Zx", "Zy", "xY", "xZ", "xy",
        "xz", "yX", "yZ", "yx", "yz", "zX", "zY", "zx", "zy", "XX", "YY", "ZZ", "xx", "yy", "zz",
    ];

    /// US-ASCII first 128 chars
    pub static VALID_CHARS: &str = r"!”#$%&’()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{}~";
    lazy_static! {
        static ref STR_VEC: HashMap<char, [i16; 3]> = {
            hash_map! {
                'S' => [-1, -1, -1],
                'X' => [2, 0, 0],
                'x' => [-2, 0, 0],
                'Y' => [0, 2, 0],
                'y' => [0, -2, 0],
                'Z' => [0, 0, 2],
                'z' => [0, 0, -2]
            }
        };
        static ref VEC_STR: HashMap<[i16; 3], char> = {
            hash_map! {
                [2, 0, 0] => 'X',
                [-2, 0, 0] => 'x',
                [0, 2, 0] => 'Y',
                [0, -2, 0] => 'y',
                [0, 0, 2] => 'Z',
                [0, 0, -2] => 'z',
            }
        };
        static ref STR_CHR: HashMap<String, char> = {
            hash_map! {
                "XY".to_string() => 'a',
                "XZ".to_string() => 'b',
                "Xy".to_string() => 'c',
                "Xz".to_string() => 'd',
                "YX".to_string() => 'e',
                "YZ".to_string() => 'f',
                "Yx".to_string() => 'g',
                "Yz".to_string() => 'h',
                "ZX".to_string() => 'i',
                "ZY".to_string() => 'j',
                "Zx".to_string() => 'k',
                "Zy".to_string() => 'l',
                "xY".to_string() => 'm',
                "xZ".to_string() => 'n',
                "xy".to_string() => 'o',
                "xz".to_string() => 'p',
                "yX".to_string() => 'q',
                "yZ".to_string() => 'r',
                "yx".to_string() => 's',
                "yz".to_string() => 't',
                "zX".to_string() => 'u',
                "zY".to_string() => 'v',
                "zx".to_string() => 'w',
                "zy".to_string() => 'x',
                "XX".to_string() => '0',
                "YY".to_string() => '1',
                "ZZ".to_string() => '2',
                "xx".to_string() => '3',
                "yy".to_string() => '4',
                "zz".to_string() => '5',

            }
        };
        static ref CHR_STR: HashMap<char, String> = {
            hash_map! {
                'a' => "XY".to_string(),
                'b' => "XZ".to_string(),
                'c' => "Xy".to_string(),
                'd' => "Xz".to_string(),
                'e' => "YX".to_string(),
                'f' => "YZ".to_string(),
                'g' => "Yx".to_string(),
                'h' => "Yz".to_string(),
                'i' => "ZX".to_string(),
                'j' => "ZY".to_string(),
                'k' => "Zx".to_string(),
                'l' => "Zy".to_string(),
                'm' => "xY".to_string(),
                'n' => "xZ".to_string(),
                'o' => "xy".to_string(),
                'p' => "xz".to_string(),
                'q' => "yX".to_string(),
                'r' => "yZ".to_string(),
                's' => "yx".to_string(),
                't' => "yz".to_string(),
                'u' => "zX".to_string(),
                'v' => "zY".to_string(),
                'w' => "zx".to_string(),
                'x' => "zy".to_string(),
                '0' => "XX".to_string(),
                '1' => "YY".to_string(),
                '2' => "ZZ".to_string(),
                '3' => "xx".to_string(),
                '4' => "yy".to_string(),
                '5' => "zz".to_string(),

            }
        };
    }

    pub trait GetPivotsKeyLhs {
        fn get_pivot_key_lhs(
            &self,
            adj: &HashMap<u32, HashSet<u32>>,
            verts: &Vec<[i16; 3]>,
        ) -> usize;
    }

    pub trait GetPivotsKeyRhs {
        fn get_pivot_key_rhs(
            &self,
            adj: &HashMap<u32, HashSet<u32>>,
            verts: &Vec<[i16; 3]>,
        ) -> usize;
    }

    impl GetPivotsKeyLhs for Vec<u32> {
        fn get_pivot_key_lhs(
            &self,
            adj: &HashMap<u32, HashSet<u32>>,
            verts: &Vec<[i16; 3]>,
        ) -> usize {
            let first = verts[self[0] as usize];
            self.index(
                self.get_pivots_lhs(adj)
                    .into_iter()
                    .sorted_by_key(|&node| md(verts[node as usize], first))
                    .next()
                    .unwrap(),
            )
        }
    }

    impl GetPivotsKeyRhs for Vec<u32> {
        fn get_pivot_key_rhs(
            &self,
            adj: &HashMap<u32, HashSet<u32>>,
            verts: &Vec<[i16; 3]>,
        ) -> usize {
            let first = verts[self[0] as usize];
            self.index(
                self.get_pivots_rhs(adj)
                    .into_iter()
                    .sorted_by_key(|&node| md(verts[node as usize], first))
                    .next()
                    .unwrap(),
            ) + 1
        }
    }

    pub trait GetPivotsLhs {
        fn get_pivots_lhs(&self, adj: &HashMap<u32, HashSet<u32>>) -> HashSet<u32>;
        fn get_pivot_lhs(&self, adj: &HashMap<u32, HashSet<u32>>) -> usize;
    }

    pub trait GetPivotsRhs {
        fn get_pivots_rhs(&self, adj: &HashMap<u32, HashSet<u32>>) -> HashSet<u32>;
        fn get_pivot_rhs(&self, adj: &HashMap<u32, HashSet<u32>>) -> usize;
    }

    impl GetPivotsLhs for Vec<u32> {
        fn get_pivots_lhs(&self, adj: &HashMap<u32, HashSet<u32>>) -> HashSet<u32> {
            adj[&self[0]].sub(&HashSet::from([self[1]]))
        }

        fn get_pivot_lhs(&self, adj: &HashMap<u32, HashSet<u32>>) -> usize {
            self.index(self.get_pivots_lhs(adj).into_iter().next().unwrap().clone())
        }
    }

    impl GetPivotsRhs for Vec<u32> {
        fn get_pivots_rhs(&self, adj: &HashMap<u32, HashSet<u32>>) -> HashSet<u32> {
            let last = self.len() - 1;
            adj[&self[last]].sub(&HashSet::from([self[last - 1]]))
        }

        fn get_pivot_rhs(&self, adj: &HashMap<u32, HashSet<u32>>) -> usize {
            self.index(self.get_pivots_rhs(adj).into_iter().next().unwrap()) + 1
        }
    }

    pub trait Index<T> {
        fn index(&self, item: T) -> usize;
    }

    impl Index<u32> for Vec<u32> {
        fn index(&self, item: u32) -> usize {
            self.par_iter().position_any(|&n| n == item).unwrap()
        }
    }
    /// Rotate the loop so that [1, 1, 1] is in the first position and loop[1] < loop[-1] after keying.
    /// ```
    /// let mut vecloop = vec![[3, 3, 1], [3, 1, 1], [1, 1, 1], [1, 1, -1]];
    /// vecloop.keyed();
    /// assert_eq!(vecloop, vec![[1, 1, 1], [1, 1, -1], [3, 3, 1], [3, 1, 1]]);
    /// ```
    pub trait KeyLoop {
        /// Rotate the loop so that [1, 1, 1] is in the first position and loop[1] < loop[-1] after keying.
        fn keyed(&self) -> Tour;
    }

    impl KeyLoop for Tour {
        fn keyed(&self) -> Tour {
            let mut result = self.clone();
            let idx = result
                .par_iter()
                .position_any(|&vert| vert == [-1, -1, -1])
                .unwrap();
            result.rotate_left(idx);
            if result[result.len() - 1] < result[1] {
                result.rotate_left(1);
                result.reverse();
            }
            result
        }
    }

    pub trait AddVectors {
        fn add(self, prev: Self) -> Self;
    }

    impl AddVectors for [i16; 3] {
        fn add(self, [x, y, z]: Self) -> Self {
            [x + self[0], y + self[1], z + self[2]]
        }
    }

    pub trait SubtractVectors {
        fn sub(self, prev: Self) -> Self;
    }

    impl SubtractVectors for [i16; 3] {
        fn sub(self, [x, y, z]: Self) -> Self {
            [x - self[0], y - self[1], z - self[2]]
        }
    }

    pub trait VectorDisplacement {
        fn get_vdisps(&self) -> Solution;
    }

    impl VectorDisplacement for Solution {
        fn get_vdisps(&self) -> Solution {
            let last_idx = self.len() - 1;
            self.par_iter()
                .enumerate()
                .map(|(i, v)| v.sub(self[if i != last_idx { i + 1 } else { 0 }]))
                .collect()
        }
    }

    pub trait ChardDisplacementVector {
        fn as_chrds(self) -> String;
    }

    impl ChardDisplacementVector for Solution {
        fn as_chrds(self) -> String {
            self.par_iter().map(|vec| VEC_STR[vec]).collect()
        }
    }

    pub trait PairChrds {
        fn pair(&self) -> Vec<String>;
    }

    pub trait VecToString {
        fn to_string(&self) -> String;
        fn to_string_with(&self, key: &str) -> String;
    }

    impl VecToString for Vec<String> {
        fn to_string(&self) -> String {
            self.par_iter().map(|p| STR_CHR[p]).collect()
        }

        fn to_string_with(&self, key: &str) -> String {
            let str_chr: HashMap<String, char> = HashMap::from(
                key.chars()
                    .enumerate()
                    .map(|(idx, key)| (CORNERS[idx].to_string(), key))
                    .collect::<HashMap<String, char>>(),
            );
            self.par_iter().map(|p| str_chr[p]).collect()
        }
    }

    pub trait Encode {
        fn encode(&self) -> String;
        fn encode_with(&self, key: &str) -> String;
    }

    impl Encode for Solution {
        fn encode(&self) -> String {
            self.keyed().get_vdisps().as_chrds().pair().to_string()
        }

        fn encode_with(&self, key: &str) -> String {
            self.keyed()
                .get_vdisps()
                .as_chrds()
                .pair()
                .to_string_with(key)
        }
    }

    pub trait Decode {
        fn decode(&self) -> Solution;
        fn decode_with(&self, key: String) -> Solution;
        fn decode_to_node(&self) -> Vec<u32>;
    }

    impl Decode for String {
        fn decode(&self) -> Solution {
            [
                vec![[-1, -1, -1]],
                self.chars()
                    .flat_map(|letter| CHR_STR[&letter].unpair())
                    .into_iter()
                    .map(|s| STR_VEC[&s])
                    .collect_vec()[..(self.len() * 2) - 1]
                    .to_vec(),
            ]
            .concat()
            .iter()
            .scan([0, 0, 0], |state, vd| {
                *state = state.add(*vd);
                Some(*state)
            })
            .collect_vec()
        }

        fn decode_to_node(&self) -> Vec<u32> {
            self.decode().to_nodes()
        }

        fn decode_with(&self, key: String) -> Solution {
            let decoder: HashMap<char, &str> = HashMap::from(
                key.chars()
                    .enumerate()
                    .map(|(idx, c)| (c, CORNERS[idx]))
                    .collect::<HashMap<char, &str>>(),
            );
            [
                vec![[-1, -1, -1]],
                self.chars()
                    .flat_map(|letter| decoder[&letter].unpair())
                    .into_iter()
                    .map(|s| STR_VEC[&s])
                    .collect_vec()[..(self.len() * 2) - 1]
                    .to_vec(),
            ]
            .concat()
            .iter()
            .scan([0, 0, 0], |state, vd| {
                *state = state.add(*vd);
                Some(*state)
            })
            .collect_vec()
        }
    }

    pub trait Unpair {
        fn unpair(&self) -> Vec<char>;
    }

    impl Unpair for String {
        fn unpair(&self) -> Vec<char> {
            self.chars().collect_vec()
        }
    }

    impl Unpair for &str {
        fn unpair(&self) -> Vec<char> {
            self.chars().collect_vec()
        }
    }

    pub trait GetVectors {
        fn as_points(self) -> Solution;
    }

    impl GetVectors for String {
        fn as_points(self) -> Solution {
            ['S'.to_string(), self[..self.len() - 1].to_string()]
                .concat()
                .chars()
                .scan([0, 0, 0], |state, char| {
                    *state = state.add(STR_VEC[&char]);
                    Some(*state)
                })
                .collect_vec()
        }
    }

    impl PairChrds for String {
        fn pair(&self) -> Vec<String> {
            self.chars()
                .collect_vec()
                .chunks(2)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect_vec()
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

///! 🩺 TEST
///!
///!
///!
///!
///!
///! 🩺 Test overflow and expected outputs for n.get_some_info().
#[cfg(test)]
mod tests_graph_info_from_n {
    use super::prelude::*;

    #[test]
    /// Test behavior if input is max_i16
    fn test_info_n_with_max_i16() {
        let max_i16 = std::i16::MAX as Count;
        assert_eq!(max_i16.loom_size() as i16, 16384);
        assert_eq!(max_i16.get_max_absumv() as i16, -1);
        assert_eq!(max_i16.get_radius(), -3);
        assert_eq!(max_i16.get_radius_from_order(), 57);
        assert_eq!(max_i16.get_n_from_order(), 29);
        assert_eq!(max_i16.get_order_from_n(), 46912496074752);
        assert_eq!(max_i16.get_spool_size(), 2147418112);
    }

    #[test]
    /// Test behavior if input is max_i16
    fn test_info_n_pass() {
        let n = 100;
        assert_eq!(n.loom_size() as i16, 51);
        assert_eq!(n.get_max_absumv(), 201);
        assert_eq!(n.get_radius(), 199);
        assert_eq!((n + 60).get_radius_from_order(), 7);
        assert_eq!((n - 20).get_n_from_order(), 3);
        assert_eq!(n.get_order_from_n(), 1373600);
        assert_eq!(n.get_spool_size(), 20200);
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
    // use super::prelude::*;

    #[test]
    /// Test by getting the requested color and slice of that color and if it has mapped it to a Vec<[i16; 3]>.
    fn test_prepare_yarn() {}

    #[test]
    /// Test by cutting a sequence using pins.
    fn test_cut_using() {}
}

/// 🩺 Test mark ends by marking ends of a thread.
#[cfg(test)]
mod tests_pin_threads {
    use super::prelude::*;

    #[test]
    /// Test that the output from mark end where zrow == -1 should give me an empty pin cushion.
    fn test_last_row_empty_cushion() {
        let mut loom = Loom::with_capacity(2);
        assert_eq!(PinCushion::with_capacity(0), loom.pin_thread_ends(-1));
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
    // use super::prelude::*;

    #[test]
    fn test_extend_threads() {
        // let n = 2;
        // let mut loom = Loom::with_capacity(n.loom_size());
        // let yarns = Yarns::color_spun(Spindle::spin(n.spool_size()));
        // loom.extend_threads(yarns.prep(-3, 1, 8).chop(&vec![]));
        // assert_eq!(loom, [[[1, 1, -3], [1, -1, -3], [-1, -1, -3], [-1, 1, -3]]]);
        // loom.extend_threads(yarns.prep(-1, 3, 0).chop(&vec![[1, 1, -1], [-1, 1, -1]]));
        // assert_eq!(
        //     loom,
        //     vec![
        //         vec![[1, 1, -3], [1, -1, -3], [-1, -1, -3], [-1, 1, -3]],
        //         vec![
        //             [-1, 1, -1],
        //             [-3, 1, -1],
        //             [-3, -1, -1],
        //             [-1, -1, -1],
        //             [-1, -3, -1],
        //             [1, -3, -1],
        //             [1, -1, -1],
        //             [3, -1, -1],
        //             [3, 1, -1]
        //         ],
        //         vec![[1, 1, -1], [1, 3, -1], [-1, 3, -1]]
        //     ]
        // );
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
