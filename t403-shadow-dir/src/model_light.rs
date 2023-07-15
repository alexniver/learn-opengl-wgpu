use wgpu::{util::DeviceExt, Buffer, BufferUsages, Device};

use crate::{transform::Transform, vertex_light::VertexLight};

pub struct ModelLight {
    pub vertex_buffer: Buffer,
    pub transform_buffer: Buffer,
    pub vertices_len: u32,
    pub instance_len: u32,
}

impl ModelLight {
    pub fn new(
        device: &Device,
        vertex_light_arr: Vec<VertexLight>,
        transform_arr: Vec<Transform>,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_light_arr),
            usage: BufferUsages::VERTEX,
        });

        let transform_mat_arr = transform_arr.iter().map(|t| t.to_raw()).collect::<Vec<_>>();

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&transform_mat_arr),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            transform_buffer,
            vertices_len: vertex_light_arr.len() as _,
            instance_len: transform_arr.len() as _,
        }
    }
}
