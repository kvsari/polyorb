//! Polyhedron building
//!
//! Polyhedron are build using [Conway Notation](https://en.wikipedia.org/wiki/Conway_polyhedron_notation). A seed value is starts
//! the polydron with various modifiers being chained on. A seed shape is usually a
//! [platonic solid](https://en.wikipedia.org/wiki/Platonic_solid).
use std::{fmt, error};

use cgmath::{Point3, Vector3, BaseFloat};

trait Operation {
    fn operate(&self) -> Polyhedron<VrFc>;
}

/// Starts a polyhedron process. `objekt::Clone` means any implementor must derive
/// `std::clone::Clone`.
pub trait Seed: objekt::Clone + fmt::Debug {
    fn polyhedron(&self) -> Polyhedron<VrFc>;
}

objekt::clone_trait_object!(Seed);

impl Operation for Seed {
    fn operate(&self) -> Polyhedron<VrFc> {
        self.polyhedron()
    }
}

#[derive(Debug, Clone)]
enum ConwayOperation {
    Seed(Box<dyn Seed>),
    Dual,
}

/// A `Polyhedron` defined as a `Seed` and an optional series of `ConwayOperation`s.
#[derive(Debug, Clone)]
pub struct ConwayDescription {
    notation: Vec<ConwayOperation>,
}

impl ConwayDescription {
    pub fn new() -> Self {
        ConwayDescription {
            notation: Vec::new(),
        }
    }

    pub fn seed<S: Seed + Copy>(&mut self, seed: S) -> Result<&mut Self, OpError> {

        Ok(self)
    }

    /*
    pub fn dual(&mut self) -> Result<&mut Self, OpError> {

        Ok(self)
    }
    */
}

/*
/// An edge defined by two points.
#[derive(Debug, Copy, Clone)]
pub struct Edge<S: BaseFloat> {
    start: Point3<S>,
    end: Point3<S>,
}

/// A face of a `Polyhedron`. Vertices are arranged counter clockwise.
#[derive(Debug, Clone)]
pub struct Face<S: BaseFloat> {
    vertices: Vec<Point3<S>>,
    normal: Vector3<S>,
    centroid: Point3<S>,
}
 */

/// Vertices and Faces. Inner state type for a `Polyhedron`. Not directly constructable.
pub struct VrFc {
    center: Point3<f32>,
    vertices: Vec<Point3<f32>>,
    faces: Vec<Vec<usize>>,
}

/// The faces, vertices and edges that make up a polyhedron.
#[derive(Debug, Clone)]
pub struct Polyhedron<T> {
    data: T,
}

impl Polyhedron<VrFc> {
    pub fn new(
        center: Point3<f32>, vertices: &[Point3<f32>], faces: &[&[usize]],
    ) -> Self {
        Polyhedron {
            data: VrFc {
                center,
                vertices: vertices.to_owned(),
                faces: faces
                    .iter()
                    .map(|f| f.to_vec())
                    .collect(),
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OpError {
    AlreadyHasSeed,
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation rejected: {}", match self {
            OpError::AlreadyHasSeed => "Seed already present.",
        })
    }
}

impl error::Error for OpError {
    fn description(&self) -> &str {
        "Error adding Conway operation."
    }
}
