
use gl;
use std::mem::size_of;

use super::shader::Shader;
use super::vertex::Vertex;
use super::vertex_array::VertexArray;

use super::RenderBatch;

use super::texture::Texture;


/// Takes care of OpenGL rendering.
pub struct Renderer {
    // Shader to use when rendering
    shader: Shader,

    // Buffers vertex and index data to the GPU before rendering
    vertex_buffer: VertexArray,


    // Location of all the uniforms in the shader
    uniforms: UniformLocations,

    // Texture that is used when no other is provided
    default_texture: Texture
}


/// Locations of all the attributes in the shader
enum AttributeLocations {
    Position = 0,
    Color = 1,
    TexCoord = 2
}

/// Locations of all the uniforms in the shader
struct UniformLocations {
    translation: i32,
    scale: i32,
    layers: i32
}


impl Renderer {
    /// Create a new renderer
    pub fn new(_window: &::window::Window) -> Self {
        let mut shader = Shader::from_source(
            include_bytes!("shaders/shader.vert"),
            include_bytes!("shaders/shader.frag")
        );

        shader.set_layout("position", AttributeLocations::Position as u32);
        shader.set_layout("color", AttributeLocations::Color as u32);
        shader.set_layout("texCoord", AttributeLocations::TexCoord as u32);

        let mut vertex_buffer = VertexArray::new();

        // Setup vertex attributes
        let stride = size_of::<Vertex>() as u32;
        vertex_buffer.set_attribute(AttributeLocations::Position as u32, 3, stride, offset_of!(Vertex, position) as u32);
        vertex_buffer.set_attribute(AttributeLocations::Color as u32, 4, stride, offset_of!(Vertex, color) as u32);
        vertex_buffer.set_attribute(AttributeLocations::TexCoord as u32, 2, stride, offset_of!(Vertex, tex_coord) as u32);

        let uniforms = UniformLocations {
            translation: shader.get_uniform_location(b"translation\0"),
            scale: shader.get_uniform_location(b"scale\0"),
            layers: shader.get_uniform_location(b"layers\0")
        };

        unsafe {
            // Enable depth test
            gl::Enable(gl::DEPTH_TEST);

            // Enable alpha opacity
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Renderer {
            shader,
            vertex_buffer,

            uniforms,

            default_texture: Texture::default()
        }
    }


    /// Set the color used to clear the screen
    pub fn set_clear_color(&mut self, color: [f32; 4]) {
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
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
        // Set shader
        self.shader.bind();


        // Set uniforms
        let (translation, scale) = batch.view.get_transformation();

        unsafe {
            gl::Uniform2f(self.uniforms.translation, translation[0], translation[1]);
            gl::Uniform2f(self.uniforms.scale, scale[0], scale[1]);
            gl::Uniform1ui(self.uniforms.layers, batch.layer_count);
        }

        // Bind texture
        if let Some(ref texture) = batch.texture {
            texture.bind();
        } else {
            self.default_texture.bind();
        }

        // Update vertex buffer
        self.vertex_buffer.upload_vertices(&batch.vertices);
        self.vertex_buffer.upload_indices(&batch.indices);

        // Draw indices
        self.vertex_buffer.draw_indices(0, batch.indices.len(), gl::TRIANGLES);
    }
}
