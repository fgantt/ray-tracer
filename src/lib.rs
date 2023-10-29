pub mod primitives {
    pub use canvas::Canvas;
    pub use color::Color;
    pub use matrix::Matrix;
    pub use point::Point;
    pub use tuple::Tuple;
    pub use vector::Vector;

    mod canvas;
    mod color;
    mod matrix;
    mod point;
    mod tuple;
    mod vector;
}
