

use gl;

mod shader;
use self::shader::Shader;

pub mod vertex;
pub use self::vertex::Vertex;

mod vertex_array;
use self::vertex_array::VertexArray;

/// Takes care of OpenGL rendering.
pub struct Renderer {
    shader: Shader,
    vertex_buffer: VertexArray
}


enum AttributeLocations {
    Position = 0,
    Color = 1,
}


impl Renderer {
    /// Create a new renderer
    pub fn new() -> Self {
        let mut shader = shader::Shader::from_source(
            include_bytes!("shaders/shader.vert"),
            include_bytes!("shaders/shader.frag")
        );

        shader.set_layout("position", AttributeLocations::Position as u32);

        use std::mem::size_of;
        let mut vertex_buffer = VertexArray::new();

        println!("Vertex: {}", size_of::<Vertex>());
        println!("position off: {}", offset_of!(Vertex, position));

        let stride = size_of::<Vertex>() as u32;

        // Setup vertex attributes
        vertex_buffer.set_attribute(AttributeLocations::Position as u32, 3, stride, offset_of!(Vertex, position) as u32);
        vertex_buffer.set_attribute(AttributeLocations::Color as u32, 4, stride, offset_of!(Vertex, color) as u32);

        Renderer {
            shader,
            vertex_buffer
        }
    }


    /// Set the color used to clear the screen
    pub fn set_clear_color(&mut self, color: (f32, f32, f32, f32)) {
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, color.3);
        }
    }


    /// Clear all render buffers
    pub fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }


    /// Submit a render batch to the renderer
    pub fn submit_batch(&mut self, batch: &RenderBatch) {
        // TODO: Set shader
        self.shader.bind();

        // TODO: Update vertex buffer
        self.vertex_buffer.upload_vertices(&batch.vertices);

        unsafe {
            gl::PointSize(2.0);
        }

        // TODO: Draw vertex arrays
        self.vertex_buffer.draw_vertices(0, batch.vertices.len(), gl::TRIANGLES);
    }
}


pub struct RenderBatch {
    pub vertices: Vec<Vertex>
}


impl RenderBatch {
}
