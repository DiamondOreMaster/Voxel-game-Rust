use std::{fs, ffi::CString, ptr::{null, null_mut}};

pub struct Shader {
    pub shader_id: u32
}

pub enum ShaderType {
    VERTEX,
    FRAGMENT
}

impl Shader {
    pub fn new(filepath: &str, shader_type: ShaderType) -> Shader{
        let shader_string = fs::read_to_string(filepath).expect("Failed to read vertex shader!");

        let gl_type = Self::get_gl_shader_type(shader_type);

        let shader_id = unsafe { gl::CreateShader(gl_type) };
        let shader_source = CString::new(shader_string).expect("Failed to convert");
        unsafe { gl::ShaderSource(shader_id, 1, &shader_source.as_ptr(), null()) };
        unsafe { gl::CompileShader(shader_id) };

        Self::validate_shader(shader_id);

        Shader {
            shader_id,
        }
    }

    fn get_gl_shader_type(shader_type: ShaderType) -> u32{
        match shader_type {
            ShaderType::VERTEX => gl::VERTEX_SHADER,
            ShaderType::FRAGMENT => gl::FRAGMENT_SHADER
        }
    }

    fn validate_shader(shader: u32) {
        let mut success: i32 = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        }
    
        if success == gl::FALSE.into() {
            let mut info_log: [i8; 512] = [0; 512];
            unsafe {
                gl::GetShaderInfoLog(shader, 512, null_mut(), info_log.as_mut_ptr());
            }
            let output = unsafe { CString::from_raw(info_log.as_mut_ptr()) };
            println!("Shader compilation error: {}", output.to_string_lossy());
        }
    }
}