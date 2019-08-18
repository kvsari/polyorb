//! Icosahedron generation
use std::ops::Neg;

use cgmath::Point3;

use crate::geop::{triangle_normal, golden_ratio};
use super::Vertex;

/// Possibly broken if the len is anything other that 1.0.
///
/// TODO: Use the golden ratio!
pub (in crate::platonic_solid) fn icosahedron(
    len: f32, colour: [f32; 3]
) -> (Vec<Vertex<f32>>, Vec<u16>) {

    // Long side of the golden rectangle.
    let g_len = len * golden_ratio() as f32;

    // Now construct three orthogonal golden rectangles centered on (0, 0, 0).
    let g_mid = g_len / 2f32;

    // Short length of the golden rectangle. Since we halved `g_len`, must halve `len` too.
    let h_len = len / 2f32;

    // Rect 1 along X Y.
    let r_xy_tl = Point3::new(g_mid.neg(), h_len, 0f32);
    let r_xy_tr = Point3::new(g_mid, h_len, 0f32);
    let r_xy_br = Point3::new(g_mid, h_len.neg(), 0f32);
    let r_xy_bl = Point3::new(g_mid.neg(), h_len.neg(), 0f32);

    // Rect 2 along X Z
    let r_xz_tl = Point3::new(h_len, 0f32, g_mid.neg());
    let r_xz_tr = Point3::new(h_len, 0f32, g_mid);
    let r_xz_br = Point3::new(h_len.neg(), 0f32, g_mid);
    let r_xz_bl = Point3::new(h_len.neg(), 0f32, g_mid.neg());

    // Rect 3 along Y Z
    let r_yz_tl = Point3::new(0f32, g_mid.neg(), h_len);
    let r_yz_tr = Point3::new(0f32, g_mid, h_len);
    let r_yz_br = Point3::new(0f32, g_mid, h_len.neg());
    let r_yz_bl = Point3::new(0f32, g_mid.neg(), h_len.neg());

    let n01 = triangle_normal(r_yz_br, r_xy_tl, r_yz_tr);
    let n02 = triangle_normal(r_xy_tl, r_xz_bl, r_xy_bl);
    let n03 = triangle_normal(r_xy_tl, r_yz_br, r_xz_bl);
    let n04 = triangle_normal(r_xz_br, r_xy_tl, r_xy_bl);
    let n05 = triangle_normal(r_yz_tr, r_xy_tl, r_xz_br);
    let n06 = triangle_normal(r_xy_bl, r_xz_bl, r_yz_bl);
    let n07 = triangle_normal(r_xy_bl, r_yz_bl, r_yz_tl);
    let n08 = triangle_normal(r_xy_bl, r_yz_tl, r_xz_br);
    let n09 = triangle_normal(r_yz_tr, r_xz_br, r_xz_tr);
    let n10 = triangle_normal(r_xy_tr, r_yz_tr, r_xz_tr);
    let n11 = triangle_normal(r_yz_tr, r_xy_tr, r_yz_br);
    let n12 = triangle_normal(r_xz_tr, r_yz_tl, r_xy_br);
    let n13 = triangle_normal(r_xy_br, r_yz_tl, r_yz_bl);
    let n14 = triangle_normal(r_xy_br, r_yz_bl, r_xz_tl);
    let n15 = triangle_normal(r_xy_tr, r_xz_tr, r_xy_br);
    let n16 = triangle_normal(r_xz_tl, r_xy_tr, r_xy_br);
    let n17 = triangle_normal(r_xz_tl, r_yz_bl, r_xz_bl);
    let n18 = triangle_normal(r_xz_bl, r_yz_br, r_xz_tl);
    let n19 = triangle_normal(r_yz_br, r_xy_tr, r_xz_tl);
    let n20 = triangle_normal(r_xz_br, r_yz_tl, r_xz_tr);

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
