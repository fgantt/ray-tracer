use crate::primitives::color::Color;


#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

// ------------------------------------------------------
impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn width(self) -> usize {
        self.width
    }

    pub fn height(self) -> usize {
        self.height
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self[x][y] = color;
    }

    pub fn pixel_at(self, x: usize, y: usize) -> Color {
        self[x][y]
    }
}

// ------------------------------------------------------
impl std::ops::Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, col: usize) -> &[Color] {
        let start = col * self.height;
        &self.pixels[start..start + self.height]
    }
}

impl std::ops::IndexMut<usize> for Canvas {
    fn index_mut(&mut self, col: usize) -> &mut [Color] {
        let start = col * self.height;
        &mut self.pixels[start..start + self.height]
    }
}

// ------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_construction() {
        let canvas = Canvas::new(10, 20, Color::black());
        // println!("{:?}", c);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        let expected = Color::black();
        for p in canvas.pixels {
            assert_eq!(p, expected);
        }
    }

    #[test]
    fn canvas_pixel_access_by_indexer() {
        let mut canvas = Canvas::new(10, 20, Color::black());
        canvas[2][3] = Color::red();
        assert_eq!(canvas[2][3], Color::red());
    }

    #[test]
    fn canvas_pixel_access_by_fn() {
        let mut canvas = Canvas::new(10, 20, Color::black());
        canvas.write_pixel(2, 3, Color::red());
        assert_eq!(canvas.pixel_at(2, 3), Color::red());
    }

    #[test]
    fn canvas_pixel_access_mix() {
        let mut canvas = Canvas::new(10, 20, Color::black());

        let x = canvas.width - 1;
        let y = canvas.height - 1;
        canvas.write_pixel(x, y, Color::blue());
        assert_eq!(canvas[x][y], Color::blue());

        let x = 4;
        let y = 2;
        canvas[x][y] = Color::green();
        assert_eq!(canvas.pixel_at(x, y), Color::green());
    }
}