extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};

use crate::constants::{HEIGHT, WIDTH};
use crate::scene::core::Scene;

mod algebra;
mod constants;
mod shaders;
mod sphere;
mod scene;

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

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut scene = Scene::new();

    while !window.should_close() {
        let current_time = glfw.get_time();

        scene.update(current_time);
        scene.render();

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
