use crate::algebra::vec3::Vec3;
use crate::objects::objects::{Object, Position};

use std::f32::consts::PI;
use std::ptr;

pub struct Sphere {
    pub vao: gl::types::GLuint,
    pub index_count: usize,
}

pub struct SphereResolution {
    pub sectors: usize,
    pub stacks: usize,
}

impl Sphere {
    pub fn new(radius: f32, position: Position, res: SphereResolution) -> Self {
        let (vertices, indices) =
            generate_sphere_vertices(radius, position, res.sectors, res.stacks);
        let index_count = indices.len();

        let (vao, _vbo, _ebo) = unsafe {
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

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                (3 * size_of::<gl::types::GLfloat>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            (vao, vbo, ebo)
        };

        Sphere { vao, index_count }
    }
}

impl Object for Sphere {
    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}

pub fn generate_sphere_vertices(
    radius: f32,
    position: Position,
    sectors: usize,
    stacks: usize,
) -> (Vec<f32>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let sector_step = 2.0 * PI / sectors as f32;
    let stack_step = PI / stacks as f32;

    for i in 0..=stacks {
        let stack_angle = PI / 2.0 - i as f32 * stack_step;
        let xy = radius * stack_angle.cos();
        let z = radius * stack_angle.sin();

        for j in 0..=sectors {
            let sector_angle = j as f32 * sector_step;
            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            vertices.push(x + position.x);
            vertices.push(y + position.y);
            vertices.push(z + position.z);

            let normal = Vec3::new(x, y, z).normalize();
            vertices.push(normal.x);
            vertices.push(normal.y);
            vertices.push(normal.z);
        }
    }

    for i in 0..stacks {
        let k1 = i * (sectors + 1);
        let k2 = k1 + sectors + 1;

        for j in 0..sectors {
            if i != 0 {
                indices.push((k1 + j) as u32);
                indices.push((k2 + j) as u32);
                indices.push((k1 + j) as u32 + 1);
            }

            if i != (stacks - 1) {
                indices.push((k1 + j) as u32 + 1);
                indices.push((k2 + j) as u32);
                indices.push((k2 + j) as u32 + 1);
            }
        }
    }

    (vertices, indices)
}
