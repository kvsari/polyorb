//! Cube generation
use std::ops::Neg;

use cgmath::Point3;

use crate::scene;
use crate::geop::triangle_normal;
use super::{Vertex, Cached};

pub (in crate::platonic_solid) fn cube(
    len: f32, colour: [f32; 3]
) -> (Vec<Vertex<f32>>, Vec<u16>) {
    // Holdover from debugging the dodecahedron.
    let cl = len;
    
    // Get the cube first. p/n means positive of negative `cl` on the x,y and z.    
    let c_ppp = Point3::new(cl, cl, cl);
    let c_npp = Point3::new(cl.neg(), cl, cl);
    let c_nnp = Point3::new(cl.neg(), cl.neg(), cl);
    let c_pnp = Point3::new(cl, cl.neg(), cl);
    let c_ppn = Point3::new(cl, cl, cl.neg());
    let c_npn = Point3::new(cl.neg(), cl, cl.neg());
    let c_nnn = Point3::new(cl.neg(), cl.neg(), cl.neg());
    let c_pnn = Point3::new(cl, cl.neg(), cl.neg());

    let n1 = triangle_normal(c_ppp, c_npp, c_nnp);
    let n2 = triangle_normal(c_npn, c_ppn, c_nnn);
    let n3 = triangle_normal(c_ppp, c_pnp, c_ppn);
    let n4 = triangle_normal(c_nnp, c_npp, c_npn);
    let n5 = triangle_normal(c_npp, c_ppp, c_ppn);
    let n6 = triangle_normal(c_pnp, c_nnp, c_pnn);

    // Vertexes. Each face will be indexed to produce two triangles.
    let vertexes = vec![
        // Top
        Vertex::new(c_ppp, n1, colour),
        Vertex::new(c_npp, n1, colour),
        Vertex::new(c_nnp, n1, colour),
        Vertex::new(c_pnp, n1, colour),

        // Bottom
        Vertex::new(c_npn, n2, colour),
        Vertex::new(c_ppn, n2, colour),
        Vertex::new(c_pnn, n2, colour),
        Vertex::new(c_nnn, n2, colour),

        // Right
        Vertex::new(c_ppp, n3, colour),
        Vertex::new(c_pnp, n3, colour),
        Vertex::new(c_pnn, n3, colour),
        Vertex::new(c_ppn, n3, colour),

        // Left
        Vertex::new(c_nnp, n4, colour),
        Vertex::new(c_npp, n4, colour),
        Vertex::new(c_npn, n4, colour),
        Vertex::new(c_nnn, n4, colour),

        // Front
        Vertex::new(c_npp, n5, colour),
        Vertex::new(c_ppp, n5, colour),
        Vertex::new(c_ppn, n5, colour),
        Vertex::new(c_npn, n5, colour),

        // Back
        Vertex::new(c_pnp, n6, colour),
        Vertex::new(c_nnp, n6, colour),
        Vertex::new(c_nnn, n6, colour),
        Vertex::new(c_pnn, n6, colour),
    ];

    // Two triangles per face.
    let indexes = vec![
        0, 1, 2, 2, 3, 0,       // Top
        4, 5, 6, 6, 7, 4,       // Bottom
        8, 9, 10, 10, 11, 8,    // Right
        12, 13, 14, 14, 15, 12, // Left
        16, 17, 18, 18, 19, 16, // Front
        20, 21, 22, 22, 23, 20, // Back
    ];

    (vertexes, indexes)
}
