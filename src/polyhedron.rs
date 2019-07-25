//! Polyhedron building
//!
//! Polyhedron are build using [Conway Notation](https://en.wikipedia.org/wiki/Conway_polyhedron_notation). A seed value is starts
//! the polydron with various modifiers being chained on. A seed shape is usually a
//! [platonic solid](https://en.wikipedia.org/wiki/Platonic_solid).
use std::{fmt, error};

use cgmath::{Point3, Vector3};

use crate::geop;
use crate::planar;

#[derive(Debug, Copy, Clone)]
pub enum SeedSolid {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl SeedSolid {
    pub fn conway_notation(&self) -> &str {
        match self {
            SeedSolid::Tetrahedron  => "T",
            SeedSolid::Cube         => "C",
            SeedSolid::Octahedron   => "O",
            SeedSolid::Dodecahedron => "D",
            SeedSolid::Icosahedron  => "I",
        }
    }
}

/// Starts a polyhedron process. `objekt::Clone` means any implementor must derive
/// `std::clone::Clone`.
pub trait Seed: objekt::Clone + fmt::Debug {
    fn solid(&self) -> SeedSolid;
    fn polyhedron(&self) -> Polyhedron<VtFc>;
}

objekt::clone_trait_object!(Seed);

#[derive(Debug, Clone)]
enum ConwayOperation {
    Seed(SeedSolid, Polyhedron<VtFc>),
    Dual,
}

/// A polyhedron ready to be built. This struct is not to be modified.
///
/// Tried to make this a recursive sequence of boxed functions calling each other but I
/// couldn't figure out how to do it. Will try again later as my trait foo gets better.
/// Will now have to do it as a luddite loop (fold) instead of cool recursion.
#[derive(Debug, Clone)]
pub struct Specification {
    notation: String,
    operations: Vec<ConwayOperation>,
}

impl Specification {
    fn new(operations: &[ConwayOperation]) -> Self {
        let notation: String = operations
            .iter()
            .rfold(String::new(), |mut ops, op| -> String {
                ops.push_str(match op {
                    ConwayOperation::Seed(ss, _) => ss.conway_notation(),
                    ConwayOperation::Dual => "d",
                });
                
                ops
            });
        
        Specification {
            notation,
            operations: operations.to_owned(),
        }
    }

    pub fn notation(&self) -> &str {
        &self.notation
    }

    pub fn produce(&self) -> Polyhedron<VtFc> {
        let seed = match &self.operations[0] {
            ConwayOperation::Seed(_, p) => p.clone(),
            _ => panic!("Specification must start with a seed."),
        };        
        
        self.operations
            .iter()
            .skip(1)
            .fold(seed, |p, op| match op {
                ConwayOperation::Dual => {
                    let p = p.rising_centroidize();
                    let vertex_face_members = p.faces_per_vertex();

                    // FOR EACH `vertex_face_members`

                    // Use the face indices to lookup the centroids

                    // Use the centroids to define a new face

                    // Make a copy of the vertex, vectorize and normalize it.

                    // Calculate the plane of that face using a point and the normal.

                    // Calculate the intersection of the vertex as a vector with the plane

                    // The intersection point is our centroid of the new face

                    // Sort the vertices of the new face in clockwize direction using
                    // then new normal and the new centroid.

                    p.downgrade()
                },
                _ => panic!("Second seed snuck in."),
            })
    }
}

/// A `Polyhedron` defined as a `Seed` and an optional series of `ConwayOperation`s.
#[derive(Debug, Clone)]
pub struct ConwayDescription {
    operations: Vec<ConwayOperation>,
}

impl ConwayDescription {
    pub fn new() -> Self {
        ConwayDescription {
            operations: Vec::new(),
        }
    }

    pub fn seed<S: Seed>(mut self, seed: &S) -> Result<Self, OpError> {
        if !self.operations.is_empty() {
            Err(OpError::AlreadyHasSeed)
        } else {
            self.operations.push(ConwayOperation::Seed(seed.solid(), seed.polyhedron()));
            Ok(self)
        }
    }

    pub fn dual(&mut self) -> Result<&mut Self, OpError> {
        if self.operations.is_empty() {
            Err(OpError::NoSeedSet)
        } else {
            self.operations.push(ConwayOperation::Dual);
            Ok(self)
        }
    }

    pub fn emit(&self) -> Result<Specification, OpError> {
        if self.operations.is_empty() {
            return Err(OpError::NoOperations);
        }
        
        Ok(Specification::new(&self.operations))
    }
}

pub trait VertexAndFaceOps {
    fn vertices_and_faces(&self) -> (&[Point3<f32>], &[Vec<usize>]);

    /// Return the index for each vertex attached with the indexes for each face a
    /// vertex is part of.
    fn faces_per_vertex(&self) -> Vec<(usize, Vec<usize>)> {
        let (points, faces) = self.vertices_and_faces();

        points
            .iter()
            .enumerate()
            .map(|(i, _p)| {
                let f_v: Vec<usize> = faces
                    .iter()
                    .fold(Vec::new(), |mut v, face_indices| -> Vec<usize> {
                        v.extend(
                            face_indices
                                .iter()
                                .filter(|x| **x == i)
                        );

                        v
                    });
                
                (i, f_v)
            })
            .collect()
    }
}

/// Vertices and Faces. Inner state type for a `Polyhedron`. Not directly constructable.
/// All faces are guaranteed to have three or more vertices.
#[derive(Debug, Clone)]
pub struct VtFc {
    center: Point3<f32>,
    vertices: Vec<Point3<f32>>,
    faces: Vec<Vec<usize>>,
}

/// Add the centroid for each face.
#[derive(Debug, Clone)]
pub struct VtFcCt {
    center: Point3<f32>,
    vertices: Vec<Point3<f32>>,
    faces: Vec<Vec<usize>>,
    centroids: Vec<Point3<f32>>,
}

/// Add the normals. Vector of normals and faces are parallel.
#[derive(Debug, Clone)]
pub struct VtFcNm {
    center: Point3<f32>,
    vertices: Vec<Point3<f32>>,
    faces: Vec<Vec<usize>>,
    normals: Vec<Vector3<f32>>,
}

/// The faces, vertices and edges that make up a polyhedron.
#[derive(Debug, Clone)]
pub struct Polyhedron<T> {
    data: T,
}

impl Polyhedron<VtFc> {
    pub fn new(
        center: Point3<f32>, vertices: &[Point3<f32>], faces: &[&[usize]],
    ) -> Self {
        Polyhedron {
            data: VtFc {
                center,
                vertices: vertices.to_owned(),
                faces: faces
                    .iter()
                    .map(|f| f.to_vec())
                    .collect(),
            },
        }
    }

    /// Calculate the normal for each face and emit a `Polyhedron` with that information
    /// saved consuming self.
    pub fn normalize(self) -> Polyhedron<VtFcNm> {
        let normals: Vec<Vector3<f32>> = self.data.faces
            .iter()
            .map(|v| geop::triangle_normal(
                self.data.vertices[v[0]],
                self.data.vertices[v[1]],
                self.data.vertices[v[2]], 
            ))
            .collect();

        Polyhedron {
            data: VtFcNm {
                center: self.data.center,
                vertices: self.data.vertices,
                faces: self.data.faces,
                normals,
            }
        }
    }

    /// Calculate the centroid for each face and emit a `Polyhedron` with that information
    /// saved consuming self.
    pub fn centroidize(self) -> Polyhedron<VtFcCt> {
        let centroids: Vec<Point3<f32>> = self.data.faces
            .iter()
            .map(|v| v
                 .iter()
                 .map(|i| self.data.vertices[*i])
                 .collect::<Vec<Point3<f32>>>()
            )
            .map(|v| geop::convex_planar_polygon_centroid(&v))
            .collect();

        Polyhedron {
            data: VtFcCt {
                center: self.data.center,
                vertices: self.data.vertices,
                faces: self.data.faces,
                centroids: centroids,
            }
        }
    }

    /// As above but using a 'flawed' centroid calculation which makes the center point
    /// rise out of the planar polygon faces. This is to prevent the polyhedron from
    /// shrinking.
    pub fn rising_centroidize(self) -> Polyhedron<VtFcCt> {
        let f_centroids: Vec<Point3<f32>> = self.data.faces
            .iter()
            .map(|v| v
                 .iter()
                 .map(|i| self.data.vertices[*i])
                 .collect::<Vec<Point3<f32>>>()
            )
            .map(|v| geop::polyhedron_face_center(&v))
            .collect();

        Polyhedron {
            data: VtFcCt {
                center: self.data.center,
                vertices: self.data.vertices,
                faces: self.data.faces,
                centroids: f_centroids,
            }
        }
    }
}

impl VertexAndFaceOps for Polyhedron<VtFc> {
    fn vertices_and_faces(&self) -> (&[Point3<f32>], &[Vec<usize>]) {
        (&self.data.vertices, &self.data.faces)
    }
}

impl Polyhedron<VtFcNm> {
    pub fn faces(&self) -> impl Iterator<Item = planar::Polygon<f32>> + '_ {
        self.data.faces
            .iter()
            .map(move |vertex_indexes| {
                vertex_indexes
                    .iter()
                    .map(move |i| self.data.vertices[*i].clone())
                    .collect::<Vec<Point3<f32>>>()
            })
            .enumerate()
            .map(move |(i, v)| planar::Polygon::new(&v, self.data.normals[i].clone()))
    }
}

impl VertexAndFaceOps for Polyhedron<VtFcNm> {
    fn vertices_and_faces(&self) -> (&[Point3<f32>], &[Vec<usize>]) {
        (&self.data.vertices, &self.data.faces)
    }
}

impl Polyhedron<VtFcCt> {
    /// Strip out the centroid information.
    pub fn downgrade(self) -> Polyhedron<VtFc> {
        Polyhedron {
            data: VtFc {
                center: self.data.center,
                vertices: self.data.vertices,
                faces: self.data.faces,
            }
        }
    }
}

impl VertexAndFaceOps for Polyhedron<VtFcCt> {
    fn vertices_and_faces(&self) -> (&[Point3<f32>], &[Vec<usize>]) {
        (&self.data.vertices, &self.data.faces)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OpError {
    NoOperations,
    AlreadyHasSeed,
    NoSeedSet,
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation rejected: {}", match self {
            OpError::NoOperations => "No Conway operations set.",
            OpError::AlreadyHasSeed => "Seed already present.",
            OpError::NoSeedSet => "No seed has been set to run Conway operations on.",
        })
    }
}

impl error::Error for OpError {
    fn description(&self) -> &str {
        "Error adding Conway operation."
    }
}
