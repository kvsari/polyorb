//! # Polyorb
//!
//! Render various Goldberg polyhedrons.

pub mod scene;
pub mod light;
pub mod shader;
pub mod presentation;
pub mod platonic_solid;

pub mod show;
pub mod input;
pub mod shape;
pub mod tetrahedron;

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
