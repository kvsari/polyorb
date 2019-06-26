//! Various 2d shapes used as building blocks for solids.
use std::ops::Neg;
use std::mem;

use derive_getters::Getters;
use cgmath::{Point2, Point3, Vector3};

#[derive(Debug, Copy, Clone, Getters)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    colour: [f32; 3], // Consider removing this in the upcoming refactor
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

fn triangle_normal(points: [&[f32; 3]; 3]) -> [f32; 3] {
    let p1 = Vector3::new(points[0][0], points[0][1], points[0][2]);
    let p2 = Vector3::new(points[1][0], points[1][1], points[1][2]);
    let p3 = Vector3::new(points[2][0], points[2][1], points[2][2]);

    let v = p2 - p1;
    let w = p3 - p1;

    let n = v.cross(w);

    [n.x, n.y, n.z] // normal gets normalized in the shader.
}

fn average_normals(normals: &[[f32; 3]]) -> [f32; 3] {
    let mut summed: [f32; 3] = [0f32, 0f32, 0f32];
    let mut count = 0;
    for [x, y, z] in normals {
        summed[0] += x;
        summed[1] += y;
        summed[2] += z;
        count += 1;
    }

    let divisor: f32 = count as f32;

    [summed[0] / divisor, summed[1] / divisor, summed[2] / divisor]
}

pub fn tetrahedron(len: f32, colour: [f32; 3]) -> ([Vertex; 12], [u16; 12]) {
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

    // Get our Y coordinates
    let center = ra_height / 3f32;                // The center point is 1/3 of the height
    let outer_radius = (ra_height * 2f32) / 3f32; // The outer radius is 2/3 of the height

    // Our equilateral triangle
    let left_point: [f32; 3] = [plot_x.neg(), center.neg(), center.neg()];
    let right_point: [f32; 3] = [plot_x, center.neg(), center.neg()];
    let top_point: [f32; 3] = [0f32, outer_radius, center.neg()];
    let depth_point: [f32; 3] = [0f32, 0f32, outer_radius];

    // From these four points we get our four triangles and normals.
    let n1 = triangle_normal([&right_point, &left_point, &top_point]);
    let n2 = triangle_normal([&left_point, &depth_point, &top_point]);
    let n3 = triangle_normal([&left_point, &right_point, &depth_point]);
    let n4 = triangle_normal([&depth_point, &right_point, &top_point]);

    let left_point_normal = average_normals(&[n1, n2, n3]);
    let right_point_normal = average_normals(&[n1, n3, n4]);
    let top_point_normal = average_normals(&[n1, n2, n4]);
    let depth_point_normal = average_normals(&[n2, n3, n4]);

    // debug colour
    let dcolour: [f32; 3] = [0f32, 0f32, 1f32];

    /*
    let vertexes = [
        // T1
        Vertex::new(right_point, right_point_normal, colour),
        Vertex::new(left_point, left_point_normal, colour),
        Vertex::new(top_point, top_point_normal, colour),
        
        // T2
        Vertex::new(left_point, left_point_normal, colour),
        Vertex::new(depth_point, depth_point_normal, colour),
        Vertex::new(top_point, top_point_normal, colour),
        
        // T3
        Vertex::new(left_point, left_point_normal, colour),
        Vertex::new(right_point, right_point_normal, colour),
        Vertex::new(depth_point, depth_point_normal, colour),

        // T4
        Vertex::new(depth_point, depth_point_normal, colour),
        Vertex::new(right_point, right_point_normal, colour),
        Vertex::new(top_point, top_point_normal, colour),
    ];
     */

    let vertexes = [
        // T1
        Vertex::new(right_point, n1, colour),
        Vertex::new(left_point, n1, colour),
        Vertex::new(top_point, n1, colour),
        
        // T2
        Vertex::new(left_point, n2, colour),
        Vertex::new(depth_point, n2, colour),
        Vertex::new(top_point, n2, colour),
        
        // T3
        Vertex::new(left_point, n3, colour),
        Vertex::new(right_point, n3, colour),
        Vertex::new(depth_point, n3, colour),

        // T4
        Vertex::new(depth_point, n4, colour),
        Vertex::new(right_point, n4, colour),
        Vertex::new(top_point, n4, colour),
    ];

    let indexes = [
        0, 1, 2,
        3, 4, 5,
        6, 7, 8,
        9, 10, 11,
    ];

    (vertexes, indexes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_makes_sense() {
        let p1 = [0f32, 0f32, 0f32];
        let p2 = [1f32, 0f32, 0f32];
        let p3 = [0f32, 1f32, 0f32];

        let n = triangle_normal([&p1, &p2, &p3]);

        println!("{:?}", &n);

        assert!(n == [0f32, 0f32, 1f32]);
    }

    /*
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
    */
}
