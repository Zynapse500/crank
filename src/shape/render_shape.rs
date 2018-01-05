
use  super::{Rectangle, Line, Triangle};

pub trait RenderShape {
    /// Draw a line with a certain width
    fn draw_line(&mut self, line: &Line, width: f32);


    /// Render a filled rectangle
    fn fill_rectangle(&mut self, rect: &Rectangle);

    /// Render the outline of a rectangle
    fn draw_rectangle(&mut self, rect: &Rectangle, line_width: f32);


    /// Render a filled triangle
    fn fill_triangle(&mut self, triangle: &Triangle);
}
