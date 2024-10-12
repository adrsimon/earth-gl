use crate::algebra::mat4::Mat4;
use crate::algebra::vec3::Vec3;

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub velocity: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, world_up: Vec3) -> Self {
        let mut camera = Camera {
            position,
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            world_up,
            yaw: -90.0,
            pitch: 0.0,
            movement_speed: 2.0,
            mouse_sensitivity: 0.1,
            velocity: Vec3::new(0.0, 0.0, 0.0),
        };
        camera.update_camera_vectors();
        camera
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(self.position, self.position + self.front, self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, pressed: bool) {
        let speed = if pressed { self.movement_speed } else { 0.0 };
        match direction {
            CameraMovement::Forward => self.velocity.z = speed,
            CameraMovement::Backward => self.velocity.z = -speed,
            CameraMovement::Left => self.velocity.x = -speed,
            CameraMovement::Right => self.velocity.x = speed,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.front * self.velocity.z * delta_time;
        self.position += self.right * self.velocity.x * delta_time;
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch -= y_offset;

        if constrain_pitch {
            self.pitch = self.pitch.clamp(-89.0, 89.0);
        }

        self.update_camera_vectors();
    }

    fn update_camera_vectors(&mut self) {
        let front = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
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
