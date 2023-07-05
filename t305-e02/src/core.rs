use std::time::Instant;

use wgpu::{
    util::DeviceExt, Adapter, Backends, BindGroup, BindGroupLayout, Buffer, ComputePipeline,
    Device, DeviceDescriptor, Features, IndexFormat, Instance, PresentMode, Queue, RenderPipeline,
    Sampler, ShaderStages, Surface, SurfaceConfiguration, TextureUsages, TextureView,
};
use winit::{dpi::PhysicalPosition, event_loop::EventLoop, window::Window};

use crate::{
    camera::Camera,
    input::Input,
    light_direction::LightDirection,
    light_point::LightPoint,
    light_spot::LightSpot,
    material::Material,
    model::DrawMethod,
    model_light::ModelLight,
    texture::{
        self, gen_texture_depth, gen_texture_sampler, gen_texture_view_post_processing_in,
        gen_texture_view_post_processing_out,
    },
    transform::TransformRawIT,
    vertex::Vertex,
};

pub struct Core {
    pub window: Window,
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface_config: SurfaceConfiguration,
    pub render_pipline_mesh: RenderPipeline,
    pub render_pipline_light: RenderPipeline,
    pub camera: Camera,
    pub input: Input,

    pub render_pipline_post_processing: RenderPipeline,
    pub compute_pipline_post_processing: ComputePipeline,
    pub bind_group_layout_post_processing: BindGroupLayout,
    pub bind_group_layout_post_processing_compute: BindGroupLayout,
    pub bind_group_post_processing: BindGroup,
    pub bind_group_post_processing_compute: BindGroup,
    pub texture_view_post_processing_compute_in: TextureView,
    pub texture_view_post_processing_compute_out: TextureView,
    pub vertex_buffer_post_processing: Buffer,
    pub index_buffer_post_processing: Buffer,
    pub index_post_processing_len: u32,

    pub texture_sampler: Sampler,

    pub camera_bind_group: BindGroup,
    pub light_arr_bind_group: BindGroup,

    pub material_bind_group_layout: BindGroupLayout,
    pub material_arr: Vec<Material>,

    pub light_direction_arr: Vec<LightDirection>,
    pub light_point_arr: Vec<LightPoint>,
    pub light_spot_arr: Vec<LightSpot>,

    pub view_buffer: Buffer,
    pub proj_buffer: Buffer,
    pub camera_pos_buffer: Buffer,

    pub texture_depth: TextureView,

    pub light_direction_buffer: Buffer,
    pub light_point_buffer: Buffer,
    pub light_spot_buffer: Buffer,

    pub model_light_arr: Vec<ModelLight>,

    pub start_time: Instant,
    pub last_time: Instant,
}

impl Core {
    pub async fn new(window: Window) -> Self {
        let window_size = window.inner_size();

        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Device"),
                    features: Features::empty(),
                    limits: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        let mesh_shader = std::fs::read_to_string("assets/shader/mesh.wgsl").unwrap();
        let mesh_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader 1"),
            source: wgpu::ShaderSource::Wgsl(mesh_shader.into()),
        });

        let light_shader = std::fs::read_to_string("assets/shader/light.wgsl").unwrap();
        let light_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader 2"),
            source: wgpu::ShaderSource::Wgsl(light_shader.into()),
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("View Proj Bind Group Layout"),
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
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
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
                ],
            });

        let light_arr_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Light Bind Group Layout"),
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

        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Material Bind Group Layout"),
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

        let bind_group_layout_post_processing =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Post Processing Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });

        let bind_group_layout_post_processing_compute =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout Post Processing Compute"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::COMPUTE,
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
                label: Some("Render Pipline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &light_arr_bind_group_layout,
                    &material_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipline_layout_light =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipline_layout_post_processing =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipline Layout Post Processing"),
                bind_group_layouts: &[&bind_group_layout_post_processing],
                push_constant_ranges: &[],
            });
        let compute_pipline_layout_post_processing =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Pipline Layout Post Processing"),
                bind_group_layouts: &[&bind_group_layout_post_processing_compute],
                push_constant_ranges: &[],
            });

        let render_pipline_mesh = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipline"),
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
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &mesh_shader,
                entry_point: "fs_main",
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ],
            }),
            multiview: None,
        });

        let render_pipline_light = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipline 2"),
            layout: Some(&render_pipline_layout_light),
            vertex: wgpu::VertexState {
                module: &light_shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::vertex_buffer_layout(),
                    TransformRawIT::vertex_buffer_layout(),
                    crate::color::Color::vertex_buffer_layout(),
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
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &light_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let shader_post_processing =
            std::fs::read_to_string("assets/shader/post_processing.wgsl").unwrap();
        let shader_post_processing = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Post Processing"),
            source: wgpu::ShaderSource::Wgsl(shader_post_processing.into()),
        });
        let render_pipline_post_processing =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipline Post Processing"),
                layout: Some(&render_pipline_layout_post_processing),
                vertex: wgpu::VertexState {
                    module: &shader_post_processing,
                    entry_point: "vs_main",
                    buffers: &[Vertex::vertex_buffer_layout()],
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
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_post_processing,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });

        let compute_shader =
            std::fs::read_to_string("assets/shader/post_processing_compute.wgsl").unwrap();
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Pipline Post Processing"),
            source: wgpu::ShaderSource::Wgsl(compute_shader.into()),
        });

        let compute_pipline_post_processing =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Compute Pipline Post Processing"),
                layout: Some(&compute_pipline_layout_post_processing),
                module: &compute_shader,
                entry_point: "cp_main",
            });
        let camera = Camera::new(surface_config.width as _, surface_config.height as _);
        let view_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("View Buffer"),
            contents: bytemuck::cast_slice(&camera.view().to_cols_array_2d()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let proj_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Proj Buffer"),
            contents: bytemuck::cast_slice(&camera.proj().to_cols_array_2d()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_pos: [f32; 3] = camera.pos.into();
        let camera_pos_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Pos Buffer"),
            contents: bytemuck::cast_slice(&camera_pos),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("View Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(view_buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(proj_buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        camera_pos_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        let light_direction_arr = vec![LightDirection::new(
            [0.0, 0.0, -1.0],
            [1.0, 1.0, 1.0, 0.0],
            [0.01, 0.01, 0.01],
            [0.5, 0.5, 0.5],
            [1.0, 1.0, 1.0],
        )];

        let light_point_arr = vec![LightPoint::new(
            [-2.5, 0.0, -1.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.1, 0.1, 0.1],
            [0.5, 0.5, 0.5],
            [1.0, 1.0, 1.0],
            1.0,
            0.09,
            0.032,
        )];

        let light_spot_arr = vec![LightSpot::new(
            camera.pos.into(),
            camera.front.into(),
            [1.0, 1.0, 0.0, 1.0],
            [0.1, 0.1, 0.1],
            [0.5, 0.5, 0.5],
            [1.0, 1.0, 1.0],
            12.5_f32.to_radians().cos(),
            17.5_f32.to_radians().cos(),
        )];

        let light_direction_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Directioin Buffer"),
            contents: bytemuck::cast_slice(&light_direction_arr),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let light_point_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Point Buffer"),
            contents: bytemuck::cast_slice(&light_point_arr),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let light_spot_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Spot Buffer"),
            contents: bytemuck::cast_slice(&light_spot_arr),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let light_arr_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Light Color Bind Group"),
            layout: &light_arr_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        light_direction_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        light_point_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        light_spot_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        let texture_sampler = gen_texture_sampler(&device);
        let input = Input::new();

        let texture_depth = gen_texture_depth(&device, &surface_config);

        let (vertices_post_processing, indices_post_processing) = Vertex::rect_screen();
        let vertex_buffer_post_processing =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer Post Processing"),
                contents: bytemuck::cast_slice(&vertices_post_processing),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer_post_processing =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer Post Processing"),
                contents: bytemuck::cast_slice(&indices_post_processing),
                usage: wgpu::BufferUsages::INDEX,
            });

        let texture_view_post_processing_compute_in =
            gen_texture_view_post_processing_in(&device, &surface_config);
        let texture_view_post_processing_compute_out =
            gen_texture_view_post_processing_out(&device, &surface_config);
        let bind_group_post_processing = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group Post Processing"),
            layout: &bind_group_layout_post_processing,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &texture_view_post_processing_compute_out,
                    ),
                },
            ],
        });

        let texture_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Size Buffer"),
            contents: bytemuck::cast_slice(&[surface_config.width, surface_config.height]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bind_group_post_processing_compute =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Bind Group Post Processing Compute"),
                layout: &bind_group_layout_post_processing_compute,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &texture_view_post_processing_compute_in,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &texture_view_post_processing_compute_out,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Buffer(
                            texture_size_buffer.as_entire_buffer_binding(),
                        ),
                    },
                ],
            });

        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            render_pipline_mesh,
            render_pipline_light,
            camera,
            input,
            texture_sampler,

            render_pipline_post_processing,
            compute_pipline_post_processing,
            bind_group_layout_post_processing,
            bind_group_layout_post_processing_compute,
            bind_group_post_processing,
            bind_group_post_processing_compute,
            texture_view_post_processing_compute_in,
            texture_view_post_processing_compute_out,
            vertex_buffer_post_processing,
            index_buffer_post_processing,
            index_post_processing_len: indices_post_processing.len() as u32,

            camera_bind_group,
            light_arr_bind_group,
            material_bind_group_layout,
            material_arr: vec![],

            model_light_arr: vec![],

            light_direction_arr,
            light_point_arr,
            light_spot_arr,

            view_buffer,
            proj_buffer,
            camera_pos_buffer,

            texture_depth,

            light_direction_buffer,
            light_point_buffer,
            light_spot_buffer,

            start_time: Instant::now(),
            last_time: Instant::now(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
        self.camera.update_size(
            self.surface_config.width as _,
            self.surface_config.height as _,
        );

        self.texture_depth = gen_texture_depth(&self.device, &self.surface_config);

        self.texture_view_post_processing_compute_in =
            gen_texture_view_post_processing_in(&self.device, &self.surface_config);
        self.texture_view_post_processing_compute_out =
            gen_texture_view_post_processing_out(&self.device, &self.surface_config);

        self.bind_group_post_processing =
            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Bind Group Post Processing"),
                layout: &self.bind_group_layout_post_processing,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&self.texture_sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &self.texture_view_post_processing_compute_out,
                        ),
                    },
                ],
            });

        let texture_size_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Texture Size Buffer"),
                    contents: bytemuck::cast_slice(&[
                        self.surface_config.width,
                        self.surface_config.height,
                    ]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });
        self.bind_group_post_processing_compute =
            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Bind Group Post Processing Compute"),
                layout: &self.bind_group_layout_post_processing_compute,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &self.texture_view_post_processing_compute_in,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &self.texture_view_post_processing_compute_out,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Buffer(
                            texture_size_buffer.as_entire_buffer_binding(),
                        ),
                    },
                ],
            });
    }

    fn render(&mut self) {
        let current_texture = self
            .surface
            .get_current_texture()
            .expect("get current texture fail");
        let texture_view = current_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &self.texture_view_post_processing_compute_in,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.texture_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipline_mesh);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.light_arr_bind_group, &[]);

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
                        render_pass
                            .set_index_buffer(model.index_buffer.slice(..), IndexFormat::Uint32);
                        render_pass.draw_indexed(0..model.indices_len, 0, 0..model.instance_num);
                    }
                }
            }
        }

        // compute pipline
        // compute kernel effect
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass Post Processing"),
            });
            compute_pass.set_pipeline(&self.compute_pipline_post_processing);
            compute_pass.set_bind_group(0, &self.bind_group_post_processing_compute, &[]);
            compute_pass.dispatch_workgroups(
                (self.surface_config.width as f32 / 8.0).ceil() as u32,
                (self.surface_config.height as f32 / 8.0).ceil() as u32,
                1,
            );
        }

        // draw final texture
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass Post Processing"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
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
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipline_post_processing);
            render_pass.set_vertex_buffer(0, self.vertex_buffer_post_processing.slice(..));
            render_pass.set_index_buffer(
                self.index_buffer_post_processing.slice(..),
                IndexFormat::Uint32,
            );

            render_pass.set_bind_group(0, &self.bind_group_post_processing, &[]);
            render_pass.draw_indexed(0..self.index_post_processing_len, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        current_texture.present();
    }

    fn update(&mut self) {
        // let total_time = (Instant::now() - self.start_time).as_secs_f32();
        let delta_time = (Instant::now() - self.last_time).as_secs_f32();
        self.last_time = Instant::now();

        self.camera.moving(&self.input, delta_time);

        self.queue.write_buffer(
            &self.view_buffer,
            0,
            bytemuck::cast_slice(&self.camera.view().to_cols_array_2d()),
        );
        self.queue.write_buffer(
            &self.proj_buffer,
            0,
            bytemuck::cast_slice(&self.camera.proj().to_cols_array_2d()),
        );

        self.queue.write_buffer(
            &self.camera_pos_buffer,
            0,
            bytemuck::cast_slice(&self.camera.pos.to_array()),
        );

        // let new_color = [
        //     (total_time * 2.0).sin(),
        //     (total_time * 0.7).sin(),
        //     (total_time * 1.3).sin(),
        // ];
        // light.change_color(new_color);
        // queue.write_buffer(&light_buffer, 0, bytemuck::bytes_of(&light));

        self.light_spot_arr[0].pos = self.camera.pos.into();
        self.light_spot_arr[0].front = self.camera.front.into();
        self.queue.write_buffer(
            &self.light_spot_buffer,
            0,
            bytemuck::cast_slice(&self.light_spot_arr),
        );
    }

    pub fn add_model_light(&mut self, model_light: ModelLight) {
        self.model_light_arr.push(model_light);
    }

    pub fn block_loop(event_loop: EventLoop<()>, mut core: Core) {
        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::RedrawRequested(window_id) if window_id == core.window.id() => {
                core.update();
                core.render();
            }
            winit::event::Event::WindowEvent { window_id, event }
                if window_id == core.window.id() =>
            {
                match event {
                    winit::event::WindowEvent::Resized(new_size) => {
                        core.resize(new_size.width, new_size.height);
                    }
                    winit::event::WindowEvent::CloseRequested
                    | winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                state: winit::event::ElementState::Released,
                                ..
                            },
                        ..
                    } => *control_flow = winit::event_loop::ControlFlow::Exit,
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        core.camera.yaw_pitch(position.x as f32, position.y as f32);
                    }
                    winit::event::WindowEvent::MouseWheel { delta, .. } => match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => core.camera.fov(y),
                        winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition {
                            x: _x,
                            y,
                        }) => {
                            core.camera.fov(y as f32);
                        }
                    },
                    winit::event::WindowEvent::KeyboardInput {
                        input: keyboard_input,
                        ..
                    } => {
                        core.input.on_input(keyboard_input);
                    }
                    _ => {}
                }
            }
            winit::event::Event::MainEventsCleared => core.window.request_redraw(),
            _ => {}
        });
    }
}
