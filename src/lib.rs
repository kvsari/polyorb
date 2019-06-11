//! # Polyorb
//!
//! Render various Goldberg polyhedrons.

pub mod show;
pub mod input;
//pub mod keyboard;
//pub mod cube;
//pub mod space;
pub mod shape;
pub mod tetrahedron;

//pub mod triangle_example;
//pub mod cube_example;

pub use self::show::run;

/// Platonic solid that is used as a base for the `GoldberPolyhedron`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlatonicSolid {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}


/*
/// The main data structure of the library.
pub struct GoldbergPolyhedron {
    base: PlatonicSolid,
    
}
*/
