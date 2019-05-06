//! Space/Spatial types and operations

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vertex {
    position: [f32; 4],
    normal: [f32; 4],
}
