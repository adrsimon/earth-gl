use crate::algebra::{Mat4, Vec3};
use crate::constants::{HEIGHT, WIDTH};
use crate::scene::camera::{perspective, Camera};
use crate::scene::light::Light;
use crate::shaders::program::ShaderProgram;
use crate::sphere::Sphere;

use std::f32::consts::PI;

pub struct Scene {
    shader_program: ShaderProgram,
    pub(crate) sphere: Sphere,
    pub(crate) camera: Camera,
    light: Light,
}

impl Scene {
    pub fn new() -> Self {
        let shader_program =
            ShaderProgram::new("src/shaders/shader.vert", "src/shaders/shader.frag");
        let sphere = Sphere::new(0.5, 30, 30);
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let light = Light::new(Vec3::new(1.0, 1.0, 2.0), Vec3::new(1.0, 1.0, 1.0));

        Scene {
            shader_program,
            sphere,
            camera,
            light,
        }
    }

    pub fn update(&mut self, time: f64) {
        let radius = 3.0;
        let cam_x = radius * (time as f32 * 0.5).sin();
        let cam_z = radius * (time as f32 * 0.5).cos();
        self.camera.position = Vec3::new(cam_x, 0.0, cam_z);
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

        self.sphere.draw();
    }
}
