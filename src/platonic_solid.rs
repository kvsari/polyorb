//! The five platonic solids.

use cgmath::{Point3, Vector3, BaseFloat};

use crate::polyhedron::{Polyhedron, VrFc, Seed};
use crate::scene;

mod tetrahedron;
mod cube;
mod octahedron;
mod dodecahedron;
mod icosahedron;

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

macro_rules! platonic2 {
    ($name: ident, $function:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $name {
            side_len: f32,
        }

        impl $name {
            pub fn new(side_len: f32) -> Self {
                $name { side_len }
            }

            pub fn generate(&self) -> Polyhedron<VrFc> {
                $function(self.side_len)
            }
        }

        impl Seed for $name {
            fn polyhedron(&self) -> Polyhedron<VrFc> {
                self.generate()
            }
        }
    }
}

platonic2!(Cube2, cube::cube2);
