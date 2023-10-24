pub mod primitives {
    pub use canvas::Canvas;
    pub use color::Color;
    pub use tuple::Tuple;
    pub use point::Point;
    pub use vector::Vector;

    mod canvas;
    mod color;
    mod tuple;
    mod point;
    mod vector;
}