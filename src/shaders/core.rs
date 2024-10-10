use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

pub fn load_shader(path: &str, shader_type: gl::types::GLenum) -> gl::types::GLuint {
    let mut file = File::open(path).unwrap();
    let mut shader_source = String::new();
    file.read_to_string(&mut shader_source).unwrap();

    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(shader_source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as gl::types::GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
            println!("ERROR::SHADER::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }

        shader
    }
}