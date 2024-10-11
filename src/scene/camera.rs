use crate::algebra::mat4::Mat4;
use crate::algebra::vec3::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub up: Vec3,
    pub front: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    pub speed: f32,
}

impl Camera {
    pub fn new(position: Vec3, front: Vec3, world_up: Vec3) -> Self {
        let right = front.cross(&world_up).normalize();
        let up = right.cross(&front).normalize();

        Camera {
            position,
            front,
            up,
            right,
            world_up,
            speed: 0.2,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(self.position, self.position + self.front, self.up)
    }

    pub fn pan(&mut self, direction: Vec3, speed: f32) {
        self.position += direction * speed;
    }

    pub fn update_camera_vectors(&mut self) {
        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }
}

pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y / 2.0).tan();
    Mat4::new([
        [f / aspect, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (far + near) / (near - far), -1.0],
        [0.0, 0.0, (2.0 * far * near) / (near - far), 0.0],
    ])
}

pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let f = Vec3::new(center.x - eye.x, center.y - eye.y, center.z - eye.z).normalize();
    let s = f.cross(&up).normalize();
    let u = s.cross(&f);

    Mat4::new([
        [s.x, u.x, -f.x, 0.0],
        [s.y, u.y, -f.y, 0.0],
        [s.z, u.z, -f.z, 0.0],
        [
            -s.x * eye.x - s.y * eye.y - s.z * eye.z,
            -u.x * eye.x - u.y * eye.y - u.z * eye.z,
            f.x * eye.x + f.y * eye.y + f.z * eye.z,
            1.0,
        ],
    ])
}
