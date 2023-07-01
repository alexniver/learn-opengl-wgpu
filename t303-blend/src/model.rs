use wgpu::{util::DeviceExt, Buffer, Device};

use crate::{transform::Transform, vertex::Vertex};

pub struct Model {
    pub draw_method: DrawMethod,
    // vertex: Vertex,
    // index: Vec<u16>,
    pub transform_arr: Vec<Transform>,
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

        let transform_buffer = gen_transform_arr_buffer(device, &transform_arr);

        let instance_num = transform_arr.len() as u32;

        Self {
            draw_method,
            vertex_buffer,
            index_buffer,
            transform_arr,
            transform_buffer,
            vertices_len: vertices.len() as u32,
            indices_len: indices.len() as u32,
            instance_num,
        }
    }
}

fn gen_transform_arr_buffer(device: &Device, transform_arr: &Vec<Transform>) -> Buffer {
    let transform_mat_arr = transform_arr
        .iter()
        .map(|t| t.to_raw_it())
        .collect::<Vec<_>>();

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Transform Buffer"),
        contents: bytemuck::cast_slice(&transform_mat_arr),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

impl Model {
    pub fn update_transform_buffer(self: &mut Self, device: &Device) {
        let transform_buffer = gen_transform_arr_buffer(device, &self.transform_arr);
        self.transform_buffer = transform_buffer;
        self.instance_num = self.transform_arr.len() as u32;
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum DrawMethod {
    Vertex,
    Index,
}
