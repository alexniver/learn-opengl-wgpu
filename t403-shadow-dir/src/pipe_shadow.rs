use glam::{Mat4, Vec3};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, IndexFormat,
    Queue, RenderPipeline, SurfaceConfiguration, TextureView,
};

use crate::{
    light_direction::LightDirection,
    material::Material,
    model::DrawMethod,
    pipe_mesh::SAMPLE_COUNT,
    texture::{gen_texture_view_depth, DEPTH_FORMAT},
    transform::TransformRawIT,
    vertex::Vertex,
};

// const SAMPLE_COUNT: u32 = 1;

pub struct PipeShadow {
    pub render_pipeline: RenderPipeline,
    pub texture_view_depth: TextureView,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
    pub buffer_view_proj: Buffer,
    pub proj: Mat4,
}

impl PipeShadow {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let texture_view_depth = gen_texture_view_depth(device, surface_config, SAMPLE_COUNT);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout Shadow"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout Pipe Shadow"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let shader = std::fs::read_to_string("assets/shader/shadow.wgsl").unwrap();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Shadow"),
            source: wgpu::ShaderSource::Wgsl(shader.into()),
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline Pipe Shadow"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::vertex_buffer_layout(),
                    TransformRawIT::vertex_buffer_layout(),
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2, // Corresponds to bilinear filtering
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState {
                count: SAMPLE_COUNT,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: None,
            multiview: None,
        });

        let view = Mat4::look_to_rh(Vec3::ZERO, Vec3::ZERO, Vec3::Y);
        let proj = Mat4::orthographic_rh(-10.0, 10.0, -10.0, 10.0, 1.0, 10.0);
        let buffer_view_proj = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer Shadow View Proj"),
            contents: bytemuck::cast_slice(&(proj.mul_mat4(&view)).to_cols_array_2d()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group Shadow"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    buffer_view_proj.as_entire_buffer_binding(),
                ),
            }],
        });

        Self {
            render_pipeline,
            texture_view_depth,
            bind_group_layout,
            bind_group,
            buffer_view_proj,
            proj,
        }
    }

    pub fn render(&mut self, encoder: &mut CommandEncoder, material_arr: &Vec<Material>) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass Shadow"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.texture_view_depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        for material in material_arr {
            // render model
            for model in &material.model_arr {
                if model.draw_method == DrawMethod::Vertex {
                    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, model.transform_buffer.slice(..));
                    render_pass.draw(0..model.vertices_len, 0..model.instance_num);
                } else {
                    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, model.transform_buffer.slice(..));
                    render_pass.set_index_buffer(model.index_buffer.slice(..), IndexFormat::Uint32);
                    render_pass.draw_indexed(0..model.indices_len, 0, 0..model.instance_num);
                }
            }
        }
    }

    pub fn resize(&mut self, device: &Device, surface_config: &SurfaceConfiguration) {
        self.texture_view_depth = gen_texture_view_depth(device, surface_config, SAMPLE_COUNT);
    }

    pub fn set_light_direction(
        &mut self,
        queue: &Queue,
        light_pos: Vec3,
        light_direction: &LightDirection,
    ) {
        let view = Mat4::look_to_rh(light_pos, light_direction.dir.into(), Vec3::Y);
        queue.write_buffer(
            &self.buffer_view_proj,
            0,
            bytemuck::cast_slice(&(self.proj.mul_mat4(&view)).to_cols_array_2d()),
        );
    }
}
