//! Space/Spatial types and operations
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vertex {
    position: [f32; 4],
    //normal: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 4]) -> Self {
        Vertex { position }
    }
}

impl Vertex {
    pub const fn sizeof() -> usize {
        mem::size_of::<Vertex>()
    }
}
