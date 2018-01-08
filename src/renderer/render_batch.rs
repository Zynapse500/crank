use super::Vertex;
use super::view::{View};
use super::texture::Texture;

use super::mesh::Mesh;

use ::shape::{RenderShape, Rectangle, Line, Triangle};

use std::f32::consts::PI;
use std::collections::HashMap;

use ::{FloatType, Vector2};

pub struct RenderBatch {
    pub(super) mesh_indices: HashMap<Texture, usize>,
    pub(super) meshes: Vec<Mesh>,

    pub(super) layer_count: u32,

    current_color: [f32; 4],
    current_mesh: usize,

    default_texture: Texture,

    pub(super) view: Box<View>,
}


impl RenderBatch {
    /// Create a new batch
    pub fn new() -> RenderBatch {
        let default_texture = Texture::default();

        let mut mesh_indices = HashMap::new();
        mesh_indices.insert(default_texture, 0);

        RenderBatch {
            mesh_indices,
            meshes: vec![Mesh::new()],

            layer_count: 0,

            current_color: [1.0; 4],
            current_mesh: 0,

            default_texture,

            view: Box::new(Rectangle::default()),
        }
    }


    /// Return the current amount of layers
    pub fn get_layer_count(&self) -> u32 {
        self.layer_count
    }


    /// Remove data from previous rendering commands
    pub fn clear(&mut self) {
        for (_, &index) in self.mesh_indices.iter() {
            self.meshes[index].vertices.clear();
            self.meshes[index].indices.clear();
        }

        self.mesh_indices.clear();
        self.set_texture(None);

        self.current_color = [1.0; 4];
        self.layer_count = 0;
        self.view = Box::new(Rectangle::default());
    }


    /// Set the current view
    pub fn set_view<V>(&mut self, view: V)
        where V: View + 'static
    {
        self.view = Box::new(view);
    }


    /// Return the bounds of the view
    pub fn get_view_bounds(&self) -> Rectangle {
        let mut bottom_left = self.view.ndc_to_world(Vector2::new(-1.0, -1.0));
        let mut top_right = self.view.ndc_to_world(Vector2::new(1.0, 1.0));

        if bottom_left.y > top_right.y {
            let tmp = bottom_left.y;
            bottom_left.y = top_right.y;
            top_right.y = tmp;
        }

        Rectangle { min: bottom_left, max: top_right }
    }


    /// Set the current fill color
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.current_color = color;
    }


    /// Set the current texture
    pub fn set_texture(&mut self, texture: Option<Texture>) {
        if let Some(texture) = texture {
            {
                let mesh_index = self.mesh_indices.get(&texture);
                if let Some(mesh) = mesh_index {
                    self.current_mesh = mesh.clone();
                    return;
                }
            }
            let mesh_index = self.mesh_indices.len();
            self.mesh_indices.insert(texture, mesh_index);
            while mesh_index >= self.meshes.len() {
                self.meshes.push(Mesh::new());
            }
            self.current_mesh = mesh_index;
        } else {
            let texture = self.default_texture;
            self.set_texture(Some(texture));
        }
    }


    /// Get the z-value of the next layer and increase the layer count
    fn advance_layer(&mut self) -> f32 {
        let z = self.layer_count as f32;
        self.layer_count += 1;
        z
    }


    /// Draw a circle with segments
    pub fn draw_circle_segments(&mut self, center: Vector2, radius: FloatType, segments: u32) {
        let x: f32 = center.x as f32;
        let y: f32 = center.y as f32;
        let radius = radius as f32;

        let z = self.advance_layer();

        // Get current mesh
        let mesh = &mut self.meshes[self.current_mesh];

        let index_start: u32 = mesh.vertices.len() as u32;

        // Add center vertex
        mesh.vertices.push(Vertex::new([x, y, z]).with_color(self.current_color));

        // Add perimeter
        let delta_angle = 2.0 * PI / segments as f32;
        let mut angle: f32 = 0.0;

        for s in 0..segments {
            // Add perimeter vertices
            let (dy, dx) = angle.sin_cos();
            mesh.vertices.push(
                Vertex::new([x + radius * dx, y + radius * dy, z])
                    .with_color(self.current_color)
            );

            // Add center
            mesh.indices.push(index_start);

            // Add perimeters
            mesh.indices.push(index_start + s + 1);
            mesh.indices.push(index_start + (s + 1) % segments + 1);

            // Increase angle
            angle += delta_angle;
        }
    }

    /// Draw a circle with automatic number of segments
    pub fn draw_circle(&mut self, center: Vector2, radius: FloatType) {
        self.draw_circle_segments(center, radius, 32);
    }

    /// Draw an extruded rectangle in a direction
    pub fn draw_extruded_rectangle(&mut self, rect: &Rectangle, direction: Vector2, width: FloatType) {
        let a = Vector2::new(rect.min.x, rect.min.y);
        let b = Vector2::new(rect.min.x, rect.max.y);
        let c = Vector2::new(rect.max.x, rect.min.y);
        let d = Vector2::new(rect.max.x, rect.max.y);

        let mut other_rect = rect.clone();
        other_rect.min += direction;
        other_rect.max += direction;

        self.draw_rectangle(rect, width);
        self.draw_rectangle(&other_rect, width);

        self.draw_line(&Line::new(a, a + direction), width);
        self.draw_line(&Line::new(b, b + direction), width);
        self.draw_line(&Line::new(c, c + direction), width);
        self.draw_line(&Line::new(d, d + direction), width);
    }
}


impl RenderShape for RenderBatch {
    fn draw_line(&mut self, line: &Line, width:  FloatType) {
        // Find the line perpendicular to the line
        let d = line.get_direction();
        let p = d.perpendicular();

        // "Radius" of the line
        let r = width / 2.0;

        // Calculate scaled perpendicular, used in Minkowski addition
        let dr = r * d;
        let pr = r * p;

        // Calculate adjusted endpoints
        let start = line.start - dr;
        let end = line.end + dr;

        // Calculate corners of line using Minkowski addition
        let a = start + pr;
        let b = start - pr;

        let c = end + pr;
        let d = end - pr;

        let z = self.advance_layer();

        // Get current mesh
        let mesh = &mut self.meshes[self.current_mesh];


        // Construct line vertices
        let index_start: u32 = mesh.vertices.len() as u32;

        mesh.vertices.push(
            Vertex::new([b.x as f32, b.y as f32, z])
                .with_color(self.current_color)
                .with_tex_coord([0.0, 1.0])
        );
        mesh.vertices.push(
            Vertex::new([d.x as f32, d.y as f32, z])
                .with_color(self.current_color)
                .with_tex_coord([1.0, 1.0])
        );
        mesh.vertices.push(
            Vertex::new([c.x as f32, c.y as f32, z])
                .with_color(self.current_color)
                .with_tex_coord([1.0, 0.0])
        );
        mesh.vertices.push(
            Vertex::new([a.x as f32, a.y as f32, z])
                .with_color(self.current_color)
                .with_tex_coord([0.0, 0.0])
        );


        // Index vertices
        mesh.indices.push(index_start + 0);
        mesh.indices.push(index_start + 1);
        mesh.indices.push(index_start + 2);
        mesh.indices.push(index_start + 2);
        mesh.indices.push(index_start + 3);
        mesh.indices.push(index_start + 0);
    }

    fn fill_rectangle(&mut self, rect: &Rectangle) {
        let x0 = rect.min.x as f32;
        let y0 = rect.min.y as f32;
        let x1 = rect.max.x as f32;
        let y1 = rect.max.y as f32;

        let z = self.advance_layer();

        // Get current mesh
        let mesh = &mut self.meshes[self.current_mesh];

        let index_start: u32 = mesh.vertices.len() as u32;

        mesh.vertices.push(
            Vertex::new([x0, y0, z])
                .with_color(self.current_color)
                .with_tex_coord([0.0, 1.0])
        );
        mesh.vertices.push(
            Vertex::new([x1, y0, z])
                .with_color(self.current_color)
                .with_tex_coord([1.0, 1.0])
        );
        mesh.vertices.push(
            Vertex::new([x1, y1, z])
                .with_color(self.current_color)
                .with_tex_coord([1.0, 0.0])
        );
        mesh.vertices.push(
            Vertex::new([x0, y1, z])
                .with_color(self.current_color)
                .with_tex_coord([0.0, 0.0])
        );

        mesh.indices.push(index_start + 0);
        mesh.indices.push(index_start + 1);
        mesh.indices.push(index_start + 2);
        mesh.indices.push(index_start + 2);
        mesh.indices.push(index_start + 3);
        mesh.indices.push(index_start + 0);
    }

    fn draw_rectangle(&mut self, rect: &Rectangle, line_width: FloatType) {
        let x0 = rect.min.x;
        let y0 = rect.min.y;
        let x1 = rect.max.x;
        let y1 = rect.max.y;

        self.draw_line(&Line::new(Vector2::new(x0, y0), Vector2::new(x1, y0)), line_width);
        self.draw_line(&Line::new(Vector2::new(x1, y0), Vector2::new(x1, y1)), line_width);
        self.draw_line(&Line::new(Vector2::new(x1, y1), Vector2::new(x0, y1)), line_width);
        self.draw_line(&Line::new(Vector2::new(x0, y1), Vector2::new(x0, y0)), line_width);
    }

    fn fill_triangle(&mut self, triangle: &Triangle) {
        let z = self.advance_layer();

        // Get current mesh
        let mesh = &mut self.meshes[self.current_mesh];
        let index_start: u32 = mesh.vertices.len() as u32;

        use shape::Bounded;
        let extent = triangle.bounding_box();

        for i in 0..3 {
            let point = triangle.points[i];

            // Calculate texture coordinates
            let tex_coord = (point - extent.min) / (extent.max - extent.min);

            mesh.vertices.push(
                Vertex::new([point.x as f32, point.y as f32, z])
                    .with_color(self.current_color)
                    .with_tex_coord([tex_coord.x as f32, 1.0 - tex_coord.y as f32])
            );
            mesh.indices.push(index_start + i as u32);
        }
    }
}
