
use gl;
use gl::types::*;

pub struct Shader {
    program: GLuint
}


impl Shader {
    /// Create a new OpenGL shader program
    pub fn from_source(vertex_source: &[u8], fragment_source: &[u8]) -> Shader {
        let vertex_shader = Shader::compile_shader(vertex_source, gl::VERTEX_SHADER);
        let fragment_shader = Shader::compile_shader(fragment_source, gl::FRAGMENT_SHADER);

        let program: GLuint;

        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            // Check for link errors
            let mut link_status: GLint = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_status);

            if link_status != gl::TRUE as i32 {
                let mut log_length: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);

                let mut log = vec![0; log_length as usize];
                gl::GetProgramInfoLog(program, log_length, &mut log_length, log.as_mut_ptr());
                ;
                println!("{}", String::from_utf8(log.iter().map(|c| *c as u8).collect()).unwrap());
                panic!("Failed to link shader program!")
            }
        }

        Shader {
            program
        }
    }


    // Create and compile an OpenGL shader
    fn compile_shader(source: &[u8], shader_type: GLenum) -> GLuint {
        let shader: GLuint;
        unsafe {
            // Create shader
            shader = gl::CreateShader(shader_type);

            // Attach source to the shader
            gl::ShaderSource(shader, 1, [source.iter().map(|c| *c as i8).collect::<Vec<i8>>().as_ptr()].as_ptr(), [source.len() as i32].as_ptr());

            // Compile the shader
            gl::CompileShader(shader);

            // Check for compilation errors
            let mut compile_status: GLint = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compile_status);

            if compile_status != gl::TRUE as i32 {
                let mut log_length: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

                let mut log = vec![0; log_length as usize];
                gl::GetShaderInfoLog(shader, log_length, &mut log_length, log.as_mut_ptr());

                println!("{}", String::from_utf8(log.iter().map(|c| *c as u8).collect()).unwrap());
                panic!(format!("Failed to compile {} shader!", match shader_type {
                    gl::VERTEX_SHADER => "vertex",
                    gl::FRAGMENT_SHADER => "vertex",
                    gl::GEOMETRY_SHADER => "geometry",

                    _ => "unknown"
                }))
            }
        }

        // Return the shader
        shader
    }


    /// Set the location of an attribute
    pub fn set_layout(&mut self, name: &str, location: u32) {

        let c_name: Vec<i8> = name.to_owned().into_bytes().into_iter().map(|c| c as i8).collect();
        unsafe {
            gl::BindAttribLocation(self.program, location, c_name.as_ptr());
        }
    }


    /// Use this shader program
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }


    /// Get the location of an uniform in the shader
    pub fn get_uniform_location(&self, name: &[u8]) -> i32 {
        let c_name = name.iter().map(|e| *e as i8).collect::<Vec<i8>>();

        unsafe {
            gl::GetUniformLocation(self.program, c_name.as_ptr())
        }
    }
}
