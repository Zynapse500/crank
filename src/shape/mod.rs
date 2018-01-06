


mod rectangle;
pub use self::rectangle::Rectangle;

mod line;
pub use self::line::Line;

mod triangle;
pub use self::triangle::Triangle;

mod render_shape;
pub use self::render_shape::RenderShape;



/// For shapes that occupy a finite amount of space
pub trait Bounded {
    /// Return the object's bounding box
    fn bounding_box(&self) -> Rectangle;
}
