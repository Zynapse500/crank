
pub mod vector2;
pub use self::vector2::*;

pub mod vector2i;
pub use self::vector2i::*;


pub type IntType = i64;
pub type FloatType = f64;



/// Conversion between basic vector types
impl From<Vector2i> for Vector2 {
    fn from(v: Vector2i) -> Self {
        Vector2 {
            x: v.x as FloatType,
            y: v.y as FloatType
        }
    }
}

impl From<Vector2> for Vector2i {
    fn from(v: Vector2) -> Self {
        Vector2i {
            x: v.x as IntType,
            y: v.y as IntType
        }
    }
}
