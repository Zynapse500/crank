
mod vec2;

pub use self::vec2::Vec2;


// Predefined vector types
pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;


// Implement conversion between predefined vector types
impl From<Vec2f> for Vec2i {
    fn from(v: Vec2f) -> Self {
        Vec2i::new(v.x as i32, v.y as i32)
    }
}
