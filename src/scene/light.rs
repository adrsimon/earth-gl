use crate::algebra::Vec3;
use crate::shaders::program::ShaderProgram;

pub struct Light {
    position: Vec3,
    color: Vec3,
}

impl Light {
    pub fn new(position: Vec3, color: Vec3) -> Self {
        Light { position, color }
    }

    pub fn update_shader(&self, shader: &ShaderProgram) {
        shader.set_vec3("lightPos", &self.position);
        shader.set_vec3("lightColor", &self.color);
    }
}
