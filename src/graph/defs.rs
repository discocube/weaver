use super::utils::certify::SequenceID;
use common_macros::hash_set;
use itertools::{all, Itertools};
use ndarray::{array, s, Array1, Array2};
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    iter::{repeat, zip},
};

pub const LAST_ROW: ScalarXyz = -1;
pub const DISP_VECTORS: [[V2; 2]; 4] = [
    [[-2, 0], [0, -2]],
    [[-2, 0], [0, 2]],
    [[2, 0], [0, 2]],
    [[2, 0], [0, -2]],
];

pub type Adjacency = HashMap<V, Neighbors>;
pub type ZAdjacency = HashMap<[ScalarXyz; 2], Vec<[ScalarXyz; 2]>>;
pub type Cushion = Vec<V>;
pub type Count = usize;
pub type Edge = (V, V);
pub type Edges = HashSet<Edge>;
pub type Loom = Vec<VecDeque<V>>;
pub type LoomThread = VecDeque<V>;
pub type LoomSlice = [VecDeque<V>];
pub type Neighbors = HashSet<V>;
pub type Vs = HashSet<V>;
pub type ScalarXyz = i16;
pub type Points = HashSet<ScalarXyz>;
pub type Solution = Tour;
pub type Spool = Vec<V2>;
pub type Yarns = HashMap<u8, Yarn>;
pub type Spun = HashMap<V2, bool>;
pub type Subtours = Vec<Tour>;
pub type Tour = Vec<V>;
pub type TourSlice<'a> = &'a [V2];
pub type YarnEnds = VecDeque<V>;
pub type V = [ScalarXyz; 3];
pub type V2 = [ScalarXyz; 2];
pub type Vert = (ScalarXyz, ScalarXyz, ScalarXyz);
pub type Verts = [V];
pub type VecVert = Vec<Vert>;
pub type Visited = HashMap<V2, bool>;
pub type VIMap = HashMap<Vert, V>;
pub type Warp = Vec<V>;
pub type Warps = Vec<Vec<V>>;
pub type Weights = HashMap<V, ScalarXyz>;
pub type SignedIdx = i32;
pub type Yarn = Array2<ScalarXyz>;
pub type ZlevelVsMap = HashMap<ScalarXyz, Vs>;
pub type ZOrder = Vec<(ScalarXyz, usize)>;

pub struct Spinner<'a> {
    pub n: usize,
    pub turns: Vec<usize>,
    pub zigzags: std::iter::Cycle<core::slice::Iter<'static, [V2; 2]>>,
    pub yxyx: std::iter::Cycle<core::slice::Iter<'a, usize>>,
}

impl<'a> Spinner<'a> {
    pub fn start(n: usize) -> (Self, (usize, &'static [V2; 2])) {
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

    pub fn turn(&mut self, iyx: usize) -> (usize, &'static [V2; 2]) {
        (
            self.turns.pop().unwrap() - iyx,
            self.zigzags.next().unwrap(),
        )
    }

    fn get_turns(n: usize) -> Vec<usize> {
        let mut result = (-(n as i64 * 2)..=-2)
            .map(|i| -i as usize)
            .step_by(2)
            .flat_map(|cut| [cut, cut])
            .scan(0, |state, n| {
                *state += n;
                Some(*state - 1)
            })
            .collect::<Vec<usize>>();
        result.reverse();
        result
    }

    fn get_zigzags() -> std::iter::Cycle<core::slice::Iter<'static, [V2; 2]>> {
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

pub trait InfoN {
    /// Return a Vec of discocube orders where self is the lower bound and the input is the upper bound.
    fn from_uon_to(self, end: usize) -> Vec<usize>;
    /// Retrieve loom size based on n.
    fn get_loom_size(self) -> Self;
    /// Get the maximum L1-norm for n.
    fn get_max_absumv(self) -> i16;
    /// Get maximum scalar value allowed for x, y or z.
    fn get_max_xyz(self) -> i16;
    /// Get maximum scalar value allowed for x, y or z where n is the order.
    fn get_max_xyz_from_order(self) -> SignedIdx;
    /// Minimum scalar value allowed for x, y, or z.
    fn get_min_xyz(self) -> i16;
    /// get level from order, or n.
    fn get_n_from_order(self) -> Self;
    /// get order from level/n.
    fn get_order_from_n(self) -> Self;
    /// spool size is the number of verts in the graph whose z-scalar value is -1.
    fn get_spool_size(self) -> (usize, usize);
    /// Get length of the current level.
    fn get_zpos_size_color(self) -> Vec<((i16, usize), u8)>;
}

impl InfoN for usize {
    fn from_uon_to(self, end: usize) -> Vec<usize> {
        (0..end + 2)
            .filter_map(move |i| {
                let _uon = (0..2000 * 2 + 2)
                    .step_by(2)
                    .take(i)
                    .map(|n| n * (n + 2))
                    .sum();
                if _uon >= self && _uon <= end {
                    Some(_uon)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_loom_size(self) -> Self {
        (self / 2) + 1
    }
    fn get_max_absumv(self) -> i16 {
        (self * 2 + 1) as i16
    }

    fn get_max_xyz(self) -> i16 {
        (self * 2 - 1) as i16
    }

    fn get_max_xyz_from_order(self) -> SignedIdx {
        (((3 * self) / 4) as f64).powf(1.0 / 3.0) as i32 * 2 - 1
    }

    fn get_min_xyz(self) -> i16 {
        (self * 2 - 5) as i16
    }

    fn get_n_from_order(self) -> Self {
        (((3 * self) / 4) as f64).powf(1.0 / 3.0) as usize
    }

    fn get_order_from_n(self) -> Self {
        (4 * (self + 2) * (self + 1) * self) / 3
    }

    fn get_spool_size(self) -> (usize, usize) {
        (self, 2 * self * (self + 1))
    }

    fn get_zpos_size_color(self) -> Vec<((i16, usize), u8)> {
        let spool_length = 2 * self * (self + 1);
        zip(
            zip(
                (-((self * 2 - 1) as i16)..=-1).step_by(2),
                (1..=self).map(|_n| spool_length - (2 * _n * (_n + 1))),
            ),
            match self % 2 {
                0 => repeat(1).take(self).interleave(repeat(3).take(self)),
                _ => repeat(3).take(self).interleave(repeat(1).take(self)),
            },
        )
        .collect()
    }
}

/// Weft is the main loop into which the warps are incorporated.
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
    pub fn new(mut data: YarnEnds, order: usize) -> Weft {
        let mut preallocated = Vec::with_capacity(order);
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
                a == (if self.joined { 1 } else { 3 }) && (z + c).bitabs() == self.max_sum_z
            })
            .map(|(a, b)| (*a, *b).orient())
            .collect()
    }

    /// Join the warp with the weft.
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

pub trait ReverseIterator<T> {
    /// A convenience reverse iterator for `vec_deque`.
    fn rev_iter(&self) -> std::iter::Rev<std::collections::vec_deque::Iter<'_, T>>;
}

impl<T> ReverseIterator<T> for VecDeque<T> {
    fn rev_iter(&self) -> std::iter::Rev<std::collections::vec_deque::Iter<'_, T>> {
        self.iter().rev()
    }
}

pub trait ReverseIteratorSlice<T> {
    /// A convenience reverse iterator for `slice`.
    fn rev_iter(&self) -> std::iter::Rev<std::slice::Iter<'_, T>>;
}

impl ReverseIteratorSlice<V> for &[V] {
    fn rev_iter(&self) -> std::iter::Rev<std::slice::Iter<'_, V>> {
        self.iter().rev()
    }
}

pub trait ReversedVec<T> {
    /// Reverse, clone & collect a slice to `Vec`: `self.iter().rev().cloned().collect_vec()`
    fn to_reversed_vec(&self) -> Vec<V>;
}

impl ReversedVec<V> for &[V] {
    fn to_reversed_vec(&self) -> Vec<V> {
        self.iter().rev().cloned().collect()
    }
}

pub trait DequeExtEnds<T> {
    /// Insert pins into each end of each thread in the loom. A pin is the vertex adjacent to and directly above an end. Collect the a copy of all inserted pins to be used for cutting the finished yarn from the next level up.
    fn insert_pins(&mut self) -> [V; 2];
}

impl DequeExtEnds<V> for VecDeque<V> {
    fn insert_pins(&mut self) -> [V; 2] {
        let [[x, y, z], [i, j, k]] = [self[0], self[self.len() - 1]];
        let [front, back] = [[x, y, z + 2], [i, j, k + 2]];
        self.push_front(front);
        self.push_back(back);
        [front, back]
    }
}

pub trait ExtendFront<T> {
    /// Extend vec to the front of the deque by pushing each element using `push_front` but skipping the first. Named ExtendFront to disambiguate with ExtendLeft, which usually involves reversing the sequence before extending.
    fn extend_front(&mut self, vec: &mut Vec<T>);
}

impl<T> ExtendFront<T> for VecDeque<T> {
    fn extend_front(&mut self, vec: &mut Vec<T>) {
        vec.drain(..).skip(1).for_each(|item| self.push_front(item));
    }
}

pub trait ExtendBack<T> {
    /// Extend vec to the back of the deque by pushing each element using `push_back` but skipping the first. Named ExtendFront to disambiguate with ExtendLeft, which usually involves reversing the sequence before extending.
    fn extend_back(&mut self, vec: &mut Vec<T>);
}

impl<T> ExtendBack<T> for VecDeque<T> {
    fn extend_back(&mut self, vec: &mut Vec<T>) {
        self.extend(vec.drain(..).skip(1));
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

/// The sum of the absolute value of each scalar x, y and or z.
/// Also known as the L1-Norm- manhattan distance from a to b where b is the origin (0, 0, 0)
/// The L1 norm is calculated as the sum of the absolute vector values, where the absolute value of a scalar uses the notation |a1|.
/// In effect, the norm is a calculation of the Manhattan distance from the origin of the vector space.
pub trait AbSumV {
    /// Calculate the L1-norm for a vector.
    fn absumv(&self) -> i16;
}

impl AbSumV for V {
    fn absumv(&self) -> i16 {
        self.iter()
            .map(|v| {
                let mask = v >> 15;
                (v ^ mask) - mask
            })
            .sum()
    }
}

impl AbSumV for Array1<i16> {
    fn absumv(&self) -> i16 {
        self.iter()
            .map(|v| {
                let mask = *v >> 15;
                (*v ^ mask) - mask
            })
            .sum()
    }
}

impl AbSumV for V2 {
    fn absumv(&self) -> i16 {
        self.iter()
            .map(|v| {
                let mask = v >> 15;
                (v ^ mask) - mask
            })
            .sum()
    }
}

impl AbSumV for (i16, i16, i16) {
    fn absumv(&self) -> i16 {
        let (x, y, z) = self;
        let mask_x = x >> 15;
        let mask_y = y >> 15;
        let mask_z = z >> 15;
        (x ^ mask_x) - mask_x + (y ^ mask_y) - mask_y + (z ^ mask_z) - mask_z
    }
}

impl AbSumV for (i16, i16) {
    fn absumv(&self) -> i16 {
        let (x, y) = self;
        let mask_x = x >> 15;
        let mask_y = y >> 15;
        (x ^ mask_x) - mask_x + (y ^ mask_y) - mask_y
    }
}

pub trait AbsOf<T> {
    /// Get the absolute value of an i16 using bitwise operations.
    fn bitabs(self) -> i16;
}

impl AbsOf<i16> for i16 {
    fn bitabs(self) -> i16 {
        let mask = self >> 15;
        (self ^ mask) - mask
    }
}

pub trait AddVec {
    // add two 2-dimensional vectors together.
    fn add_vec(&self, other: V2) -> V2;
}

impl AddVec for V2 {
    fn add_vec(&self, [a, b]: V2) -> V2 {
        [self[0] + a, self[1] + b]
    }
}

pub trait Bridge<T> {
    /// Using the & set operator, find the common bridge i.e., intersection between a set of edges and a set of adjacent edges and return the next() from the set.
    fn bridge(&self, other: &T) -> Edge;
}

impl Bridge<Edge> for HashSet<Edge> {
    /// Using the & set operator, find the common bridge i.e., intersection between a set of edges and a set of adjacent edges and return the next() from the set.
    /// This version automatically reverses the edge as it is always the case (removed the check).
    fn bridge(&self, (weft_lhs, weft_rhs): &(V, V)) -> Edge {
        let (warp_lhs, warp_rhs) = (self & &((*weft_lhs, *weft_rhs).get_eadjs()))
            .into_iter()
            .next()
            .unwrap();
        (warp_rhs, warp_lhs)
    }
}

impl Bridge<HashSet<Edge>> for HashSet<Edge> {
    fn bridge(&self, other: &HashSet<Edge>) -> Edge {
        (self & &other.eadjs()).into_iter().next().unwrap()
    }
}

pub trait MakeEadjs<T> {
    /// Get the adjacent/parallel edges of an edge.
    fn get_eadjs(self) -> Edges;
}

impl MakeEadjs<Edges> for Edge {
    fn get_eadjs(self) -> Edges {
        let ([a, b, c], [x, y, z]) = self;
        match (a != x, b != y, c != z) {
            (true, false, false) => hash_set!(([a + 0, b + 2, c + 0], [x + 0, y + 2, z + 0])),
            (false, true, false) => hash_set!(([a + 2, b + 0, c + 0], [x + 2, y + 0, z + 0])),
            (false, false, true) => hash_set!(([a + 2, b + 0, c + 0], [x + 2, y + 0, z + 0])),
            _ => panic!("NOT A VALID EDGE"),
        }
    }
}

pub trait AdjEdges<T> {
    /// Get the adjacent/parallel edges of an other edge.
    fn eadjs(&self) -> Edges;
}

impl AdjEdges<Edge> for Edges {
    fn eadjs(&self) -> Edges {
        self.iter()
            .filter(|([x, y, _], _)| (x == &3 || x == &1) && (y == &3 || y == &1))
            .flat_map(|&(q, r)| (q, r).get_edges())
            .collect()
    }
}

pub trait MakeEdges<T> {
    /// Get and filter the adjacent edges of an edge. Needs disambiguation.
    fn get_edges(self) -> Edges;
}

impl MakeEdges<Edges> for Edge {
    fn get_edges(self) -> Edges {
        let ([a, b, c], [x, y, z]) = self;
        match (a != x, b != y, c != z) {
            (true, false, false) => [[0, 2, 0], [0, -2, 0]],
            (false, true, false) => [[2, 0, 0], [0, 0, 2]],
            (false, false, true) => [[-2, 0, 0], [0, 2, 0]],
            _ => panic!("NOT A VALID EDGE"),
        }
        .par_iter()
        .filter_map(|[i, j, k]| {
            let ([x, y, z], [a, b, c]) = ([a + i, b + j, c + k], [x + i, y + j, z + k]);
            match x == b && b == 1 {
                true => Some(([x, y, z], [a, b, c])),
                false => None,
            }
        })
        .collect()
    }
}

pub trait ExtendThreads<T> {
    /// Extend each end of each thread in the loom with the segmented, colored and finished yarn.
    fn extend_threads(
        &mut self,
        zrow: i16,
        color: u8,
        size: usize,
        cushion: &Cushion,
        yarns: &Yarns,
    );
}

impl ExtendThreads<Loom> for Loom {
    fn extend_threads(
        &mut self,
        zrow: i16,
        color: u8,
        size: usize,
        cushion: &Cushion,
        yarns: &Yarns,
    ) {
        let mut warps = Warps::prepare(zrow, color, size, cushion, yarns);
        self.iter_mut().for_each(|thread: &mut LoomThread| {
            for warp in warps.iter_mut().filter(|w| !w.is_empty()) {
                match (thread[0] == warp[0], thread[thread.len() - 1] == warp[0]) {
                    (true, _) => thread.extend_front(warp),
                    (_, true) => thread.extend_back(warp),
                    _ => continue,
                }
            }
        });
        warps.iter_mut().filter(|s| !s.is_empty()).for_each(|seq| {
            self.push(seq.drain(..).collect::<YarnEnds>());
        });
    }
}

pub trait Mirrored<T> {
    /// For each thread in the loom all of whose ends are not adjacent, reflect each thread turning chains into cycles.
    /// Imagine reflecting a row of arcs to form a row of ovals.
    fn mirror_threads(&mut self);
}

impl Mirrored<Loom> for Loom {
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

pub trait GetEdges<T> {
    /// Construct edges from the Vec and filter. 
    /// 'vec![1, 2, 3]' -> `hash_set![(1, 2), (2, 3), (3, 1)]` which is then filtered further to avoid memory waste.
    fn edges(&self, min_xyz: i16, joined: bool) -> Edges;
}

impl GetEdges<Edges> for Vec<V> {
    fn edges(&self, max_sum_z: i16, joined: bool) -> Edges {
        self.iter()
            .circular_tuple_windows()
            .filter(|(&[x, y, z], &[_, b, c])| {
                (x == 3 || x == 1)
                    && (y == 3 || y == 1)
                    && b == (if joined { 1 } else { 3 })
                    && (z + c).bitabs() == max_sum_z
            })
            .map(|(a, b)| (*a, *b).orient())
            .collect()
    }
}

pub trait IsAdjacent {
    /// Check if self is adjacent to another vertex.
    fn is_adj_to(&self, other: V) -> bool;
}

impl IsAdjacent for V {
    fn is_adj_to(&self, [x, y, z]: V) -> bool {
        let [a, b, c] = self;
        let n = a - x + b - y + c - z;
        n == 2 || n == -2
    }
}

pub trait AlignToEdge<T> {
    /// Align self to given edge such that the ends lhs of the edge and self match and the rhs of the edge and self match.
    fn align_to(&mut self, edge: (V, V));
}

impl AlignToEdge<Vec<V>> for Vec<V> {
    fn align_to(&mut self, (lhs, rhs): (V, V)) {
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

pub trait OrientAscending<Edge> {
    /// Orient edge such that lhs < rhs.
    fn orient(self) -> Edge;
}

impl OrientAscending<Edge> for Edge {
    fn orient(self) -> Edge {
        let (m, n) = self;
        match m < n {
            true => (m, n),
            false => (n, m),
        }
    }
}

pub trait CutYarn<T> {
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
    fn cut_using(self, pins: &Cushion) -> Warps;
}

impl CutYarn<Warp> for Warp {
    fn cut_using(self, pins: &Cushion) -> Warps {
        let mut warps: Warps = Warps::with_capacity(pins.len() + 1);
        let last_iyarn: usize = self.len() - 1;
        let last_ipin: usize = pins.len() - 1;
        let mut iprev: usize = 0;
        self.iter()
            .enumerate()
            .filter_map(|(idx, vec)| (pins.contains(vec)).then_some(idx))
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

pub trait PinEnds<T> {
    /// 📌 For each thread end `[thread[0], thread[thread.len() - 1]]` in the loom make a pin by adding 2 (length of an edge) to the z-scalar value: `[x, y, z + 2]`. Collect the pins in the cushion for cutting later.
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
    fn pin_ends(&mut self, last: bool) -> Cushion;
}

impl PinEnds<Cushion> for Loom {
    fn pin_ends(&mut self, last: bool) -> Cushion {
        match last {
            false => self
                .iter_mut()
                .flat_map(|thread| thread.insert_pins())
                .collect(),
            true => Cushion::with_capacity(0),
        }
    }
}

pub trait Spin<T> {
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
    fn spin(n_spool_size: (usize, usize)) -> Spool;
}

impl Spin<Spool> for Spool {
    fn spin((n, spool_size): (usize, usize)) -> Spool {
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

pub trait Convert<T> {
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
    fn colorized(spool: Spool) -> Yarns;
}

impl Convert<Spool> for Yarns {
    fn colorized(mut spool: Spool) -> Yarns {
        let blue: Yarn = Yarn::from(spool.drain(..).collect::<Vec<_>>());
        let red: Yarn = blue.dot(&array![[-1, 0], [0, -1]]) + array![[0, 2]];
        Yarns::from([(3, blue), (1, red)])
    }
}

pub trait GetWeftWarps<T> {
    /// Remove the threads from the loom and separate into weft, the lead and first thread in the loom and warps which is the rest.
    fn detach_weft(self, n: usize) -> (Weft, Warps);
}

impl GetWeftWarps<Loom> for Loom {
    fn detach_weft(mut self, n: usize) -> (Weft, Warps) {
        (
            Weft::new(self[0].split_off(0), n.get_order_from_n()),
            Warps::drain_from(self),
        )
    }
}

pub trait Prepare<Warps> {
    /// 👨‍🍳 Measure requested color and size as a slice and finish by positioning the yarn to the current elevation by adding a scalar `zpos` to each item in the slice. Cut finished yarn if there are pins in the pins by calling `cut_yarn(_yarn, pins)` or return uncut.
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
    fn prepare(zpos: i16, color: u8, size: usize, pins: &Cushion, yarns: &Yarns) -> Warps;
}

impl Prepare<Warps> for Warps {
    fn prepare(zpos: i16, color: u8, size: usize, pins: &Cushion, yarns: &Yarns) -> Warps {
        match yarns[&color]
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

pub trait Certify<T> {
    /// Certify if a sequence is a Hamiltonian chain by checking for duplicates, the length of the solution is equal to the order, check that each V is adjacent to the other...
    fn certify(&self, order: usize, max_absumv3d: i16) -> SequenceID;
}

impl Certify<Solution> for Solution {
    fn certify(&self, order: usize, max_absumv3d: i16) -> SequenceID {
        if self.iter().duplicates().count() > 0  // Check for duplicates
        || self.len() != order  // CHECK THAT SOLUTION LENGTH == ORDER
        || !all(self.iter(), |vert| vert.absumv() <= max_absumv3d)  // CHECK IF ANY OF THE VERTS ARE OUTSIDE THE FURTHEST VERTS
        || self // CHECK IF THE SUM OF ALL THE VECTORS ARE EQUAL TO ZERO.
            .iter()
            .fold((0, 0, 0), |acc: (i64, i64, i64), &[x, y, z]| {
                (acc.0 + x as i64, acc.1 + y as i64, acc.2 + z as i64)
            })
            != (0, 0, 0)
        {
            return SequenceID::Broken;
        }
        match self.windows(2).all(|window| window[0].is_adj_to(window[1])) {
            // CHECK EACH EDGE TO SEE IF THEY ARE ADJACENT
            true if self[self.len() - 1].is_adj_to(self[0]) => SequenceID::HamCycle, // ENDS MATCH
            true => SequenceID::HamChain, // EVERYTHING ELSE BUT ENDS IS ADJACENT
            false => SequenceID::Broken,
        }
    }
}

pub trait GetUon<T> {
    /// Get a Vec of orders from self where lhs of self is the lower bound and rhs of self is the upper bound.
    fn get_uon(self) -> Vec<usize>;
}

impl GetUon<(usize, usize)> for (usize, usize) {
    fn get_uon(self) -> Vec<usize> {
        let (start, end) = self;
        (0..2000 + 2)
            .filter_map(move |i| {
                let _uon = (0..2000 * 2 + 2)
                    .step_by(2)
                    .take(i)
                    .map(|n| n * (n + 2))
                    .sum();
                if _uon >= start && _uon <= end {
                    Some(_uon)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Simple 3-dimensional vector struct for serializing points to csv.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CsvVector {
    x: i16,
    y: i16,
    z: i16,
}

pub trait SerializeToCsv<T> {
    /// Save solution to `file_path` as a `.csv` file with the columns `x`, `y`, `z` for each axis.\
    /// A python module using `pandas` and `plotly` to create a 3d line plot is available [here](https://github.com/discocube/plot_solution).
    fn serialize_to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
}

impl SerializeToCsv<Result<(), Box<dyn Error>>> for Vec<V> {
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
