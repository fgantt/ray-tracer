use approx::abs_diff_eq;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

// ------------------------------------------------------
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    pub fn add(lhs: Color, rhs: Color) -> Color {
        Color::new(
            lhs.r() + rhs.r(),
            lhs.g() + rhs.g(),
            lhs.b() + rhs.b()
        )
    }

    pub fn sub(lhs: Color, rhs: Color) -> Color {
        Color::new(
            lhs.r() - rhs.r(),
            lhs.g() - rhs.g(),
            lhs.b() - rhs.b()
        )
    }

    pub fn mul_by_scalar(color: Color, num: f64) -> Color {
        Color::new(
            color.r() * num,
            color.g() * num,
            color.b() * num
        )
    }

    pub fn mul(lhs: Color, rhs: Color) -> Color {
        Color::new(
            rhs.r() * lhs.r(),
            rhs.g() * lhs.g(),
            rhs.b() * lhs.b() 
        )
    }

}

// ------------------------------------------------------
impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        abs_diff_eq!(self.r, other.r, epsilon = f64::EPSILON) &&
        abs_diff_eq!(self.g, other.g, epsilon = f64::EPSILON) &&
        abs_diff_eq!(self.b, other.b, epsilon = f64::EPSILON)
    }
}

// ------------------------------------------------------
impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::add(self, rhs)
    }
}

// ------------------------------------------------------
impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::sub(self, rhs)
    }
}

// ------------------------------------------------------
impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, num: f64) -> Self::Output {
        Color::mul_by_scalar(self, num)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Self::Output {
        Color::mul_by_scalar(color, self)
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::mul(self, rhs)
    }
}

// ------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_construction() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = c1 + c2;
        let expected = Color::new(1.6, 0.7, 1.0);
        assert_eq!(res, expected);
    }

    #[test]
    fn sub_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = c1 - c2;
        let expected = Color::new(0.2, 0.5, 0.5);
        assert_eq!(res, expected);
    }

    #[test]
    fn mul_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let res = c1 * c2;
        let expected = Color::new(0.9, 0.2, 0.04);
        assert_eq!(res, expected);
    }
    
    #[test]
    fn mul_color_by_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let res = c1 * 2.0;
        let expected = Color::new(0.4, 0.6, 0.8);
        assert_eq!(res, expected);
    }
    
}