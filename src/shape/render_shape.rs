
use super::{Rectangle, Line, Triangle};
use ::{FloatType};

pub trait RenderShape {
    /// Draw a line with a certain width
    fn draw_line(&mut self, line: &Line, width: FloatType);


    /// Render a filled rectangle
    fn fill_rectangle(&mut self, rect: &Rectangle);

    /// Render the outline of a rectangle
    fn draw_rectangle(&mut self, rect: &Rectangle, line_width: FloatType);


    /// Render a filled triangle
    fn fill_triangle(&mut self, triangle: &Triangle);
}
