
use super::Vertex;
use super::view::{View, BoundedView};
use super::texture::Texture;

use super::mesh::Mesh;

use ::shape::{RenderShape, Rectangle, Line};

use std::f32::consts::PI;
use std::collections::HashMap;

pub struct RenderBatch {
    pub(super) mesh_indices: HashMap<Texture, usize>,
    pub(super) meshes: Vec<Mesh>,

    pub(super) layer_count: u32,

    current_color: [f32; 4],
    current_mesh: usize,

    default_texture: Texture,

    pub(super) view: Box<View>
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

            view: Box::new(BoundedView::default())
        }
    }

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
        self.view = Box::new(BoundedView::default());
    }


    /// Set the current view
    pub fn set_view<V>(&mut self, view: V)
        where V: View + 'static
    {
        self.view = Box::new(view);
    }


    /// Set the current fill color
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.current_color = color;
    }


    /// Set the current texture
    pub fn set_texture(&mut self, texture: Option<Texture>) {
        if let Some(texture) = texture {
            if self.mesh_indices.contains_key(&texture) {
                self.current_mesh = self.mesh_indices[&texture];
            } else {
                let mesh_index = self.mesh_indices.len();
                self.mesh_indices.insert(texture, mesh_index);
                while mesh_index >= self.meshes.len() {
                    self.meshes.push(Mesh::new());
                }
                self.current_mesh = mesh_index;
            }
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
    pub fn draw_circle_segments(&mut self, center: [f32; 2], radius: f32, segments: u32) {
        let x: f32 = center[0];
        let y: f32 = center[1];

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
    pub fn draw_circle(&mut self, center: [f32; 2], radius: f32) {
        self.draw_circle_segments(center, radius, 16);
    }
}


impl RenderShape for RenderBatch {
    fn draw_line(&mut self, line: &Line, width: f32) {
        // Find the line perpendicular to the line
        let d = line.get_direction();
        let p = ::vec2_perp(d);

        // "Radius" of the line
        let r = width / 2.0;

        // Calculate scaled perpendicular, used in Minkowski addition
        let dr = ::vec2_scale(r, d);
        let pr = ::vec2_scale(r, p);

        // Calculate adjusted endpoints
        let start = ::vec2_sub(line.start, dr);
        let end = ::vec2_add(line.end, dr);

        // Calculate corners of line using Minkowski addition
        let a = ::vec2_add(start, pr);
        let b = ::vec2_sub(start, pr);

        let c = ::vec2_add(end, pr);
        let d = ::vec2_sub(end, pr);

        let z = self.advance_layer();

        // Get current mesh
        let mesh = &mut self.meshes[self.current_mesh];


        // Construct line vertices
        let index_start: u32 = mesh.vertices.len() as u32;

        mesh.vertices.push(
            Vertex::new([b[0], b[1], z])
                .with_color(self.current_color)
                .with_tex_coord([0.0, 1.0])
        );
        mesh.vertices.push(
            Vertex::new([d[0], d[1], z])
                .with_color(self.current_color)
                .with_tex_coord([1.0, 1.0])
        );
        mesh.vertices.push(
            Vertex::new([c[0], c[1], z])
                .with_color(self.current_color)
                .with_tex_coord([1.0, 0.0])
        );
        mesh.vertices.push(
            Vertex::new([a[0], a[1], z])
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
        let x: f32 = rect.center[0] - rect.size[0] / 2.0;
        let y: f32 = rect.center[1] - rect.size[1] / 2.0;
        let w: f32 = rect.size[0];
        let h: f32 = rect.size[1];

        let z = self.advance_layer();

        // Get current mesh
        let mesh = &mut self.meshes[self.current_mesh];

        let index_start: u32 = mesh.vertices.len() as u32;

        mesh.vertices.push(
            Vertex::new([x,     y,     z])
                .with_color(self.current_color)
                .with_tex_coord([0.0, 1.0])
        );
        mesh.vertices.push(
            Vertex::new([x + w, y,     z])
                .with_color(self.current_color)
                .with_tex_coord([1.0, 1.0])
        );
        mesh.vertices.push(
            Vertex::new([x + w, y + h, z])
                .with_color(self.current_color)
                .with_tex_coord([1.0, 0.0])
        );
        mesh.vertices.push(
            Vertex::new([x,     y + h, z])
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

    fn draw_rectangle(&mut self, rect: &Rectangle, line_width: f32) {
        let x: f32 = rect.center[0] - rect.size[0] / 2.0;
        let y: f32 = rect.center[1] - rect.size[1] / 2.0;
        let w: f32 = rect.size[0];
        let h: f32 = rect.size[1];

        self.draw_line(&Line::new([x,     y],     [x + w, y]),     line_width);
        self.draw_line(&Line::new([x + w, y],     [x + w, y + h]), line_width);
        self.draw_line(&Line::new([x + w, y + h], [x,     y + h]), line_width);
        self.draw_line(&Line::new([x,     y + h], [x,     y]),     line_width);
    }
}
