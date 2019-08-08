//! # Line

use derive_getters::Getters;
use cgmath::{Point3, BaseFloat};

#[derive(Debug, Clone, Getters)]
pub struct Line<S: BaseFloat> {
    point1: Point3<S>,
    point2: Point3<S>,
}

impl<S: BaseFloat> Line<S> {
    pub fn new(point1: Point3<S>, point2: Point3<S>) -> Self {
        Line { point1, point2 }
    }
}

impl<S: BaseFloat> From<(Point3<S>, Point3<S>)> for Line<S> {
    fn from(t: (Point3<S>, Point3<S>)) -> Self {
        Line::new(t.0, t.1)
    }
}
