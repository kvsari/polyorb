//! # Geometry Operations
//!
//! Common geomtery data types and operations that are used in polyhedron generation.
//use std::ops;

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

pub fn sum_three_points<S: BaseFloat>(
    p1: &Point3<S>, p2: &Point3<S>, p3: &Point3<S>
) -> Point3<S> {
    Point3::new(
        p1.x + p2.x + p3.x,
        p1.y + p2.y + p3.y,
        p1.z + p2.z + p3.z,
    )
}

/// We compute the centroid of the convex planar polygon by splitting it into 3 vertex
/// facets (triangles). Then the centroid and area is calculated for each triangle and
/// summed together and divided. This function assumes that there will be at least three
/// vertices and they all lie on the same plane if there are more than three.
///
/// Using [this formula](http://paulbourke.net/geometry/polygonmesh/). You need to scroll
/// down most of the page. It's 'Centroid of a 3D shell described by 3 vertex facets'.
pub fn convex_planar_polygon_centroid(vertices: &[Point3<f32>]) -> Point3<f32> {
    // Break into triangles by rotating on a starting axis. This works because it's
    // assumed to be a convex polygon.
    let p1 = vertices[0];

    let mut summed_area = 0.0;
    let mut summed_point_area: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
    
    for i in 1..(vertices.len() - 1) {
        let p2 = vertices[i];
        let p3 = vertices[i + 1];

        let average = sum_three_points(&p1, &p2, &p3) / 3.0;
        let area = (p2 - p1).cross(p3 - p1).magnitude();
        summed_point_area.x += area * average.x;
        summed_point_area.y += area * average.y;
        summed_point_area.z += area * average.z;
        
        summed_area += area;
    }

    // Centroid time
    summed_point_area / summed_area
}

/// A cheap and 'innacurate' form of calculating a centroid. By not taking the area into
/// question, the center moves 'out' of the planar polygon like the point of a pyramid
/// rising from the sand. This is technically wrong but when used for the Conway Dual
/// operation it ensures that the polyhedron doesn't shrink. Conway Operators after all
/// only specify operations on 'topology', not how the shape is geometrically calculated.
pub fn polyhedron_face_center(vertices: &[Point3<f32>]) -> Point3<f32> {
    let summed: Point3<f32> = vertices
        .iter()
        .fold(Point3::new(0.0, 0.0, 0.0), |mut s, p| -> Point3<f32> {
            s.x += p.x;
            s.y += p.y;
            s.z += p.z;

            s
        });

    summed / (vertices.len() as f32)
}

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
