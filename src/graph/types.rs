use ndarray::Array2;
use std::collections::{HashMap, HashSet, VecDeque};

pub type BridgeEdge = Edge;
pub type ColorIdx = u8;
pub type Count = usize;
pub type Counts = Vec<Count>;
pub type Edge = (V3d, V3d);
pub type Edges = HashSet<Edge>;
pub type Loom = Vec<LoomThread>;
pub type LoomThread = VecDeque<V3d>;
pub type LoomSlice<'a> = &'a [V3d];
pub type Node = u32;
pub type Nodes = Vec<u32>;
pub type PinCushion = Vec<V3d>;
pub type ScalarXyz = i16;
pub type SignedIdx = i32;
pub type Solution = Tour;
pub type Spindle = Vec<V2d>;
pub type Spun = HashMap<V2d, bool>;
pub type Subtours = Vec<Tour>;
pub type Tour = Vec<V3d>;
pub type V2d = [ScalarXyz; 2];
pub type V3d = [ScalarXyz; 3];
pub type Vectors = Vec<V3d>;
pub type Verts = [V3d];
pub type Warp = Vec<V3d>;
pub type Warps = Vec<Warp>;
pub type WarpEdges = HashSet<Edge>;
pub type WeftEdge = (V3d, V3d);
pub type WeftEdges = HashSet<WeftEdge>;
pub type Yarn = Array2<ScalarXyz>;
pub type Yarns = HashMap<ColorIdx, Yarn>;
pub type ZrowColorSize = Vec<((ScalarXyz, ColorIdx), Count)>;
