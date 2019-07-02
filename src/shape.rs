//! Various 2d shapes used as building blocks for solids.
use std::ops::Neg;
use std::mem;

use derive_getters::Getters;
use cgmath::{Point2, Point3, Vector3};

/// Produce the golden ratio of 1.6180339887...
/// Why not just a constant? Why not constant function? Because rust hasn't yet made sqrt
/// a const function. I don't know why. It's a maths function. It should be easy.
pub fn golden_ratio_f32() -> f32 {
    (1.0 + 5f32.sqrt()) / 2.0
}

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

pub fn cube(len: f32, colour: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    // Holdover from debugging the dodecahedron.
    let cl = len;
    
    // Get the cube first. p/n means positive of negative `cl` on the x,y and z.    
    let c_ppp = [cl, cl, cl];
    let c_npp = [cl.neg(), cl, cl];
    let c_nnp = [cl.neg(), cl.neg(), cl];
    let c_pnp = [cl, cl.neg(), cl];
    let c_ppn = [cl, cl, cl.neg()];
    let c_npn = [cl.neg(), cl, cl.neg()];
    let c_nnn = [cl.neg(), cl.neg(), cl.neg()];
    let c_pnn = [cl, cl.neg(), cl.neg()];

    let n1 = triangle_normal([&c_ppp, &c_npp, &c_nnp]);
    let n2 = triangle_normal([&c_npn, &c_ppn, &c_nnn]);
    let n3 = triangle_normal([&c_ppp, &c_pnp, &c_ppn]);
    let n4 = triangle_normal([&c_nnp, &c_npp, &c_npn]);
    let n5 = triangle_normal([&c_npp, &c_ppp, &c_ppn]);
    let n6 = triangle_normal([&c_pnp, &c_nnp, &c_pnn]);

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

pub fn octahedron(len: f32, colour: [f32; 3]) -> ([Vertex; 24], [u16; 24]) {
    // We want to build the anchor square in the center (0, 0, 0) over X, Y.
    let h_len: f32 = len / 2f32;

    // We spell out the formula instead of using `half_len` to avoid confusion.
    let circumscribed_sphere_radius: f32 = (len / 2f32) * 2f32.sqrt();

    // Build our square.
    let p_top_left:  [f32; 3] = [h_len.neg(), h_len, 0f32];
    let p_top_right: [f32; 3] = [h_len, h_len, 0f32];
    let p_bot_left:  [f32; 3] = [h_len.neg(), h_len.neg(), 0f32];
    let p_bot_right: [f32; 3] = [h_len, h_len.neg(), 0f32];

    // Get the Z points using the sphere radius
    let p_far:  [f32; 3] = [0f32, 0f32, circumscribed_sphere_radius.neg()];
    let p_near: [f32; 3] = [0f32, 0f32, circumscribed_sphere_radius];

    // Calc our normals!
    let n1 = triangle_normal([&p_top_left, &p_top_right, &p_far]);
    let n2 = triangle_normal([&p_bot_left, &p_top_left, &p_far]);
    let n3 = triangle_normal([&p_bot_right, &p_bot_left, &p_far]);
    let n4 = triangle_normal([&p_top_right, &p_bot_right, &p_far]);
    
    let n5 = triangle_normal([&p_top_right, &p_top_left, &p_near]);
    let n6 = triangle_normal([&p_top_left, &p_bot_left, &p_near]);
    let n7 = triangle_normal([&p_bot_left, &p_bot_right, &p_near]);
    let n8 = triangle_normal([&p_bot_right, &p_top_right, &p_near]);

    // debug colour
    let dcolour: [f32; 3] = [0f32, 0f32, 1f32];

    let vertexes = [
        // T1
        Vertex::new(p_top_left, n1, colour),
        Vertex::new(p_top_right, n1, colour),
        Vertex::new(p_far, n1, colour),

        // T2
        Vertex::new(p_bot_left, n2, colour),
        Vertex::new(p_top_left, n2, colour),
        Vertex::new(p_far, n2, colour),

        // T3
        Vertex::new(p_bot_right, n3, colour),
        Vertex::new(p_bot_left, n3, colour),
        Vertex::new(p_far, n3, colour),

        // T4
        Vertex::new(p_top_right, n4, colour),
        Vertex::new(p_bot_right, n4, colour),
        Vertex::new(p_far, n4, colour),

        // T5
        Vertex::new(p_top_right, n5, colour),
        Vertex::new(p_top_left, n5, colour),
        Vertex::new(p_near, n5, colour),

        // T6
        Vertex::new(p_top_left, n6, colour),
        Vertex::new(p_bot_left, n6, colour),
        Vertex::new(p_near, n6, colour),

        // T7
        Vertex::new(p_bot_left, n7, colour),
        Vertex::new(p_bot_right, n7, colour),
        Vertex::new(p_near, n7, colour),

        // T8
        Vertex::new(p_bot_right, n8, colour),
        Vertex::new(p_top_right, n8, colour),
        Vertex::new(p_near, n8, colour),
    ];

    let indexes = [
        0,  1,  2,
        3,  4,  5,
        6,  7,  8,
        9,  10, 11,
        12, 13, 14,
        15, 16, 17,
        18, 19, 20,
        21, 22, 23,
    ];

    (vertexes, indexes)
}

/// Use the three orthogonal golden rectangle technique to generate the icosahedron.
pub fn icosahedron(len: f32, colour: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    // debug colour
    let dcolour: [f32; 3] = [1f32, 0f32, 0f32];
    
    // Build the long side of the golden rectangle.
    let h_len = len / 2f32;
    let g_len = h_len + (h_len * 5f32.sqrt());

    // Now construct three orthogonal golden rectangles centered on (0, 0, 0).
    let g_mid = g_len / 2f32;

    // Rect 1 along X Y.
    let r_xy_tl = [g_mid.neg(), h_len, 0f32];
    let r_xy_tr = [g_mid, h_len, 0f32];
    let r_xy_br = [g_mid, h_len.neg(), 0f32];
    let r_xy_bl = [g_mid.neg(), h_len.neg(), 0f32];

    // Rect 2 along X Z
    let r_xz_tl = [h_len, 0f32, g_mid.neg()];
    let r_xz_tr = [h_len, 0f32, g_mid];
    let r_xz_br = [h_len.neg(), 0f32, g_mid];
    let r_xz_bl = [h_len.neg(), 0f32, g_mid.neg()];

    // Rect 3 along Y Z
    let r_yz_tl = [0f32, g_mid.neg(), h_len];
    let r_yz_tr = [0f32, g_mid, h_len];
    let r_yz_br = [0f32, g_mid, h_len.neg()];
    let r_yz_bl = [0f32, g_mid.neg(), h_len.neg()];

    let n01 = triangle_normal([&r_yz_br, &r_xy_tl, &r_yz_tr]);
    let n02 = triangle_normal([&r_xy_tl, &r_xz_bl, &r_xy_bl]);
    let n03 = triangle_normal([&r_xy_tl, &r_yz_br, &r_xz_bl]);
    let n04 = triangle_normal([&r_xz_br, &r_xy_tl, &r_xy_bl]);
    let n05 = triangle_normal([&r_yz_tr, &r_xy_tl, &r_xz_br]);
    let n06 = triangle_normal([&r_xy_bl, &r_xz_bl, &r_yz_bl]);
    let n07 = triangle_normal([&r_xy_bl, &r_yz_bl, &r_yz_tl]);
    let n08 = triangle_normal([&r_xy_bl, &r_yz_tl, &r_xz_br]);
    let n09 = triangle_normal([&r_yz_tr, &r_xz_br, &r_xz_tr]);
    let n10 = triangle_normal([&r_xy_tr, &r_yz_tr, &r_xz_tr]);
    let n11 = triangle_normal([&r_yz_tr, &r_xy_tr, &r_yz_br]);
    let n12 = triangle_normal([&r_xz_tr, &r_yz_tl, &r_xy_br]);
    let n13 = triangle_normal([&r_xy_br, &r_yz_tl, &r_yz_bl]);
    let n14 = triangle_normal([&r_xy_br, &r_yz_bl, &r_xz_tl]);
    let n15 = triangle_normal([&r_xy_tr, &r_xz_tr, &r_xy_br]);
    let n16 = triangle_normal([&r_xz_tl, &r_xy_tr, &r_xy_br]);
    let n17 = triangle_normal([&r_xz_tl, &r_yz_bl, &r_xz_bl]);
    let n18 = triangle_normal([&r_xz_bl, &r_yz_br, &r_xz_tl]);
    let n19 = triangle_normal([&r_yz_br, &r_xy_tr, &r_xz_tl]);
    let n20 = triangle_normal([&r_xz_br, &r_yz_tl, &r_xz_tr]);

    let vertexes = vec![
        // T1
        Vertex::new(r_yz_br, n01, colour),
        Vertex::new(r_xy_tl, n01, colour),
        Vertex::new(r_yz_tr, n01, colour),

        // T2
        Vertex::new(r_xy_tl, n02, colour),
        Vertex::new(r_xz_bl, n02, colour),
        Vertex::new(r_xy_bl, n02, colour),

        // T3
        Vertex::new(r_xy_tl, n03, colour),
        Vertex::new(r_yz_br, n03, colour),
        Vertex::new(r_xz_bl, n03, colour),

        // T4
        Vertex::new(r_xz_br, n04, colour),
        Vertex::new(r_xy_tl, n04, colour),
        Vertex::new(r_xy_bl, n04, colour),

        // T5
        Vertex::new(r_yz_tr, n05, colour),
        Vertex::new(r_xy_tl, n05, colour),
        Vertex::new(r_xz_br, n05, colour),

        // T6
        Vertex::new(r_xy_bl, n06, colour),
        Vertex::new(r_xz_bl, n06, colour),
        Vertex::new(r_yz_bl, n06, colour),

        // T7
        Vertex::new(r_xy_bl, n07, colour),
        Vertex::new(r_yz_bl, n07, colour),
        Vertex::new(r_yz_tl, n07, colour),

        // T8
        Vertex::new(r_xy_bl, n08, colour),
        Vertex::new(r_yz_tl, n08, colour),
        Vertex::new(r_xz_br, n08, colour),

        // T9
        Vertex::new(r_yz_tr, n09, colour),
        Vertex::new(r_xz_br, n09, colour),
        Vertex::new(r_xz_tr, n09, colour),

        // T10
        Vertex::new(r_xy_tr, n10, colour),
        Vertex::new(r_yz_tr, n10, colour),
        Vertex::new(r_xz_tr, n10, colour),

        // T11
        Vertex::new(r_yz_tr, n11, colour),
        Vertex::new(r_xy_tr, n11, colour),
        Vertex::new(r_yz_br, n11, colour),

        // T12
        Vertex::new(r_xz_tr, n12, colour),
        Vertex::new(r_yz_tl, n12, colour),
        Vertex::new(r_xy_br, n12, colour),

        // T13
        Vertex::new(r_xy_br, n13, colour),
        Vertex::new(r_yz_tl, n13, colour),
        Vertex::new(r_yz_bl, n13, colour),

        // T14
        Vertex::new(r_xy_br, n14, colour),
        Vertex::new(r_yz_bl, n14, colour),
        Vertex::new(r_xz_tl, n14, colour),

        // T15
        Vertex::new(r_xy_tr, n15, colour),
        Vertex::new(r_xz_tr, n15, colour),
        Vertex::new(r_xy_br, n15, colour),

        // T16
        Vertex::new(r_xz_tl, n16, colour),
        Vertex::new(r_xy_tr, n16, colour),
        Vertex::new(r_xy_br, n16, colour),

        // T17
        Vertex::new(r_xz_tl, n17, colour),
        Vertex::new(r_yz_bl, n17, colour),
        Vertex::new(r_xz_bl, n17, colour),

        // T18
        Vertex::new(r_xz_bl, n18, colour),
        Vertex::new(r_yz_br, n18, colour),
        Vertex::new(r_xz_tl, n18, colour),

        // T19
        Vertex::new(r_yz_br, n19, colour),
        Vertex::new(r_xy_tr, n19, colour),
        Vertex::new(r_xz_tl, n19, colour),

        // T20
        Vertex::new(r_xz_br, n20, colour),
        Vertex::new(r_yz_tl, n20, colour),
        Vertex::new(r_xz_tr, n20, colour),
    ];

    let indexes = vec![
        0, 1, 2,
        3, 4, 5,
        6, 7, 8,
        9, 10, 11,
        12, 13, 14,
        15, 16, 17,
        18, 19, 20,
        21, 22, 23,
        24, 25, 26,
        27, 28, 29,
        30, 31, 32,
        33, 34, 35,
        36, 37, 38,
        39, 40, 41,
        42, 43, 44,
        45, 46, 47,
        48, 49, 50,
        51, 52, 53,
        54, 55, 56,
        57, 58, 59,
    ];

    (vertexes, indexes)
}

/// Build a dodecahedron by first calculating the 20 verteces via the cube and three
/// orthogonal rectangles.
pub fn dodecahedron(len: f32, colour: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {    
    // Maybie half the length to get started? We are centering on (0, 0, 0).
    let len = len / 2f32;

    // Get the golden ratio
    let g = golden_ratio_f32();

    // Compute the verteces.

    // The cube is the line crossing the two sides of a pentagon. Thus it is the `len * g`.
    let cl = len * g;
    // Get the cube first. p/n means positive of negative `cl` on the x,y and z.    
    let c_ppp = [cl, cl, cl];
    let c_npp = [cl.neg(), cl, cl];
    let c_nnp = [cl.neg(), cl.neg(), cl];
    let c_pnp = [cl, cl.neg(), cl];
    let c_ppn = [cl, cl, cl.neg()];
    let c_npn = [cl.neg(), cl, cl.neg()];
    let c_nnn = [cl.neg(), cl.neg(), cl.neg()];
    let c_pnn = [cl, cl.neg(), cl.neg()];

    // Now we get our rectangles using the golden ratio we prepared earlier. p/n again means
    // positive or negative values but this time the coordinates are denoted in the name.

    // The long edges of the rectangle are the len * g * g or cl * g.
    let s = len;
    let l = cl * g;

    // Rectangle that lies on the x y plane.
    let r_xy_pp = [l, s, 0f32];
    let r_xy_pn = [l, s.neg(), 0f32];
    let r_xy_nn = [l.neg(), s.neg(), 0f32];
    let r_xy_np = [l.neg(), s, 0f32];

    // Rectangle that lies on the x z plane.
    let r_xz_pp = [s, 0f32, l];
    let r_xz_pn = [s, 0f32, l.neg()];
    let r_xz_nn = [s.neg(), 0f32, l.neg()];
    let r_xz_np = [s.neg(), 0f32, l];

    // Rectangle that lies on the y z plane.
    let r_yz_pp = [0f32, l, s];
    let r_yz_pn = [0f32, l, s.neg()];
    let r_yz_nn = [0f32, l.neg(), s.neg()];
    let r_yz_np = [0f32, l.neg(), s];

    /*
    let n1 = triangle_normal([&r_xy_pp, &r_xy_pn, &r_xy_nn]);
    let n2 = triangle_normal([&r_xz_pp, &r_xz_pn, &r_xz_nn]);
    let n3 = triangle_normal([&r_yz_pp, &r_yz_pn, &r_yz_nn]);

    // Render each rectangle to ensure it's correct.
    let vertexes = vec![
        /*
        Vertex::new(r_xy_pp, n1, colour),
        Vertex::new(r_xy_pn, n1, colour),
        Vertex::new(r_xy_nn, n1, colour),
        Vertex::new(r_xy_np, n1, colour),
        */

        /*
        Vertex::new(r_xz_pp, n2, colour),
        Vertex::new(r_xz_pn, n2, colour),
        Vertex::new(r_xz_nn, n2, colour),
        Vertex::new(r_xz_np, n2, colour),
        */

        Vertex::new(r_yz_pp, n3, colour),
        Vertex::new(r_yz_pn, n3, colour),
        Vertex::new(r_yz_nn, n3, colour),
        Vertex::new(r_yz_np, n3, colour),

    ];

    let indexes = vec![
        0, 1, 2, 2, 3, 0, /* and reverse! */ 2, 1, 0, 0, 3, 2,
    ];
    */

    // Get the normals for flat shading our pentagons. We only need a triangle.
    let n01 = triangle_normal([&c_nnp, &r_yz_np, &c_pnp]);

    // debug colour;
    let dcolour: [f32; 3] = [1f32, 0f32, 0f32];

    // Define the vertexes for each pentagon. We trace three triangles using the indexes.
    let vertexes = vec![
        // P1
        Vertex::new(r_xz_np, n01, colour),
        Vertex::new(c_nnp, n01, colour),
        Vertex::new(r_yz_np, n01, colour),
        Vertex::new(c_pnp, n01, colour),
        Vertex::new(r_xz_pp, n01, colour),
    ];

    let indexes = vec![
        // P1
        0, 1, 2, /*T1*/ 0, 2, 4, /*T2*/ 4, 2, 3, /*T3*/
    ];

    /*
    let n1 = triangle_normal([&c_ppp, &c_npp, &c_nnp]);
    let n2 = triangle_normal([&c_npn, &c_ppn, &c_nnn]);
    let n3 = triangle_normal([&c_ppp, &c_pnp, &c_ppn]);
    let n4 = triangle_normal([&c_nnp, &c_npp, &c_npn]);
    let n5 = triangle_normal([&c_npp, &c_ppp, &c_ppn]);
    let n6 = triangle_normal([&c_pnp, &c_nnp, &c_pnn]);

    // Check that we've got the cube correctly
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
    */

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

    #[test]
    fn golden_ratio_golden() {
        let g = 1.6180339887;
        assert!(g == golden_ratio_f32());
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
