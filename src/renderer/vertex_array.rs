
use gl;
use gl::types::*;

use ::Vertex;

pub struct VertexArray {
    handle: GLuint,
    vertex_buffer: Buffer
}

impl VertexArray {
    pub fn new() -> VertexArray {

        let mut handle: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut handle);
            gl::BindVertexArray(handle);
        }

        let mut vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
        vertex_buffer.bind();

        VertexArray {
            handle,
            vertex_buffer
        }
    }


    /// Setup the shader attributes
    pub fn set_attribute(&mut self, location: u32, size: u32, stride: u32, offset: u32) {
        use std::os::raw::c_void;

        unsafe {
            gl::BindVertexArray(self.handle);
            self.vertex_buffer.bind();

            gl::EnableVertexAttribArray(location);
            gl::VertexAttribPointer(location, size as i32, gl::FLOAT, gl::FALSE, stride as i32, offset as *const c_void)
        }
    }


    /// Upload vertices into the vertex buffer
    pub fn upload_vertices(&mut self, vertices: &[Vertex]) {
        self.vertex_buffer.upload(vertices);
    }


    /// Draw vertices
    pub fn draw_vertices(&mut self, offset: usize, count: usize, mode: GLenum) {
        unsafe {
            gl::BindVertexArray(self.handle);

            gl::DrawArrays(mode, offset as GLint, count as GLsizei);
        }
    }
}



struct Buffer {
    handle: GLuint,
    target: GLenum
}


impl Buffer {
    /// Create a new buffer
    pub fn new(buffer_type: GLenum) -> Buffer {
        let mut handle: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut handle);
        }

        Buffer {
            handle,
            target: buffer_type
        }
    }



    /// Bind this buffer
    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(self.target, self.handle);
        }
    }


    /// Upload data to the buffer
    pub fn upload<T>(&mut self, data: &[T]) {
        use std::mem::size_of;
        self.bind();

        let c_data = data.as_ptr() as *const GLvoid;
        let t_size = size_of::<T>();
        let t_count = data.len();
        let data_size = t_count * t_size;

        unsafe {
            gl::BufferData(self.target, data_size as isize, c_data, gl::DYNAMIC_DRAW);
        }
    }
}

