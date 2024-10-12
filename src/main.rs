extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key, MouseButton, WindowEvent};

use crate::core::constants::{HEIGHT, WIDTH};
use crate::objects::objects::Position;
use crate::objects::sphere::{Sphere, SphereResolution};
use crate::scene::camera::CameraMovement;
use crate::scene::core::Scene;

mod algebra;
mod core;
mod objects;
mod scene;
mod shaders;

use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, "OpenGL Sphere", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut scene = Scene::new();
    scene.push_instance(Sphere::new(
        1.0,
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        SphereResolution {
            sectors: 64,
            stacks: 64,
        },
    ));

    scene.push_instance(Sphere::new(
        0.5,
        Position {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        SphereResolution {
            sectors: 64,
            stacks: 64,
        },
    ));

    let mut last_frame_time = glfw.get_time();
    let mut last_x = window.get_size().0 as f32 / 2.0;
    let mut last_y = window.get_size().1 as f32 / 2.0;
    let mut first_mouse = true;
    let mut right_mouse_pressed = false;

    while !window.should_close() {
        let current_time = glfw.get_time();
        let delta_time = (current_time - last_frame_time) as f32;
        last_frame_time = current_time;

        scene.update(delta_time);
        scene.render();

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                WindowEvent::Key(Key::Tab, _, Action::Press, _) => {
                    scene.toggle_wireframe();
                }
                WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    scene.process_keyboard(CameraMovement::Forward, true);
                }
                WindowEvent::Key(Key::W, _, Action::Release, _) => {
                    scene.process_keyboard(CameraMovement::Forward, false);
                }
                WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    scene.process_keyboard(CameraMovement::Backward, true);
                }
                WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    scene.process_keyboard(CameraMovement::Backward, false);
                }
                WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    scene.process_keyboard(CameraMovement::Left, true);
                }
                WindowEvent::Key(Key::A, _, Action::Release, _) => {
                    scene.process_keyboard(CameraMovement::Left, false);
                }
                WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    scene.process_keyboard(CameraMovement::Right, true);
                }
                WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    scene.process_keyboard(CameraMovement::Right, false);
                }
                WindowEvent::MouseButton(MouseButton::Button2, Action::Press, _) => {
                    right_mouse_pressed = true;
                    first_mouse = true;
                }
                WindowEvent::MouseButton(MouseButton::Button2, Action::Release, _) => {
                    right_mouse_pressed = false;
                }
                WindowEvent::CursorPos(x_pos, y_pos) => {
                    let x_pos = x_pos as f32;
                    let y_pos = y_pos as f32;

                    if right_mouse_pressed {
                        if first_mouse {
                            last_x = x_pos;
                            last_y = y_pos;
                            first_mouse = false;
                        } else {
                            let x_offset = x_pos - last_x;
                            let y_offset = y_pos - last_y;

                            scene.process_mouse_movement(x_offset, y_offset);
                        }

                        last_x = x_pos;
                        last_y = y_pos;
                    }
                }
                _ => {}
            }
        }
    }
}
