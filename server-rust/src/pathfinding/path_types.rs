
use std::fmt;

#[derive(Debug)]
pub struct PathNode {
    pub index: usize,
    pub x: f32,
    pub y: f32,
    pub seen: bool,
    pub explored: bool,
    pub distance: f32,
}

impl fmt::Display for PathNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.index)
    }
}