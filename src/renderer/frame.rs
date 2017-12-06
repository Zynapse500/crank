
use super::Vertex;
use super::shape::Shape;

use super::camera::{Camera, OrthographicCamera};


macro_rules! current_color_vertex {
    ($self:ident, $x:expr, $y:expr) => {
        Vertex {
            pos: [$x, $y, 0.0, 1.0],
            color: ($self.current_color)
        }
    };
}




pub struct RenderFrame {
    current_color: [f32; 4],

    pub(super) cameras: Vec<CameraState>,

    pub(super) shapes: Vec<Shape>,
}

impl RenderFrame {
    /// Create a new frame
    pub fn new() -> Self {
        RenderFrame {
            cameras: vec![CameraState::new(Box::new(OrthographicCamera::default()), 0)],

            current_color: [1.0, 1.0, 1.0, 1.0],

            shapes: Vec::new(),
        }
    }


    /// Set the current fill color
    pub fn set_color(&mut self, color: &[f32; 4]) {
        self.current_color = *color;
    }


    /// Set the current camera
    pub fn set_camera<C>(&mut self, camera: C)
        where C: Camera + 'static
    {
        self.cameras.push(CameraState::new(Box::new(camera),
                                           self.shapes.len() as u32));
    }


    /// Add a shape to the render queue
    fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
        self.cameras.last_mut().unwrap().add_shape();
    }


    /// Draw a filled polygon
    pub fn draw_polygon(&mut self, points: &[[f32; 2]]) {
        let mut shape = Shape::new();

        for point in points {
            shape.vertices.push(Vertex {
                pos: [point[0], point[1], 0.0, 1.0],
                color: self.current_color
            });
        }

        for index in 0..points.len() as u32 - 2 {
            shape.indices.push(0);
            shape.indices.push(index + 1);
            shape.indices.push(index + 2);
        }

        self.add_shape(shape);
    }

    /// Draw a filled rectangle
    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let shape = Shape {
            vertices: vec![
                current_color_vertex!(self, x, y),
                current_color_vertex!(self, x, y + h),
                current_color_vertex!(self, x + w, y + h),
                current_color_vertex!(self, x + w, y),
            ],

            indices: vec![
                0, 1, 2,
                2, 3, 0
            ]
        };

        self.add_shape(shape);
    }
}


use std::ops::Range;

/// Each camera has to keep track of which objects were rendered using it
pub(super) struct CameraState {
    camera: Box<Camera>,

    // Range of drawn shapes
    shapes: Range<u32>
}

impl CameraState {
    pub fn new(camera: Box<Camera>, shape_count: u32) -> Self {
        CameraState {
            camera,
            shapes: shape_count..shape_count
        }
    }


    pub fn get_transform(&self) -> [[f32; 4]; 4] {
        self.camera.get_transform().into()
    }

    pub fn get_shapes(&self) -> Range<u32> {
        self.shapes.clone()
    }


    pub fn add_shape(&mut self) {
        self.shapes.end += 1;
    }
}
