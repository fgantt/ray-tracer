use std::{fs::File, io::Write};

use ray_tracer::primitives::{Canvas, Color, Point, Tuple, Vector};

pub fn run() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    // println!("{:?}", p);

    let mut c = Canvas::new(900, 550);

    while p.position.y() > 0.0 {
        p.tick(&e);
        // println!("{:?}", p);

        let cy = c.height() - 1 - (p.position.y().round() as usize);
        let cx = p.position.x().round() as usize;

        c[cx][cy] = Color::red();
    }

    let ppm = c.to_ppm();

    // Create a new file for writing
    // Create a file
    let mut data_file = File::create("images/projectile.ppm").expect("creation failed");

    // Write contents to the file
    data_file.write(ppm.as_bytes()).expect("write failed");

    println!("Write Operation Successful");
    // print!("{}", ppm);
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Projectile {
    fn tick(&mut self, env: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}
