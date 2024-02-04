

use std::fmt;
use std::cmp::Ordering;
#[derive(Debug, Clone)]
pub struct PathNode {
    pub index: usize,
    pub distance: f32,
    pub seen : bool,
    pub explored : bool,
    pub predecessor: usize,
}

impl fmt::Display for PathNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.index)
    }
}
impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for PathNode {}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}
