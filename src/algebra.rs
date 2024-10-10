#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> Vec3 {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

pub struct Mat4 {
    data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Mat4 { data }
    }

    pub fn multiply(&self, other: &Mat4) -> Mat4 {
        let mut result = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] =
                    self.data[i][0] * other.data[0][j] +
                        self.data[i][1] * other.data[1][j] +
                        self.data[i][2] * other.data[2][j] +
                        self.data[i][3] * other.data[3][j];
            }
        }
        result
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
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
    let f = Vec3::new(
        center.x - eye.x,
        center.y - eye.y,
        center.z - eye.z,
    ).normalize();
    let s = f.cross(&up).normalize();
    let u = s.cross(&f);

    Mat4::new([
        [s.x, u.x, -f.x, 0.0],
        [s.y, u.y, -f.y, 0.0],
        [s.z, u.z, -f.z, 0.0],
        [-s.x * eye.x - s.y * eye.y - s.z * eye.z,
            -u.x * eye.x - u.y * eye.y - u.z * eye.z,
            f.x * eye.x + f.y * eye.y + f.z * eye.z,
            1.0],
    ])
}