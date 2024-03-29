use wgpu::{util::DeviceExt, BindGroupLayout, Sampler};

use crate::{model::Model, pipe_hub::PipeHub};

pub struct Material {
    pub diffuse: wgpu::TextureView,
    pub shininess: f32,

    pub bind_group: wgpu::BindGroup,
    pub model_arr: Vec<Model>,
}

impl Material {
    pub fn new(
        diffuse: wgpu::TextureView,
        shininess: f32,
        core: &PipeHub,
        material_bind_group_layout: &BindGroupLayout,
        texture_sampler: &Sampler,
    ) -> Self {
        let device = &core.device;

        let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Color Buffer"),
            contents: bytemuck::cast_slice(&[1.0, 0.5, 0.31]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let shininess_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::bytes_of(&shininess),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: material_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&diffuse),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        color_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(
                        shininess_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        Self {
            diffuse,
            shininess,
            bind_group,
            model_arr: vec![],
        }
    }

    pub fn add_model(&mut self, model: Model) {
        self.model_arr.push(model);
    }
}
