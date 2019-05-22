//! # Polyorb
//!
//! Render various Goldberg polyhedrons.
use std::{path, io, fs};

use log::info;
use wgpu::winit::{self, Event};
use shaderc::ShaderKind;

pub mod scene;
//pub mod cube;
//pub mod space;
pub mod tetrahedron;

pub mod triangle_example;
pub mod cube_example;

pub use self::scene::run;

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
