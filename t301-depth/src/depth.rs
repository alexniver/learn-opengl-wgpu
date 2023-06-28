use wgpu::{
    util::DeviceExt, BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BlendState,
    Buffer, BufferUsages, ColorWrites, Device, Face, RenderPipeline, Sampler, ShaderStages,
    SurfaceConfiguration, TextureView, VertexState,
};

use crate::{texture::gen_texture_depth, vertex::Vertex};

pub struct Depth {
    pub texture_depth: TextureView,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_len: u32,
    bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
    pub render_pipline: RenderPipeline,
    pub sampler: Sampler,
}

impl Depth {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let vertices = [
            Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
            Vertex::new([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
            Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
            Vertex::new([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
        ];
        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Depth Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Depth Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Depth Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });

        let render_pipline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Depth Render Pipline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = std::fs::read_to_string("assets/shader/depth.wgsl").unwrap();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Depth Shader Module"),
            source: wgpu::ShaderSource::Wgsl(shader.into()),
        });
        let render_pipline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Depth Render Pipline"),
            layout: Some(&render_pipline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::vertex_buffer_layout()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Depth Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: None,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        let texture_depth = gen_texture_depth(&device, &surface_config);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Depth Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_depth),
                },
            ],
        });

        Self {
            texture_depth,
            vertex_buffer,
            index_buffer,
            index_len: indices.len() as _,
            bind_group_layout,
            bind_group,
            render_pipline,
            sampler,
        }
    }

    pub fn resize(&mut self, device: &Device, surface_config: &SurfaceConfiguration) {
        self.texture_depth = gen_texture_depth(device, surface_config);
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Depth Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.texture_depth),
                },
            ],
        });
    }
}
