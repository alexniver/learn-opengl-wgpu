use std::time::Instant;

use wgpu::{
    util::DeviceExt, Adapter, Backends, BindGroup, Buffer, Device, DeviceDescriptor, Features,
    Instance, PresentMode, Queue, RenderPipeline, Sampler, Surface, SurfaceConfiguration,
    TextureUsages, TextureView,
};
use winit::{dpi::PhysicalPosition, event_loop::EventLoop, window::Window};

use crate::{
    camera::Camera,
    input::Input,
    texture::{self, gen_texture_depth, gen_texture_sampler},
    transform::{Transform, TransformRawIT},
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
    pub render_pipline_red: RenderPipeline,
    pub render_pipline_green: RenderPipeline,
    pub render_pipline_blue: RenderPipeline,
    pub render_pipline_yellow: RenderPipeline,
    pub camera: Camera,
    pub input: Input,

    pub texture_sampler: Sampler,

    pub camera_bind_group: BindGroup,

    pub view_buffer: Buffer,
    pub proj_buffer: Buffer,
    pub camera_pos_buffer: Buffer,

    pub vertex_buffer_cube: Buffer,
    pub vertex_len: u32,

    pub transform_buffer_red: Buffer,
    pub transform_buffer_green: Buffer,
    pub transform_buffer_blue: Buffer,
    pub transform_buffer_yellow: Buffer,

    pub texture_depth: TextureView,

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

        let render_pipline_layout_mesh =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipline_red = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                entry_point: "fs_main_red",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let render_pipline_green = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                entry_point: "fs_main_green",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let render_pipline_blue = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                entry_point: "fs_main_blue",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        let render_pipline_yellow =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                    entry_point: "fs_main_yellow",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
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

        let texture_sampler = gen_texture_sampler(&device);
        let input = Input::new();

        let texture_depth = gen_texture_depth(&device, &surface_config);

        let vertices_cube = Vertex::cube();
        let vertex_buffer_cube = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices_cube),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let transform_red = Transform::new(
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Quat::IDENTITY,
            glam::Vec3::ONE,
        );

        let transform_green = Transform::new(
            glam::Vec3::new(2.0, 0.0, 0.0),
            glam::Quat::IDENTITY,
            glam::Vec3::ONE,
        );

        let transform_blue = Transform::new(
            glam::Vec3::new(2.0, 2.0, 0.0),
            glam::Quat::IDENTITY,
            glam::Vec3::ONE,
        );

        let transform_yellow = Transform::new(
            glam::Vec3::new(0.0, 2.0, 0.0),
            glam::Quat::IDENTITY,
            glam::Vec3::ONE,
        );

        let transform_buffer_red = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer Red"),
            contents: bytemuck::cast_slice(&[transform_red.to_raw_it()]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let transform_buffer_green = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer Green"),
            contents: bytemuck::cast_slice(&[transform_green.to_raw_it()]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let transform_buffer_blue = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer Blue"),
            contents: bytemuck::cast_slice(&[transform_blue.to_raw_it()]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let transform_buffer_yellow =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Transform Buffer Yellow"),
                contents: bytemuck::cast_slice(&[transform_yellow.to_raw_it()]),
                usage: wgpu::BufferUsages::VERTEX,
            });
        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            render_pipline_red,
            render_pipline_green,
            render_pipline_blue,
            render_pipline_yellow,
            camera,
            input,
            texture_sampler,

            camera_bind_group,

            view_buffer,
            proj_buffer,
            camera_pos_buffer,

            vertex_buffer_cube,
            vertex_len: vertices_cube.len() as u32,
            transform_buffer_red,
            transform_buffer_green,
            transform_buffer_blue,
            transform_buffer_yellow,

            texture_depth,

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
    }

    fn render(&self) {
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.texture_depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_vertex_buffer(0, self.vertex_buffer_cube.slice(..));
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            render_pass.set_pipeline(&self.render_pipline_red);
            render_pass.set_vertex_buffer(1, self.transform_buffer_red.slice(..));
            render_pass.draw(0..self.vertex_len, 0..1);

            render_pass.set_pipeline(&self.render_pipline_green);
            render_pass.set_vertex_buffer(1, self.transform_buffer_green.slice(..));
            render_pass.draw(0..self.vertex_len, 0..1);

            render_pass.set_pipeline(&self.render_pipline_blue);
            render_pass.set_vertex_buffer(1, self.transform_buffer_blue.slice(..));
            render_pass.draw(0..self.vertex_len, 0..1);

            render_pass.set_pipeline(&self.render_pipline_yellow);
            render_pass.set_vertex_buffer(1, self.transform_buffer_yellow.slice(..));
            render_pass.draw(0..self.vertex_len, 0..1);
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
