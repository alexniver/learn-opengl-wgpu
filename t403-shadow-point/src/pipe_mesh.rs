use glam::Mat4;
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, IndexFormat,
    Queue, RenderPipeline, Sampler, SurfaceConfiguration, TextureView,
};

pub const SAMPLE_COUNT: u32 = 4;

use crate::{
    camera::Camera,
    input::Input,
    light_direction::LightDirection,
    light_point::LightPoint,
    light_spot::LightSpot,
    material::Material,
    model::DrawMethod,
    texture::{
        self, gen_sampler_clamp, gen_sampler_repeat, gen_texture_depth, gen_texture_view_msaa,
    },
    transform::TransformRawIT,
    vertex::Vertex,
};

pub struct PipeMesh {
    pub render_pipline_mesh: RenderPipeline,

    pub sampler: Sampler,
    pub sampler_repeat: Sampler,

    pub texture_view_depth: TextureView,
    pub texture_view_msaa: TextureView,

    pub bind_group_layout_material: BindGroupLayout,
    pub material_arr: Vec<Material>,

    pub camera: Camera,
    pub bind_group_layout_camera: BindGroupLayout,
    pub bind_group_camera: BindGroup,
    pub buffer_view_proj: Buffer,
    pub buffer_camera_pos: Buffer,
    pub sampler_view_shadow_map: Sampler,

    pub light_direction_arr: Vec<LightDirection>,
    pub light_point_arr: Vec<LightPoint>,
    pub light_spot_arr: Vec<LightSpot>,
    pub bind_group_light_arr: BindGroup,
    pub buffer_light_direction: Buffer,
    pub buffer_light_point: Buffer,
    pub buffer_light_spot: Buffer,
}

impl PipeMesh {
    pub fn new(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        texture_view_shadow_map: &TextureView,
    ) -> Self {
        let mesh_shader = std::fs::read_to_string("assets/shader/mesh.wgsl").unwrap();
        let mesh_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Mesh"),
            source: wgpu::ShaderSource::Wgsl(mesh_shader.into()),
        });

        let bind_group_layout_camera =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout View Proj"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::Cube,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 5,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let bind_group_layout_light_arr =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout Light Arr"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let bind_group_layout_material =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout Material"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let render_pipline_layout_mesh =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipline Layout Mesh"),
                bind_group_layouts: &[
                    &bind_group_layout_camera,
                    &bind_group_layout_light_arr,
                    &bind_group_layout_material,
                ],
                push_constant_ranges: &[],
            });

        let render_pipline_mesh = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipline Mesh"),
            layout: Some(&render_pipline_layout_mesh),
            vertex: wgpu::VertexState {
                module: &mesh_shader,
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
                format: texture::DEPTH_FORMAT,
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
            fragment: Some(wgpu::FragmentState {
                module: &mesh_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let camera = Camera::new(surface_config.width as _, surface_config.height as _);
        let buffer_view_proj = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer View Proj"),
            contents: bytemuck::cast_slice(&camera.view_proj().to_cols_array_2d()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_pos: [f32; 3] = camera.pos.into();
        let buffer_camera_pos = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer Camera Pos"),
            contents: bytemuck::cast_slice(&camera_pos),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let buffer_shadow_map_view_proj_arr =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Buffer Shadow Map View Proj Array"),
                contents: bytemuck::cast_slice(
                    &(0..6)
                        .map(|_| Mat4::IDENTITY.to_cols_array_2d())
                        .collect::<Vec<[[f32; 4]; 4]>>(),
                ),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        let buffer_light_point_pos = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("buffer_light_point_pos"),
            contents: bytemuck::cast_slice(&[0.0, 0.0, 0.0]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let sampler_view_shadow_map = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sampler_view_shadow_map"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: None,
            ..Default::default()
        });

        let bind_group_camera = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group Camera"),
            layout: &bind_group_layout_camera,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_view_proj.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_camera_pos.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_shadow_map_view_proj_arr.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_light_point_pos.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&texture_view_shadow_map),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&sampler_view_shadow_map),
                },
            ],
        });

        let light_direction_arr = vec![];
        let light_direction_zero = vec![LightDirection::zero()];
        let light_point_arr = vec![];
        let light_point_zero = vec![LightPoint::zero()];
        let light_spot_arr = vec![];
        let light_spot_zero = vec![LightSpot::zero()];

        let buffer_light_direction = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer Light Directioin"),
            contents: if light_direction_arr.len() > 0 {
                bytemuck::cast_slice(&light_direction_arr)
            } else {
                bytemuck::cast_slice(&light_direction_zero)
            },
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let buffer_light_point = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer Light Point"),
            contents: if light_point_arr.len() > 1 {
                bytemuck::cast_slice(&light_point_arr)
            } else {
                bytemuck::cast_slice(&light_point_zero)
            },
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let buffer_light_spot = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer Light Spot"),
            contents: if light_spot_arr.len() > 1 {
                bytemuck::cast_slice(&light_spot_arr)
            } else {
                bytemuck::cast_slice(&light_spot_zero)
            },
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_light_arr = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group Light Array"),
            layout: &bind_group_layout_light_arr,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_light_direction.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_light_point.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_light_spot.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        let sampler = gen_sampler_clamp(&device);
        let sampler_repeat = gen_sampler_repeat(&device);

        let texture_view_depth = gen_texture_depth(
            &device,
            surface_config.width,
            surface_config.height,
            SAMPLE_COUNT,
        )
        .create_view(&wgpu::TextureViewDescriptor::default());

        let texture_view_msaa = gen_texture_view_msaa(&device, &surface_config, SAMPLE_COUNT);

        Self {
            render_pipline_mesh,
            camera,

            sampler,
            sampler_repeat,

            bind_group_layout_camera,
            bind_group_camera,
            bind_group_light_arr,
            bind_group_layout_material,
            material_arr: vec![],

            buffer_view_proj,
            buffer_camera_pos,
            texture_view_depth,
            texture_view_msaa,
            sampler_view_shadow_map,

            light_direction_arr,
            light_point_arr,
            light_spot_arr,
            buffer_light_direction,
            buffer_light_point,
            buffer_light_spot,
        }
    }

    pub fn resize(&mut self, device: &Device, surface_config: &SurfaceConfiguration) {
        self.camera
            .update_size(surface_config.width as _, surface_config.height as _);

        self.texture_view_depth = gen_texture_depth(
            device,
            surface_config.width,
            surface_config.height,
            SAMPLE_COUNT,
        )
        .create_view(&wgpu::TextureViewDescriptor::default());
        self.texture_view_msaa = gen_texture_view_msaa(device, surface_config, SAMPLE_COUNT);
    }

    pub fn render(&mut self, encoder: &mut CommandEncoder, texture_view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass Mesh"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.texture_view_msaa,
                resolve_target: Some(texture_view),
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.texture_view_depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&self.render_pipline_mesh);
        render_pass.set_bind_group(0, &self.bind_group_camera, &[]);
        render_pass.set_bind_group(1, &self.bind_group_light_arr, &[]);

        for material in &self.material_arr {
            render_pass.set_bind_group(2, &material.bind_group, &[]);

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

    pub fn update(&mut self, queue: &mut Queue, input: &Input, delta_time: f32) {
        self.camera.moving(input, delta_time);

        queue.write_buffer(
            &self.buffer_view_proj,
            0,
            bytemuck::cast_slice(&self.camera.view_proj().to_cols_array_2d()),
        );

        queue.write_buffer(
            &self.buffer_camera_pos,
            0,
            bytemuck::cast_slice(&self.camera.pos.to_array()),
        );

        if self.light_spot_arr.len() > 0 {
            self.light_spot_arr[0].pos = self.camera.pos.into();
            self.light_spot_arr[0].front = self.camera.front.into();
            queue.write_buffer(
                &self.buffer_light_spot,
                0,
                bytemuck::cast_slice(&self.light_spot_arr),
            );
        }
    }

    pub fn add_light_direction(&mut self, queue: &mut Queue, light: LightDirection) {
        self.light_direction_arr.push(light);
        queue.write_buffer(
            &self.buffer_light_direction,
            0,
            bytemuck::cast_slice(&self.light_direction_arr),
        );
    }

    pub fn add_light_point(&mut self, queue: &mut Queue, light: LightPoint) {
        self.light_point_arr.push(light);
        queue.write_buffer(
            &self.buffer_light_point,
            0,
            bytemuck::cast_slice(&self.light_point_arr),
        );
    }

    pub fn add_light_spot(&mut self, queue: &mut Queue, light: LightSpot) {
        self.light_spot_arr.push(light);
        queue.write_buffer(
            &self.buffer_light_spot,
            0,
            bytemuck::cast_slice(&self.light_spot_arr),
        );
    }

    pub fn set_shadow_map(
        &mut self,
        device: &Device,
        buffer_view_proj_arr_shadow_map: Buffer,
        buffer_light_point_pos: Buffer,
        texture_view_shadow_map: &TextureView,
    ) {
        self.bind_group_camera = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group Camera"),
            layout: &self.bind_group_layout_camera,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        self.buffer_view_proj.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        self.buffer_camera_pos.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_view_proj_arr_shadow_map.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(
                        buffer_light_point_pos.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&texture_view_shadow_map),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&self.sampler_view_shadow_map),
                },
            ],
        });
    }
}
