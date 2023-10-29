use crate::primitives::color::Color;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

// ------------------------------------------------------
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas::new_with_bgcolor(width, height, Color::black())
    }

    pub fn new_with_bgcolor(width: usize, height: usize, color: Color) -> Self {
        if width == 0 || height == 0 {
            panic!("Invalid canvas dimensions.");
        }

        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self[x][y] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self[x][y]
    }

    pub fn to_ppm(self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let (r, g, b) = scale_color_components(self[x][y]);

                for comp in [r, g, b] {
                    let comp_str = format!("{} ", comp);
                    if line.len() + comp_str.len() > 70 {
                        line.pop();
                        line.push_str("\n");
                        ppm.push_str(&line);
                        line.clear();
                    }
                    line.push_str(&comp_str);
                }
            }
            if line.ends_with(" ") {
                line.pop();
            }
            ppm.push_str(&line);
            ppm.push_str("\n");
        }

        ppm
    }
}

fn scale_color_components(color: Color) -> (u8, u8, u8) {
    let r = (color.r().clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color.g().clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color.b().clamp(0.0, 1.0) * 255.0).round() as u8;
    (r, g, b)
}

// ------------------------------------------------------
impl std::ops::Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, row: usize) -> &[Color] {
        let start = row * self.height;
        // println!("Index col = {}: {:?}", row, &self.pixels[start..start + self.height]);
        &self.pixels[start..start + self.height]
    }
}

impl std::ops::IndexMut<usize> for Canvas {
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        let start = row * self.height;
        // println!("IndexMut row = {}: {:?}", row, &self.pixels[start..start + self.height]);
        &mut self.pixels[start..start + self.height]
    }
}

// ------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_construction() {
        let canvas = Canvas::new(10, 20);
        // println!("{:?}", c);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        let expected = Color::black();
        for p in canvas.pixels {
            assert_eq!(p, expected);
        }
    }

    #[test]
    fn canvas_ppm_header() {
        let c = Canvas::new(5, 3);
        let res = c.to_ppm();
        let v: Vec<&str> = res.split("\n").collect();
        assert_eq!(v[0], "P3");
        assert_eq!(v[1], "5 3");
        assert_eq!(v[2], "255");
    }

    #[test]
    fn canves_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c[0][0] = c1;
        c[2][1] = c2;
        c[4][2] = c3;
        let ppm = c.to_ppm();
        let v: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(v[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(v[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(v[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn canves_ppm_pixel_data_line_len70() {
        let c = Canvas::new_with_bgcolor(10, 2, Color::new(1.0, 0.8, 0.6));
        let ppm = c.to_ppm();
        println!("{}", ppm);
        for (_, line) in ppm.split("\n").enumerate() {
            assert!(line.len() <= 70)
        }
    }

    #[test]
    fn canvas_ppm_ends_with_newline() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.ends_with("\n"));
    }

    #[test]
    fn canvas_pixel_access_by_indexer() {
        let mut canvas = Canvas::new(10, 20);
        canvas[2][3] = Color::red();
        assert_eq!(canvas[2][3], Color::red());
    }

    #[test]
    fn canvas_pixel_access_by_fn() {
        let mut canvas = Canvas::new(10, 20);
        canvas.write_pixel(2, 3, Color::red());
        assert_eq!(canvas.pixel_at(2, 3), Color::red());
    }

    #[test]
    fn canvas_pixel_access_mix() {
        let mut canvas = Canvas::new(10, 20);

        let w = canvas.width;
        let h = canvas.height;

        let x = canvas.width - 1;
        let y = canvas.height - 1;
        canvas.write_pixel(x, y, Color::blue());
        assert_eq!(canvas[x][y], Color::blue());

        let x = 4;
        let y = 2;
        canvas[x][y] = Color::green();
        assert_eq!(canvas.pixel_at(x, y), Color::green());

        canvas[0][0] = Color::white();
        canvas[1][0] = Color::white();
        canvas[2][0] = Color::white();
        canvas[3][0] = Color::white();
        canvas[4][0] = Color::white();
        canvas[9][5] = Color::white();
        canvas[9][6] = Color::white();
        canvas[9][7] = Color::white();
        canvas[9][8] = Color::white();
        canvas[9][9] = Color::white();
        canvas[9][19] = Color::white();

        for y in 0..h {
            for x in 0..w {
                // print!("{:?} ", canvas[x][y]);
                // print!("{:?} ", canvas.pixels[y*w + x]);
                let t = canvas.pixel_at(x, y);
                // let t = canvas.pixels[x*h + y];
                if t == Color::white() {
                    print!("w");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
