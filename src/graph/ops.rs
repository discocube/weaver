/// Collection of modules used to build Hamiltonian cycle.
pub mod prelude {
    pub use super::{
        color_spun_yarn::*, extend_loom_threads::*, finish_colored_yarn::*, graph_info_from_n::*,
        mark_thread_ends::*, merge_cycle::*, mirror_loom::*, spin_yarn::*,
    };
}

pub mod graph_info_from_n {
    use crate::graph::defs::*;
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
        fn zrow_color_size(self) -> ZrowColorSize;
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

        fn zrow_color_size(self) -> ZrowColorSize {
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

pub mod spin_yarn {
    use super::graph_info_from_n::InfoN;
    use crate::graph::defs::*;
    use std;

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
                spool[idx + 1] = spool[idx].add_vec(zigzag[yx])
            });
            spool
        }
    }

    /// A struct responsible for providing the right set of displacement vectors so that an inward-turning zigzag is produced from an initial vector.
    pub struct Spinner<'a> {
        /// Value of n.
        pub n: Count,
        /// A list of indices indicating when the displacement vectors should be replaced with a new set.
        pub turns: Counts,
        /// An infinite iterator of displacement vectors.
        pub zigzags: std::iter::Cycle<core::slice::Iter<'static, [V2d; 2]>>,
        /// An infinite iterator of alternating 1 and 0 which indexes to either a y-axis displacement vector or x.
        pub yxyx: std::iter::Cycle<core::slice::Iter<'a, Count>>,
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

        pub(crate) fn get_turns(n: usize) -> Counts {
            let mut result = (-(n as i64 * 2)..=-2)
                .map(|i| -i as usize)
                .step_by(2)
                .flat_map(|cut| [cut, cut])
                .scan(0, |state, n| {
                    *state += n;
                    Some(*state - 1)
                })
                .collect::<Counts>();
            result.reverse();
            result
        }

        pub(crate) fn get_zigzags() -> std::iter::Cycle<core::slice::Iter<'static, [V2d; 2]>> {
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

    pub trait AddVec {
        // add two 2-dimensional vectors together.
        fn add_vec(&self, other: V2d) -> V2d;
    }

    impl AddVec for V2d {
        fn add_vec(&self, [a, b]: V2d) -> V2d {
            [self[0] + a, self[1] + b]
        }
    }
}

pub mod color_spun_yarn {
    use ndarray::array;

    use crate::graph::defs::*;

    pub trait Convert {
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
        fn colorized(spool: Spindle) -> Yarns;
    }

    impl Convert for Yarns {
        fn colorized(mut spool: Spindle) -> Yarns {
            let blue = Yarn::from(spool.drain(..).collect::<Spindle>());
            let red = blue.dot(&array![[-1, 0], [0, -1]]) + array![[0, 2]];
            Yarns::from([(3, blue), (1, red)])
        }
    }
}

pub mod finish_colored_yarn {
    use crate::graph::defs::*;
    use itertools::Itertools;
    use ndarray::s;

    pub trait Finish {
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
        fn finish(&self, zpos: i16, color: u8, size: usize, pins: &PinCushion) -> Warps;
    }

    impl Finish for Yarns {
        fn finish(&self, zpos: i16, color: u8, size: usize, pins: &PinCushion) -> Warps {
            match self[&color]
                .slice(s![size.., ..])
                .outer_iter()
                .map(|row| [row[0], row[1], zpos])
                .collect_vec()
            {
                _yarn if pins.is_empty() => vec![_yarn],
                _yarn => _yarn.cut_using(pins),
            }
        }
    }

    pub trait CutYarn {
        /// ✂️ Cut yarn using pins from the pins as cut markers.
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
        fn cut_using(self, pins: &PinCushion) -> Warps;
    }

    impl CutYarn for Warp {
        fn cut_using(self, pins: &PinCushion) -> Warps {
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
                            warps.push(s.to_reversed_vec())
                        };
                    } else {
                        warps.push((&self[iprev..=iyarn]).to_reversed_vec());
                        iprev = iyarn + 1;
                    }
                });
            warps
        }
    }

    pub trait ReversedVec {
        /// Reverse, clone & collect a slice to `Vec`: `self.iter().rev().cloned().collect_vec()`
        fn to_reversed_vec(&self) -> Warp;
    }

    impl ReversedVec for LoomSlice<'_> {
        fn to_reversed_vec(&self) -> Warp {
            self.iter().rev().cloned().collect()
        }
    }
}

pub mod extend_loom_threads {
    use crate::graph::defs::*;

    pub trait ExtendThreads {
        /// Extend each end of each thread in the loom with the segmented, colored and finished yarn.
        fn extend_threads(&mut self, warps: Warps);
    }

    impl ExtendThreads for Loom {
        fn extend_threads(&mut self, mut warps: Warps) {
            self.iter_mut().for_each(|thread: &mut LoomThread| {
                for warp in warps.iter_mut().filter(|w| !w.is_empty()) {
                    match (thread[0] == warp[0], thread[thread.len() - 1] == warp[0]) {
                        (true, _) => thread.extend_thread_front(warp),
                        (_, true) => thread.extend_thread_back(warp),
                        _ => continue,
                    }
                }
            });
            warps.iter_mut().filter(|s| !s.is_empty()).for_each(|seq| {
                self.push(seq.drain(..).collect::<LoomThread>());
            });
        }
    }

    pub trait ExtendThreadFrontBack {
        /// Extend vec to the front of the deque by pushing each element using `push_front` but skipping the first. Named ExtendFront to disambiguate with ExtendLeft, which usually involves reversing the sequence before extending.
        fn extend_thread_front(&mut self, pinned: &mut Warp);
        /// Extend vec to the back of the deque by pushing each element using `push_back` but skipping the first. Named ExtendFront to disambiguate with ExtendLeft, which usually involves reversing the sequence before extending.
        fn extend_thread_back(&mut self, pinned: &mut Warp);
    }

    impl ExtendThreadFrontBack for LoomThread {
        fn extend_thread_front(&mut self, pinned: &mut Warp) {
            pinned
                .drain(..)
                .skip(1)
                .for_each(|item| self.push_front(item));
        }

        fn extend_thread_back(&mut self, pinned: &mut Warp) {
            self.extend(pinned.drain(..).skip(1));
        }
    }
}

pub mod mark_thread_ends {
    use crate::graph::defs::*;

    pub trait MarkEnds {
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
        ///
        fn mark_next_ends(&mut self, zrow: ScalarXyz) -> PinCushion;
    }

    impl MarkEnds for Loom {
        fn mark_next_ends(&mut self, zrow: ScalarXyz) -> PinCushion {
            match zrow == LAST_ROW {
                false => self
                    .iter_mut()
                    .flat_map(|thread| thread.mark_end())
                    .collect(),
                true => PinCushion::with_capacity(0),
            }
        }
    }

    pub trait MarkThreadEnds {
        /// Insert pins into each end of each thread in the loom. A pin is the vertex adjacent to and directly above an end. Collect the a copy of all inserted pins to be used for cutting the finished yarn from the next level up.
        fn mark_end(&mut self) -> [V3d; 2];
    }

    impl MarkThreadEnds for LoomThread {
        fn mark_end(&mut self) -> [V3d; 2] {
            let [[x, y, z], [i, j, k]] = [self[0], self[self.len() - 1]];
            let [front, back] = [[x, y, z + 2], [i, j, k + 2]];
            self.push_front(front);
            self.push_back(back);
            [front, back]
        }
    }
}

pub mod mirror_loom {
    use crate::graph::defs::{Loom, LoomThread, Tour, V3d};
    use rayon::prelude::*;

    pub trait Mirrored {
        /// For each thread in the loom all of whose ends are not adjacent, reflect each thread turning chains into cycles.
        /// Imagine reflecting a row of arcs to form a row of ovals.
        fn mirror_threads(&mut self);
    }

    impl Mirrored for Loom {
        fn mirror_threads(&mut self) {
            self.par_iter_mut().for_each(|thread| {
                thread.extend(
                    thread
                        .rev_iter()
                        .map(|&[x, y, z]| [x, y, -z])
                        .collect::<Tour>(),
                )
            });
        }
    }

    pub trait ReverseIterator {
        /// A convenience reverse iterator for `vec_deque`.
        fn rev_iter(&self) -> std::iter::Rev<std::collections::vec_deque::Iter<'_, V3d>>;
    }

    impl ReverseIterator for LoomThread {
        fn rev_iter(&self) -> std::iter::Rev<std::collections::vec_deque::Iter<'_, V3d>> {
            self.iter().rev()
        }
    }
}

pub mod merge_cycle {
    use super::graph_info_from_n::InfoN;
    use crate::graph::defs::*;
    use common_macros::hash_set;
    use itertools::Itertools;

    pub trait GetWeftWarps {
        /// Remove the threads from the loom and separate into weft and warps. Prepare for cycle merging. Loop over each warp, join with the weft until the end of the loop where there will only be the weft.
        fn prepare_merging(self, n: usize) -> (Weft, Warps);
    }

    impl GetWeftWarps for Loom {
        fn prepare_merging(mut self, n: usize) -> (Weft, Warps) {
            (
                Weft::new(self[0].split_off(0), n.get_order_from_n()),
                Warps::drain_from(self),
            )
        }
    }

    pub trait DrainFrom<T> {
        /// Remove the individual subcycles from the loom and convert into a Vec and ready for joining with the weft.
        fn drain_from(t: T) -> Self;
    }

    impl DrainFrom<Loom> for Warps {
        fn drain_from(mut loom: Loom) -> Warps {
            loom.split_off(1)
                .into_iter()
                .map(|mut data| data.drain(..).collect())
                .collect()
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

        /// Convert data into set of edges using `circular_tuple_windows()` and filter using `is_valid_bridge_edge()`.
        pub fn edges(&mut self) -> Edges {
            self.data
                .iter()
                .circular_tuple_windows()
                .filter(|(&[_, _, z], &[a, _, c])| {
                    a == (if self.joined { 1 } else { 3 }) && (z + c).absbit() == self.max_sum_z
                })
                .map(|(a, b)| (*a, *b).orient())
                .collect()
        }

        /// Join the warp with the weft. The bridges change once the warp has been joined once.
        pub fn join(&mut self, warp: &mut Warp) {
            self.data.append(warp);
            if !self.joined {
                self.joined = true;
                self.max_abs_z -= 2;
            } else {
                self.max_abs_z -= 4;
            }
            self.max_sum_z = self.max_abs_z * 2 - 2;
        }

        /// Retrieve the finished solution.
        pub fn get_woven(&self) -> Solution {
            self.data.to_vec()
        }
    }

    pub trait WarpEdges {
        /// Construct edges from the Vec and filter.
        /// 'vec![1, 2, 3]' -> `hash_set![(1, 2), (2, 3), (3, 1)]` which is then filtered further to avoid memory waste.
        fn edges(&self, min_xyz: i16, joined: bool) -> Edges;
    }

    impl WarpEdges for Warp {
        fn edges(&self, max_sum_z: i16, joined: bool) -> Edges {
            self.iter()
                .circular_tuple_windows()
                .filter(|(&[x, y, z], &[_, b, c])| {
                    (x == 3 || x == 1)
                        && (y == 3 || y == 1)
                        && b == (if joined { 1 } else { 3 })
                        && (z + c).absbit() == max_sum_z
                })
                .map(|(a, b)| (*a, *b).orient())
                .collect()
        }
    }

    pub trait AbsBit<T> {
        /// Get the absolute value of an i16 using bitwise operations.
        fn absbit(self) -> T;
    }

    impl AbsBit<i16> for i16 {
        fn absbit(self) -> i16 {
            let mask = self >> 15;
            (self ^ mask) - mask
        }
    }

    pub trait OrientAscending {
        /// Orient edge such that lhs < rhs.
        fn orient(self) -> Edge;
    }

    impl OrientAscending for Edge {
        fn orient(self) -> Edge {
            let (m, n) = self;
            match m < n {
                true => (m, n),
                false => (n, m),
            }
        }
    }

    pub trait Bridge<T> {
        /// Using the & set operator, find the common bridge i.e., intersection between a set of edges and a set of adjacent edges and return the next() from the set.
        fn bridge(&self, other: &T) -> Edge;
    }

    impl Bridge<Edge> for Edges {
        /// Using the & set operator, find the common bridge i.e., intersection between a set of edges and a set of adjacent edges and return the next() from the set.
        /// This version automatically reverses the edge as it is always the case (removed the check).
        fn bridge(&self, (weft_lhs, weft_rhs): &(V3d, V3d)) -> Edge {
            let (warp_lhs, warp_rhs) = (self & &((*weft_lhs, *weft_rhs).get_eadjs()))
                .into_iter()
                .next()
                .unwrap();
            (warp_rhs, warp_lhs)
        }
    }

    impl Bridge<Edges> for Edges {
        fn bridge(&self, other: &Edges) -> Edge {
            (self & &other.get_eadjs()).into_iter().next().unwrap()
        }
    }

    pub trait GetEadjs {
        /// Get the adjacent/parallel edges of edges.
        fn get_eadjs(&self) -> Edges;
    }

    impl GetEadjs for Edges {
        fn get_eadjs(&self) -> Edges {
            self.iter()
                .filter(|([x, y, _], _)| (x == &3 || x == &1) && (y == &3 || y == &1))
                .flat_map(|&(q, r)| (q, r).get_eadj())
                .collect()
        }
    }

    impl GetEadjs for Edge {
        fn get_eadjs(&self) -> Edges {
            let ([a, b, c], [x, y, z]) = *self;
            match (a != x, b != y, c != z) {
                (true, false, false) => hash_set!(([a, b + 2, c], [x, y + 2, z])),
                (false, true, false) => hash_set!(([a + 2, b, c], [x + 2, y, z])),
                (false, false, true) => hash_set!(([a + 2, b, c], [x + 2, y, z])),
                _ => panic!("NOT A VALID EDGE"),
            }
        }
    }

    pub trait GetEadj {
        /// Get and filter the adjacent edges of an edge.
        fn get_eadj(self) -> Edges;
    }

    impl GetEadj for Edge {
        fn get_eadj(self) -> Edges {
            let ([a, b, c], [x, y, z]) = self;
            match (a != x, b != y, c != z) {
                (true, false, false) => hash_set!(([a, b - 2, c], [x, y - 2, z])),
                (false, true, false) => hash_set!(([a, b, c + 2], [x, y, z + 2])),
                (false, false, true) => hash_set!(
                    ([a - 2, b, c], [x - 2, y, z]),
                    ([a, b + 2, c], [x, y + 2, z])
                ),
                _ => panic!("NOT A VALID EDGE"),
            }
        }
    }

    pub trait AlignToEdge {
        /// Align self to given edge such that the lhs of edge and self match and the rhs of edge and self match.
        fn align_to(&mut self, edge: (V3d, V3d));
    }

    impl AlignToEdge for Warp {
        fn align_to(&mut self, (lhs, rhs): (V3d, V3d)) {
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

pub mod check {
    use crate::graph::defs::{Solution, V2d, V3d};
    use itertools::{all, Itertools};

    use std::fmt;

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

pub mod csv_out {
    use crate::graph::defs::Solution;
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

    pub trait SerializeToCsv<T> {
        /// Save solution to `file_path` as a `.csv` file with the columns `x`, `y`, `z` for each axis.\
        /// A python module using `pandas` and `plotly` to create a 3d line plot is available [here](https://github.com/discocube/plot_solution).
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
