
use super::Vertex;

pub struct RenderFrame {
    current_color: [f32; 4],

    pub(super) shapes: Vec<Vec<Vertex>>,
}

impl RenderFrame {
    /// Create a new frame
    pub fn new() -> Self {
        RenderFrame {
            current_color: [1.0, 1.0, 1.0, 1.0],

            shapes: Vec::new(),
        }
    }


    /// Set the current fill color
    pub fn set_color(&mut self, color: &[f32; 4]) {
        self.current_color = *color;
    }

    /// Draw a filled polygon
    pub fn draw_polygon(&mut self, points: &[[f32; 2]]) {
        let mut shape = Vec::new();

        for point in points {
            shape.push(Vertex {
                pos: [point[0], point[1], 0.0, 1.0],
                color: self.current_color
            });
        }

        self.shapes.push(shape);
    }
}

