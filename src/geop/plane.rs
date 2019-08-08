//! # Plane stuff

use std::ops::Neg;

use derive_getters::Getters;
use cgmath::{Point3, Vector3, BaseFloat};
use cgmath::prelude::*;

//use super::Line;

/*
/// A plane in 3D space stored in `ax + by + cz + d = 0` form.
#[derive(Debug, Copy, Clone)]
pub enum Plane {
    XYZ { a: f64, b: f64, c: f64, d: f64 },
    XY { c: f64 },
    YZ { a: f64 },
    ZX { b: f64 },
}

impl Plane {
    pub fn line_intersection(p1: &Point3<f64>, p2: &Point3<f64>) -> Option<Point3<f64>> {
        None
    }
}

/// Get the plane that is passing through `point` and has `normal` direction. The `normal`
/// doesn't need to be normalized.
pub fn point_and_normal(point: Point3<f64>, normal: Vector3<f64>) -> Plane {
    // Check if the normal is parallel to two coordinate planes.
    if (normal.x == 0.0 && normal.y == 0.0) {
        Plane::XY { c: point.z }
    } else if (normal.y == 0.0 && normal.z == 0.0) {
        Plane::YZ { a: point.x }        
    } else if (normal.z == 0.0 && normal.x == 0.0) {
        Plane::ZX { b: point.y }
    } else { 
        let xd: f64 = (normal.x * point.x).neg();
        let yd: f64 = normal.y * point.y;
        let zd: f64 = normal.z * point.z;

        let d = xd - yd - zd;

        Plane::XYZ { a: normal.x, b: normal.y, c: normal.z, d, }
    }
}
 */

/// A plane in 3D space described by an intersecting point and normal.
#[derive(Debug, Clone, Getters)]
pub struct Plane<S: BaseFloat> {
    normal: Vector3<S>,
    point: Point3<S>,
}

impl<S: BaseFloat> Plane<S> {
    pub fn new(normal: Vector3<S>, point: Point3<S>) -> Self {
        let normal = normal.normalize();

        Plane { normal, point }
    }
    
    //pub fn line_intersection(p1: Point3<S>, p2: Point3<S>) -> Option<Point3<S>> {
        
    //}
}
