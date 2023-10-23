use ray_tracer::primitives::{Point, Vector, Tuple};

pub fn run() {

    let mut p = Projectile { 
        position: Point::new(0.0, 1.0, 0.0), 
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25 
    };

    let e = Environment { 
        gravity: Vector::new(0.0, -0.1, 0.0), 
        wind: Vector::new(-0.01, 0.0, 0.0) 
    };
    
    println!("{:?}", p);

    while p.position.y() > 0.0 {
        p.tick(&e);
        println!("{:?}", p);
    }
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector
}

#[derive(Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector
}

impl Projectile {
    fn tick(&mut self, env: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }
}