
use super::Vertex;
use super::view::{View, BoundedView};
use super::texture::Texture;

use std::f32::consts::PI;


pub struct RenderBatch {
    pub(super) vertices: Vec<Vertex>,
    pub(super) indices: Vec<u32>,

    pub(super) layer_count: u32,

    fill_color: [f32; 4],
    pub(super) texture: Option<Texture>,

    pub(super) view: Box<View>
}


impl RenderBatch {

    /// Create a new batch
    pub fn new() -> RenderBatch {
        RenderBatch {
            vertices: Vec::new(),
            indices: Vec::new(),

            layer_count: 0,

            fill_color: [1.0; 4],
            texture: None,

            view: Box::new(BoundedView::default())
        }
    }


    /// Remove data from previous rendering commands
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();

        self.fill_color = [1.0; 4];

        self.layer_count = 0;

        self.view = Box::new(BoundedView::default());
    }


    /// Set the current view
    pub fn set_view<V>(&mut self, view: V)
        where V: View + 'static
    {
        self.view = Box::new(view);
    }


    /// Set the current fill color
    pub fn set_fill_color(&mut self, color: [f32; 4]) {
        self.fill_color = color;
    }


    /// Set the current texture
    pub fn set_texture(&mut self, texture: Option<Texture>) {
        self.texture = texture;
    }


    /// Get the z-value of the next layer and increase the layer count
    fn advance_layer(&mut self) -> f32 {
        let z = self.layer_count as f32;
        self.layer_count += 1;
        z
    }


    /// Draw a rectangle
    pub fn draw_rectangle(&mut self, position: [f32; 2], size: [f32; 2]) {
        let x: f32 = position[0];
        let y: f32 = position[1];
        let w: f32 = size[0];
        let h: f32 = size[1];

        let z = self.advance_layer();

        let index_start: u32 = self.vertices.len() as u32;

        self.vertices.push(
            Vertex::new([x,     y,     z])
            .with_color(self.fill_color)
            .with_tex_coord([0.0, 0.0])
        );
        self.vertices.push(
            Vertex::new([x + w, y,     z])
            .with_color(self.fill_color)
            .with_tex_coord([1.0, 0.0])
        );
        self.vertices.push(
            Vertex::new([x + w, y + h, z])
            .with_color(self.fill_color)
            .with_tex_coord([1.0, 1.0])
        );
        self.vertices.push(
            Vertex::new([x,     y + h, z])
            .with_color(self.fill_color)
            .with_tex_coord([0.0, 1.0])
        );

        self.indices.push(index_start + 0);
        self.indices.push(index_start + 1);
        self.indices.push(index_start + 2);
        self.indices.push(index_start + 2);
        self.indices.push(index_start + 3);
        self.indices.push(index_start + 0);
    }


    /// Draw a circle with segments
    pub fn draw_circle_segments(&mut self, center: [f32; 2], radius: f32, segments: u32) {
        let x: f32 = center[0];
        let y: f32 = center[1];

        let z = self.advance_layer();

        let index_start: u32 = self.vertices.len() as u32;

        // Add center vertex
        self.vertices.push(Vertex::new([x, y, z]).with_color(self.fill_color));

        // Add perimeter
        let delta_angle = 2.0 * PI / segments as f32;
        let mut angle: f32 = 0.0;

        for s in 0..segments {
            // Add perimeter vertices
            let (dy, dx) = angle.sin_cos();
            self.vertices.push(Vertex::new([x + radius * dx, y + radius * dy, z]).with_color(self.fill_color));

            // Add center
            self.indices.push(index_start);

            // Add perimeters
            self.indices.push(index_start + s + 1);
            self.indices.push(index_start + (s + 1) % segments + 1);

            // Increase angle
            angle += delta_angle;
        }
    }

    /// Draw a circle with automatic number of segments
    pub fn draw_circle(&mut self, center: [f32; 2], radius: f32) {
        self.draw_circle_segments(center, radius, 16);
    }
}
