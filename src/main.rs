extern crate gl;
extern crate glfw;

mod algebra;
mod core;
mod shaders;

use std::ffi::CString;
use std::ptr;
use std::f32::consts::PI;

use glfw::{Action, Context, Key};

use crate::algebra::{look_at, perspective, Mat4, Vec3};
use crate::core::generate_sphere_vertices;
use crate::shaders::core::load_shader;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "OpenGL Sphere", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader_program, vao, index_count) = unsafe {
        let vertex_shader = load_shader("src/shaders/shader.vert", gl::VERTEX_SHADER);
        let fragment_shader = load_shader("src/shaders/shader.frag", gl::FRAGMENT_SHADER);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = gl::FALSE as gl::types::GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
            println!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let (vertices, indices) = generate_sphere_vertices(0.5, 30, 30);
        let index_count = indices.len();

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * size_of::<gl::types::GLfloat>() as gl::types::GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * size_of::<gl::types::GLfloat>() as gl::types::GLsizei, (3 * size_of::<gl::types::GLfloat>()) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::Enable(gl::DEPTH_TEST);

        (shader_program, vao, index_count)
    };

    let camera_target = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let light_pos = Vec3::new(1.0, 1.0, 2.0);

    while !window.should_close() {
        let current_time = glfw.get_time();

        let radius = 3.0;
        let cam_x = radius * (current_time as f32 * 0.5).sin();
        let cam_z = radius * (current_time as f32 * 0.5).cos();
        let camera_pos = Vec3::new(cam_x, 0.0, cam_z);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(shader_program);

            let model = Mat4::identity();
            let view = look_at(camera_pos, camera_target, up);
            let projection = perspective(45.0 * PI / 180.0, WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);

            let model_cstr = CString::new("model").unwrap();
            let view_cstr = CString::new("view").unwrap();
            let projection_cstr = CString::new("projection").unwrap();
            let light_pos_cstr = CString::new("lightPos").unwrap();
            let view_pos_cstr = CString::new("viewPos").unwrap();

            let model_loc = gl::GetUniformLocation(shader_program, model_cstr.as_ptr());
            let view_loc = gl::GetUniformLocation(shader_program, view_cstr.as_ptr());
            let projection_loc = gl::GetUniformLocation(shader_program, projection_cstr.as_ptr());
            let light_pos_loc = gl::GetUniformLocation(shader_program, light_pos_cstr.as_ptr());
            let view_pos_loc = gl::GetUniformLocation(shader_program, view_pos_cstr.as_ptr());

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());
            gl::Uniform3f(light_pos_loc, light_pos.x, light_pos.y, light_pos.z);
            gl::Uniform3f(view_pos_loc, camera_pos.x, camera_pos.y, camera_pos.z);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, index_count as i32, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                _ => {}
            }
        }
    }
}
