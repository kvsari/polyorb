//! Various 2d shapes used as building blocks for solids.
use std::ops::Neg;

use cgmath::Point2;

/// Create an equilateral triangle centered on (0, 0). It's up to consumers to
/// translate/scale/rotate the triangle for their needs.
///
/// First value in the tuple are the 2D points. Second value is the index order.
pub fn equilateral_triangle(len: f32) -> ([Point2<f32>; 3], [u16; 3]) {
    // Use the hypotenuse to figure out the tip.
    let x = len / 2f32;
    let h = len;
    let x2 = x.exp2();
    let h2 = h.exp2();
    let y2 = h2 - x2;
    let y = y2.sqrt() / 2f32;
    
    let p1: Point2<f32> = (x.neg(), y).into();
    let p2: Point2<f32> = (x, y).into();    
    let p3: Point2<f32> = (0f32, y.neg()).into();

    ([p1, p2, p3], [0, 1, 2])
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
