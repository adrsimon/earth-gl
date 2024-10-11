use crate::core::algebra::{Mat4, Vec3};
use crate::shaders::core::load_shader;
use std::ffi::CString;
use std::ptr;

pub struct ShaderProgram {
    pub id: gl::types::GLuint,
}

impl ShaderProgram {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let program = unsafe {
            let vertex_shader = load_shader(vertex_path, gl::VERTEX_SHADER);
            let fragment_shader = load_shader(fragment_path, gl::FRAGMENT_SHADER);

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = gl::FALSE as gl::types::GLint;
            let mut info_log = Vec::with_capacity(512 - 1);
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}",
                    String::from_utf8_lossy(&info_log)
                );
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader_program
        };

        ShaderProgram { id: program }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4(&self, name: &str, value: &Mat4) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
        }
    }

    pub fn set_vec3(&self, name: &str, value: &Vec3) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }
}
