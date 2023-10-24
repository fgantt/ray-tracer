use crate::primitives::color::Color;


#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_construction() {
        let c = Canvas::new(2, 3, Color::new(1.0, 2.0, 3.0));
        println!("{:?}", c);
    }
}