//! # Plane stuff

use cgmath::{Point3, Vector3};

/// A plane in 3D space stored in `ax + by + cz + d = 0` form.
#[derive(Debug, Copy, Clone)]
pub struct Plane {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

/*
pub fn point_and_normal(point: Point3<f64>, normal: Vector3<f64>) -> Plane {
    
}
*/
