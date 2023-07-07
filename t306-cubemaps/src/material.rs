use wgpu::util::DeviceExt;

use crate::{core::Core, model::Model};

pub struct Material {
    pub diffuse: wgpu::TextureView,
    pub shininess: f32,

    pub bind_group: wgpu::BindGroup,
    pub model_arr: Vec<Model>,
}

#[derive(Debug, Clone, Copy)]
pub enum RenderMethod {
    NORMAL,
    REFLACT,
    REFRACT,
}

impl Material {
    pub fn new(
        diffuse: wgpu::TextureView,
        shininess: f32,
        render_method: RenderMethod,
        core: &Core,
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

        let render_method_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Render Method Buffer"),
            contents: bytemuck::cast_slice(&[render_method as u32]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        println!("render_method: {:?}", render_method);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: &core.material_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&core.texture_sampler),
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
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer(
                        render_method_buffer.as_entire_buffer_binding(),
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
