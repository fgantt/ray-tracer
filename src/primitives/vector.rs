use approx::abs_diff_eq;
use crate::primitives::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
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
        0.0
    }
}

// ------------------------------------------------------
impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        abs_diff_eq!(self.x, other.x, epsilon = f64::EPSILON) &&
        abs_diff_eq!(self.y, other.y, epsilon = f64::EPSILON) &&
        abs_diff_eq!(self.z, other.z, epsilon = f64::EPSILON)
    }
}

// ------------------------------------------------------
impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.x - rhs.x(),
            self.y - rhs.y(),
            self.z - rhs.z()
        )
    }
}

// ------------------------------------------------------
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            rhs.x * self,
            rhs.y * self,
            rhs.z * self
        )
    }
}

// ------------------------------------------------------
impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs
        )
    }
}

// ------------------------------------------------------
impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

// ------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_construction() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.x(), 4.3);
        assert_eq!(v.y(), -4.2);
        assert_eq!(v.z(), 3.1);
        assert_eq!(v.w(), 0.0);
    }

    #[test]
    fn sub_vectors() {
        let v1: Vector = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let res = v1 - v2;
        let expected = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn sub_vector_from_zero_vector() {
        let v1: Vector = Vector::new(0.0, 0.0, 0.0);
        let v2 = Vector::new(1.0, -2.0, 3.0);
        let res = v1 - v2;
        let expected = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn neg_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let res = -v;
        let expected = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn mul_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let res = v * 3.5;
        let expected = Vector::new(3.5, -7.0, 10.5);
        assert_eq!(res, expected);

        let res = 3.5 * v;
        assert_eq!(res, expected);

        let res = v * 0.5;
        let expected = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(res, expected);
    }

    #[test]
    fn div_vector_by_scalae() {
        let p = Vector::new(1.0, -2.0, 3.0);
        let res = p / 2.0;
        let expected = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(res, expected);
    }
}