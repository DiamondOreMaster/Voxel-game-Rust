use crate::shader::{Shader, ShaderType};

pub struct ShaderProgram {
    pub program_id: u32,
}

impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let vertex_shader = Shader::new(vertex_shader_path, ShaderType::VERTEX);
        let fragment_shader = Shader::new(fragment_shader_path, ShaderType::FRAGMENT);

        let program_id: u32;
        unsafe {
            program_id = gl::CreateProgram();

            gl::AttachShader(program_id, vertex_shader.shader_id);
            gl::AttachShader(program_id, fragment_shader.shader_id);
            gl::LinkProgram(program_id);
        }
        

        ShaderProgram {
            program_id
        }
    }

    pub fn bind(&self) {
        unsafe{ gl::UseProgram(self.program_id) }
    }
}