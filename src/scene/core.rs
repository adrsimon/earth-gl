use crate::algebra::mat4::Mat4;
use crate::algebra::vec3::Vec3;
use crate::core::constants::{HEIGHT, WIDTH};
use crate::objects::objects::Object;
use crate::scene::camera::{perspective, Camera, CameraMovement};
use crate::scene::light::Light;
use crate::shaders::program::ShaderProgram;

use std::f32::consts::PI;

pub struct Scene {
    shader_program: ShaderProgram,
    pub instances: Vec<Box<dyn Object>>,
    pub camera: Camera,
    light: Light,
}

impl Scene {
    pub fn new() -> Self {
        let shader_program =
            ShaderProgram::new("src/shaders/shader.vert", "src/shaders/shader.frag");
        let instances = Vec::new();
        let camera = Camera::new(Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 1.0, 0.0));

        let light = Light::new(Vec3::new(1.0, 1.0, 2.0), Vec3::new(1.0, 1.0, 1.0));

        Scene {
            shader_program,
            instances,
            camera,
            light,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.camera.update(delta_time);
    }

    pub fn render(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.shader_program.use_program();

        let model = Mat4::identity();
        let view = self.camera.get_view_matrix();
        let projection = perspective(45.0 * PI / 180.0, WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);

        self.shader_program.set_mat4("model", &model);
        self.shader_program.set_mat4("view", &view);
        self.shader_program.set_mat4("projection", &projection);
        self.shader_program
            .set_vec3("viewPos", &self.camera.position);

        self.light.update_shader(&self.shader_program);

        for instance in self.instances.iter() {
            instance.draw();
        }
    }

    pub fn push_instance(&mut self, instance: impl Object + 'static) {
        self.instances.push(Box::new(instance));
    }

    pub fn toggle_wireframe(&mut self) {
        unsafe {
            let mut polygon_mode: gl::types::GLint = 0;
            gl::GetIntegerv(gl::POLYGON_MODE, &mut polygon_mode);
            let new_mode = if polygon_mode == gl::LINE as gl::types::GLint {
                gl::FILL
            } else {
                gl::LINE
            };
            gl::PolygonMode(gl::FRONT_AND_BACK, new_mode);
        }
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, pressed: bool) {
        self.camera.process_keyboard(direction, pressed);
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32) {
        self.camera.process_mouse_movement(x_offset, y_offset, true);
    }
}
