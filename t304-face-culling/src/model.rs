use wgpu::{util::DeviceExt, Buffer, Device};

use crate::{transform::Transform, vertex::Vertex};

#[derive(Debug)]
pub struct Model {
    pub draw_method: DrawMethod,
    // vertex: Vertex,
    // index: Vec<u16>,
    // transform_arr: Vec<Transform>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub transform_buffer: Buffer,
    pub vertices_len: u32,
    pub indices_len: u32,
    pub instance_num: u32,
}

impl Model {
    pub fn new(
        device: &Device,
        draw_method: DrawMethod,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        transform_arr: Vec<Transform>,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let transform_mat_arr = transform_arr
            .iter()
            .map(|t| t.to_raw_it())
            .collect::<Vec<_>>();

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&transform_mat_arr),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            draw_method,
            vertex_buffer,
            index_buffer,
            transform_buffer,
            vertices_len: vertices.len() as u32,
            indices_len: indices.len() as u32,
            instance_num: transform_arr.len() as u32,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum DrawMethod {
    Vertex,
    Index,
}
