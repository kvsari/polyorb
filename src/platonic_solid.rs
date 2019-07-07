//! The five platonic solids.

use cgmath::{Point3, Vector3, BaseFloat};

use crate::scene;

mod tetrahedron;
mod cube;
mod octahedron;
mod dodecahedron;
mod icosahedron;

//use self::tetrahedron::Tetrahedron;

/// Made private so as not to clash with `scene::Vertex`.
#[derive(Debug, Clone)]
struct Vertex<S: BaseFloat> {
    position: Point3<S>,
    normal: Vector3<S>,
    colour: [f32; 3],
}

impl<S: BaseFloat> Vertex<S> {
    fn new(position: Point3<S>, normal: Vector3<S>, colour: [f32; 3]) -> Self {
        Vertex { position, normal, colour }
    }
}

#[derive(Debug, Clone)]
pub struct Cached<S: BaseFloat> {
    vertices: Vec<Vertex<S>>,
    index: Vec<u16>,
}

impl<S: BaseFloat> Cached<S> {
    fn new(vertices: &[Vertex<S>], index: &[u16]) -> Self {
        Cached {
            vertices: vertices.to_owned(),
            index: index.to_owned(),
        }
    }
}

impl scene::Geometry for Cached<f32> {
    fn geometry(&self) -> (Vec<scene::Vertex>, Vec<u16>) {
        (
            self.vertices
                .iter()
                .map(|v| scene::Vertex::new(
                    [v.position.x, v.position.y, v.position.z],
                    [v.normal.x, v.normal.y, v.normal.z],
                    v.colour
                ))
                .collect(),
            
            self.index.to_owned()
        )
    }
}

/// Produce the golden ratio of 1.6180339887...
/// Why not just a constant? Why not constant function? Because rust hasn't yet made sqrt
/// a const function. I don't know why. It's a maths function. It should be easy.
pub fn golden_ratio_f32() -> f32 {
    (1.0 + 5f32.sqrt()) / 2.0
}

fn triangle_normal<S: BaseFloat>(
    p1: Point3<S>, p2: Point3<S>, p3: Point3<S>
) -> Vector3<S> {
    let v1 = p1.to_homogeneous().truncate();
    let v2 = p2.to_homogeneous().truncate();
    let v3 = p3.to_homogeneous().truncate();

    let v = v2 - v1;
    let w = v3 - v1;

    v.cross(w) // Normal gets normalized in the shader
}

macro_rules! platonic {
    ($name:ident, $function:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $name {
            side_len: f32,
            colour: [f32; 3],
        }

        impl $name {
            pub fn new(side_len: f32, colour: [f32; 3]) -> Self {
                $name { side_len, colour }
            }

            pub fn generate(&self) -> Cached<f32> {
                let (vertices, index) = $function(self.side_len, self.colour);
                Cached::new(&vertices, &index)
            }
        }

        impl scene::Geometry for $name {
            fn geometry(&self) -> (Vec<scene::Vertex>, Vec<u16>) {
                self.generate()
                    .geometry()
            }
        }
    };
}

platonic!(Tetrahedron, tetrahedron::tetrahedron);
platonic!(Cube, cube::cube);
platonic!(Octahedron, octahedron::octahedron);
platonic!(Dodecahedron, dodecahedron::dodecahedron);
platonic!(Icosahedron, icosahedron::icosahedron);
