use approx::abs_diff_eq;
use crate::primitives::{tuple::Tuple, vector::Vector};

// ------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64
}

// ------------------------------------------------------
impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        1.0
    }
}

// ------------------------------------------------------
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        abs_diff_eq!(self.x, other.x, epsilon = f64::EPSILON) &&
        abs_diff_eq!(self.y, other.y, epsilon = f64::EPSILON) &&
        abs_diff_eq!(self.z, other.z, epsilon = f64::EPSILON)
    }
}


// ------------------------------------------------------
impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z()
        }
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x() + rhs.x,
            y: self.y() + rhs.y,
            z: self.z() + rhs.z
        }
    }
}

// ------------------------------------------------------
impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(
            self.x - rhs.x(),
            self.y - rhs.y(),
            self.z - rhs.z()
        )
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;
    
    fn sub(self, rhs:Vector) -> Self::Output {
        Point::new(
            self.x - rhs.x(),
            self.y - rhs.y(),
            self.z - rhs.z()
        )
    }
}

// ------------------------------------------------------
impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}

impl std::ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(
            rhs.x * self,
            rhs.y * self,
            rhs.z * self
        )
    }
}

// ------------------------------------------------------
impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs
        )
    }
}

// ------------------------------------------------------
impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

// ------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_construction() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.x(), 4.3);
        assert_eq!(p.y(), -4.2);
        assert_eq!(p.z(), 3.1);
        assert_eq!(p.w(), 1.0);
    }

    #[test]
    fn add_point_add_vector() {
        let a1 = Point::new(3.0, -2.0, 5.0);
        let a2 = Vector::new(-2.0, 3.0, 1.0);
        let res = a1 + a2;
        let expexted = Point::new(1.0, 1.0, 6.0);
        assert_eq!(res, expexted);

        let res = a2 + a1;
        let expexted = Point::new(1.0, 1.0, 6.0);
        assert_eq!(res, expexted);
    }

    #[test]
    fn sub_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let res = p1 - p2;
        let expected = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn sub_point_sub_vector() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let res = p - v;
        let expected = Point::new(-2.0, -4.0, -6.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn neg_point() {
        let p = Point::new(1.0, -2.0, 3.0);
        let res = -p;
        let expected = Point::new(-1.0, 2.0, -3.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn mul_point_by_scalar() {
        let p = Point::new(1.0, -2.0, 3.0);
        let res = p * 3.5;
        let expected = Point::new(3.5, -7.0, 10.5);
        assert_eq!(res, expected);

        let res = 3.5 * p;
        assert_eq!(res, expected);

        let res = p * 0.5;
        let expected = Point::new(0.5, -1.0, 1.5);
        assert_eq!(res, expected);
    }

    #[test]
    fn div_point_by_scalae() {
        let p = Point::new(1.0, -2.0, 3.0);
        let res = p / 2.0;
        let expected = Point::new(0.5, -1.0, 1.5);
        assert_eq!(res, expected);
    }
}