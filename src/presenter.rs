//! Prepare a `Polyhedron` for presentation.

use crate::polyhedron::{Polyhedron, VtFc, VtFcNm};
use crate::scene::{Vertex, Geometry};

#[derive(Debug, Clone)]
pub struct SingleColour {
    colour: [f32; 3],
    polyhedron: Polyhedron<VtFcNm>,
}

impl SingleColour {
    pub fn new(colour: [f32; 3], polyhedron: Polyhedron<VtFc>) -> Self {
        SingleColour {
            colour,
            polyhedron: polyhedron.normalize(),
        }
    }
}

impl Geometry for SingleColour {
    fn geometry(&self) -> (Vec<Vertex>, Vec<u16>) {
        (Vec::new(), Vec::new())
    }
}
