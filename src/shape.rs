//! Various 2d shapes used as building blocks for solids.
use std::ops::Neg;
use std::mem;

use derive_getters::Getters;
use cgmath::{Point2, Point3};

#[derive(Debug, Copy, Clone, Getters)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    colour: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 3], normal: [f32; 3], colour: [f32; 3]) -> Self {
        Vertex { position, normal, colour }
    }

    pub const fn sizeof() -> usize {
        mem::size_of::<Vertex>()
    }
}

/// Create an equilateral triangle centered on (0, 0). It's up to consumers to
/// translate/scale/rotate the triangle for their needs.
///
/// First value in the tuple are the 2D points. Second value is the index order.
pub fn equilateral_triangle(len: f32) -> ([Point2<f32>; 3], [u16; 3]) {
    // Use the hypotenuse to figure out the tip and compute the center point.
    // All calculations are using the X coordinate. The bottom of the triangle.

    // Setup out starting values
    let plot_x = len / 2f32;  // We want the triangle centered on the Y coord.
    let ra_x = plot_x;        // Right angle triangle X. Same length as the `plot_x`.
    let ra_hypotenuse = len;  // Right angle triangle hypotenuse.

    // Carry out reverse hypotenuse to get the triangle height.
    let ra_x2 = ra_x.exp2();
    let ra_hypotenuse2 = ra_hypotenuse.exp2();
    let ra_height2 = ra_hypotenuse2 - ra_x2;
    let ra_height = ra_height2.sqrt();

    // Get out Y coordinates
    let center = ra_height / 3f32;                // The center point is 1/3 of the height
    let outer_radius = (ra_height * 2f32) / 3f32; // The outer radius is 2/3 of the height

    // Our equilateral triangle
    let points = [
        Point2::new(plot_x.neg(), center.neg()), // Left
        Point2::new(plot_x, center.neg()),       // Right
        Point2::new(0f32, outer_radius),         // Top
    ];

    (points, [0, 1, 2])
}

pub fn square(len: f32) -> ([Point2<f32>; 4], [u16; 6]) {
    let l = len / 2f32;
    let points = [
        (l.neg(), l.neg()).into(),
        (l, l.neg()).into(),
        (l, l).into(),
        (l.neg(), l).into(),
    ];

    (points, [0, 1, 2, 2, 3, 0])
}

pub fn cube_01(colour: [f32; 3]) -> ([Vertex; 24], [u16; 36]) {
    let points = [
        // top (0, 0, 1)
        Vertex::new([-1f32, -1f32, 1f32], [0f32, 0f32, 1f32], colour),
        Vertex::new([1f32, -1f32, 1f32], [0f32, 0f32, 1f32], colour),
        Vertex::new([1f32, 1f32, 1f32], [0f32, 0f32, 1f32], colour),
        Vertex::new([-1f32, 1f32, 1f32], [0f32, 0f32, 1f32], colour),
        
        // bottom (0, 0, -1)
        Vertex::new([-1f32, 1f32, -1f32], [0f32, 0f32, -1f32], colour),
        Vertex::new([1f32, 1f32, -1f32], [0f32, 0f32, -1f32], colour),
        Vertex::new([1f32, -1f32, -1f32], [0f32, 0f32, -1f32], colour),
        Vertex::new([-1f32, -1f32, -1f32], [0f32, 0f32, -1f32], colour),
        
        // right (1, 0, 0)
        Vertex::new([1f32, -1f32, -1f32], [1f32, 0f32, 0f32], colour),
        Vertex::new([1f32, 1f32, -1f32], [1f32, 0f32, 0f32], colour),
        Vertex::new([1f32, 1f32, 1f32], [1f32, 0f32, 0f32], colour),
        Vertex::new([1f32, -1f32, 1f32], [1f32, 0f32, 0f32], colour),
        
        // left (-1, 0, 0)
        Vertex::new([-1f32, -1f32, 1f32], [-1f32, 0f32, 0f32], colour),
        Vertex::new([-1f32, 1f32, 1f32], [-1f32, 0f32, 0f32], colour),
        Vertex::new([-1f32, 1f32, -1f32], [-1f32, 0f32, 0f32], colour),
        Vertex::new([-1f32, -1f32, -1f32], [-1f32, 0f32, 0f32], colour),
        
        // front (0, 1, 0)
        Vertex::new([1f32, 1f32, -1f32], [0f32, 1f32, 0f32], colour),
        Vertex::new([-1f32, 1f32, -1f32], [0f32, 1f32, 0f32], colour),
        Vertex::new([-1f32, 1f32, 1f32], [0f32, 1f32, 0f32], colour),
        Vertex::new([1f32, 1f32, 1f32], [0f32, 1f32, 0f32], colour),
        
        // back (0, -1, 0)
        Vertex::new([1f32, -1f32, 1f32], [0f32, -1f32, 0f32], colour),
        Vertex::new([-1f32, -1f32, 1f32], [0f32, -1f32, 0f32], colour),
        Vertex::new([-1f32, -1f32, -1f32], [0f32, -1f32, 0f32], colour),
        Vertex::new([1f32, -1f32, -1f32], [0f32, -1f32, 0f32], colour),
    ];

    let indexes = [
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (points, indexes)
}

pub fn cube_02() -> ([Point3<f32>; 8], [u16; 36]) {
    let points = [
        // Front
        Point3::new(-1f32, -1f32, 1f32), // 1 (0)
        Point3::new(1f32, -1f32, 1f32),  // 2 (1)
        Point3::new(1f32, 1f32, 1f32),   // 3 (2)
        Point3::new(-1f32, 1f32, 1f32),  // 4 (3)
        
        // Back
        Point3::new(-1f32, -1f32, -1f32),// 5 (4) 
        Point3::new(1f32, -1f32, -1f32), // 6 (5)
        Point3::new(1f32, 1f32, -1f32),  // 7 (6)
        Point3::new(-1f32, 1f32, -1f32), // 8 (7)
    ];

    let indexes = [
        0, 1, 2, 2, 3, 0, // Front
        4, 5, 6, 6, 7, 4, // Back
        4, 3, 7, 7, 8, 4, // Top
        0, 1, 5, 5, 4, 0, // Bottom
        4, 0, 3, 3, 7, 4, // Left
        5, 1, 2, 2, 6, 5, // Right
    ];

    (points, indexes)
}

/*
pub fn tetrahedron(len: f32) -> ([Point3<f32>; 4], [u16; 12]) {
    // Use the hypotenuse to figure out the tip and compute the center point.
    // All calculations are using the X coordinate. The bottom of the triangle.

    // Setup out starting values
    let plot_x = len / 2f32;  // We want the triangle centered on the Y coord.
    let ra_x = plot_x;        // Right angle triangle X. Same length as the `plot_x`.
    let ra_hypotenuse = len;  // Right angle triangle hypotenuse.

    // Carry out reverse hypotenuse to get the triangle height.
    let ra_x2 = ra_x.exp2();
    let ra_hypotenuse2 = ra_hypotenuse.exp2();
    let ra_height2 = ra_hypotenuse2 - ra_x2;
    let ra_height = ra_height2.sqrt();

    // Get out Y coordinates
    let center = ra_height / 3f32;                // The center point is 1/3 of the height
    let outer_radius = (ra_height * 2f32) / 3f32; // The outer radius is 2/3 of the height

    // Our equilateral triangle
    let points = [
        Point3::new(plot_x.neg(), center.neg(), center.neg()), // Left
        Point3::new(plot_x, center.neg(), center.neg()),       // Right
        Point3::new(0f32, outer_radius, center.neg()),         // Top
        Point3::new(0f32, 0f32, outer_radius),                 // Depth (point)
    ];

    let indexes = [
        0, 1, 2, // Front,
        0, 1, 3, // Bottom,
        0, 3, 2, // Left,
        1, 2, 3, // Right,
    ];

    (points, indexes)
}
 */



/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gen_equilateral_triangle() {
        let points = equilateral_triangle(1_f32);
        let tp1 = Point2::new(0f32, 0f32);
        let tp2 = Point2::new(1f32, 0f32);

        let x = 0.5f32;
        let h = 1f32;
        let x2 = x.exp2();
        let h2 = h.exp2();
        let y2 = h2 - x2;
        let y = y2.sqrt();
        let tp3 = Point2::new(x, y);

        assert!(points[0] == tp1);
        assert!(points[1] == tp2);
        println!("{:?}", &points[2]);
        println!("{:?}", &tp3);
        assert!(points[2] == tp3);
    }
}
*/
