use std::f32::consts::PI;
use crate::algebra::Vec3;

pub fn generate_sphere_vertices(radius: f32, sectors: usize, stacks: usize) -> (Vec<f32>, Vec<u32>) {
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

            vertices.push(x);
            vertices.push(y);
            vertices.push(z);

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
