//! # Geometry Operations
//!
//! Common geomtery data types and operations that are used in polyhedron generation.

use cgmath::{Point3, Vector3, BaseFloat};
use cgmath::prelude::InnerSpace;

/// Produce the golden ratio of 1.6180339887...
///
/// 1 + √5
/// ──────
///   2
///
/// Why not just a constant? Why not constant function? Because rust hasn't yet made sqrt
/// a const function. I don't know why. It's a maths function. It should be easy.
pub fn golden_ratio() -> f32 {
    (1.0 + 5f32.sqrt()) / 2.0
}

/// Compute plane normal described by the three points forming a triangle on said plane.
pub fn triangle_normal<S: BaseFloat>(
    p1: Point3<S>, p2: Point3<S>, p3: Point3<S>
) -> Vector3<S> {
    let v1 = p1.to_homogeneous().truncate();
    let v2 = p2.to_homogeneous().truncate();
    let v3 = p3.to_homogeneous().truncate();

    let v = v2 - v1;
    let w = v3 - v1;

    v.cross(w).normalize()
}

/*
fn average_normals(normals: &[Vector3<S>]) -> Vector3<S> {
    let mut summed: Vector3<S> = Vector3::new(0.0, 0.0, 0.0);
    let mut count = 0;
    for normal in normals {
        summed.x += normal.x;
        summed.y += normal.y;
        summed.z += normal.z;
        count += 1;
    }

    let divisor: S = count as S;

    Vector3::new(summed.x / divisor, summed.y / divisor, summed.z / divisor)
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_makes_sense() {
        let p1 = Point3::new(0f32, 0f32, 0f32);
        let p2 = Point3::new(2f32, 0f32, 0f32);
        let p3 = Point3::new(0f32, 2f32, 0f32);

        let n = triangle_normal(p1, p2, p3);

        assert!(n == Vector3::new(0f32, 0f32, 1f32));
    }

    #[test]
    fn golden_ratio_golden() {
        let g = 1.6180339887;
        assert!(g == golden_ratio());
    }
}
