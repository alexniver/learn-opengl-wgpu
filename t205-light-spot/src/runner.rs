use std::time::Instant;

use anyhow::Result;
use glam::{Quat, Vec3};
use log::debug;
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalPosition, event_loop::EventLoop, window::Window};

use crate::{
    camera::Camera,
    input::Input,
    light::Light,
    material::Material,
    texture::{self, gen_texture_depth, gen_texture_sampler, gen_texture_view},
    transform::{Transform, TransformRaw},
    vertex::Vertex,
};

pub async fn run(event_loop: EventLoop<()>, window: Window) -> Result<()> {
    let window_size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(&window)? };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("request adapter fail");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await?;

    let surface_caps = surface.get_capabilities(&adapter);
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_caps.formats[0],
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &surface_config);

    let shader_1 = std::fs::read_to_string("assets/shader/shader_1.wgsl")?;
    let shader_1 = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader 1"),
        source: wgpu::ShaderSource::Wgsl(shader_1.into()),
    });

    let view_proj_bind_group_layout =
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
            ],
        });

    let light_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Light Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let camera_pos_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
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
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
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
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

    let render_pipline_layout_1 = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipline Layout"),
        bind_group_layouts: &[
            &view_proj_bind_group_layout,
            &light_bind_group_layout,
            &camera_pos_bind_group_layout,
            &material_bind_group_layout,
        ],
        push_constant_ranges: &[],
    });

    let render_pipline_1 = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipline"),
        layout: Some(&render_pipline_layout_1),
        vertex: wgpu::VertexState {
            module: &shader_1,
            entry_point: "vs_main",
            buffers: &[
                Vertex::vertex_buffer_layout(),
                TransformRaw::vertex_buffer_layout(),
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
            module: &shader_1,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });

    let vertices = Vertex::cube();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let mut texture_depth = gen_texture_depth(&device, &surface_config);

    let mut camera = Camera::new(surface_config.width as _, surface_config.height as _);
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

    let view_proj_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("View Bind Group"),
        layout: &view_proj_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(view_buffer.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(proj_buffer.as_entire_buffer_binding()),
            },
        ],
    });

    let light_pos = [1.2, 1.0, 2.0];
    let mut light = Light::new(
        light_pos,
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.01, 0.01, 0.01],
        [0.5, 0.5, 0.5],
        [1.0, 1.0, 1.0],
        12.5_f32.to_radians().cos(),
        17.5_f32.to_radians().cos(),
    );

    let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Light Color Buffer"),
        contents: bytemuck::bytes_of(&light),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Light Color Bind Group"),
        layout: &light_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(light_buffer.as_entire_buffer_binding()),
        }],
    });

    let camera_pos_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Pos Buffer"),
        contents: bytemuck::cast_slice(&camera.pos.to_array()),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_pos_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Camera Bind Group"),
        layout: &camera_pos_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(camera_pos_buffer.as_entire_buffer_binding()),
        }],
    });

    let texture_sampler = gen_texture_sampler(&device);
    let texture_diffuse_view = gen_texture_view("assets/texture/container2.png", &device, &queue)?;
    let texture_specular_view =
        gen_texture_view("assets/texture/container2_specular.png", &device, &queue)?;
    let material = Material::new(
        texture_sampler,
        texture_diffuse_view,
        texture_specular_view,
        32.0,
    );
    let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Color Buffer"),
        contents: bytemuck::cast_slice(&[1.0, 0.5, 0.31]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let shininess_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Material Buffer"),
        contents: bytemuck::bytes_of(&material.shininess),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Material Bind Group"),
        layout: &material_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&material.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&material.diffuse),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&material.specular),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Buffer(color_buffer.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Buffer(
                    shininess_buffer.as_entire_buffer_binding(),
                ),
            },
        ],
    });

    let transform_arr = transforms();
    let transform_mat_arr = transform_arr.iter().map(|t| t.to_raw()).collect::<Vec<_>>();
    println!("size: {:?}", transform_mat_arr.len());

    let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Transform Buffer"),
        contents: bytemuck::cast_slice(&transform_mat_arr),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let start_time = Instant::now();
    let mut last_time = Instant::now();
    let mut delta_time = 0.0;

    let mut input = Input::new();

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
            let total_time = (Instant::now() - start_time).as_secs_f32();
            delta_time = (Instant::now() - last_time).as_secs_f32();
            last_time = Instant::now();

            camera.moving(&input, delta_time);

            let current_texture = surface
                .get_current_texture()
                .expect("get current texture fail");
            let texture_view = current_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            queue.write_buffer(
                &view_buffer,
                0,
                bytemuck::cast_slice(&camera.view().to_cols_array_2d()),
            );
            queue.write_buffer(
                &proj_buffer,
                0,
                bytemuck::cast_slice(&camera.proj().to_cols_array_2d()),
            );

            queue.write_buffer(
                &camera_pos_buffer,
                0,
                bytemuck::cast_slice(&camera.pos.to_array()),
            );

            light.pos = camera.pos.into();
            light.front = camera.front.into();
            queue.write_buffer(&light_buffer, 0, bytemuck::bytes_of(&light));

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
                        view: &texture_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                });

                render_pass.set_pipeline(&render_pipline_1);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, transform_buffer.slice(..));
                render_pass.set_bind_group(0, &view_proj_bind_group, &[]);
                render_pass.set_bind_group(1, &light_bind_group, &[]);
                render_pass.set_bind_group(2, &camera_pos_bind_group, &[]);
                render_pass.set_bind_group(3, &material_bind_group, &[]);
                render_pass.draw(0..vertices.len() as _, 0..transform_arr.len() as _);
            }
            queue.submit(std::iter::once(encoder.finish()));

            current_texture.present();
        }
        winit::event::Event::WindowEvent { window_id, event } if window_id == window.id() => {
            match event {
                winit::event::WindowEvent::Resized(new_size) => {
                    surface_config.width = new_size.width;
                    surface_config.height = new_size.height;
                    surface.configure(&device, &surface_config);
                    texture_depth = gen_texture_depth(&device, &surface_config);
                    camera.update_size(surface_config.width as _, surface_config.height as _);
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
                    camera.yaw_pitch(position.x as f32, position.y as f32);
                }
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => camera.fov(y),
                        winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition {
                            x: _x,
                            y,
                        }) => {
                            camera.fov(y as f32);
                        }
                    }
                    debug!("mouse scroll {:?}", delta);
                }
                winit::event::WindowEvent::KeyboardInput {
                    input: keyboard_input,
                    ..
                } => {
                    input.on_input(keyboard_input);
                }
                _ => {}
            }
        }
        winit::event::Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });
}

fn transforms() -> Vec<Transform> {
    let mut transform_arr = vec![];
    let axis = Vec3::new(1.0, 0.3, 0.5).normalize();
    let pos_arr = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 5.0, -15.0),
        Vec3::new(-1.5, -2.2, -2.5),
        Vec3::new(-3.8, -2.0, -12.3),
        Vec3::new(2.4, -0.4, -3.5),
        Vec3::new(-1.7, 3.0, -7.5),
        Vec3::new(1.3, -2.0, -2.5),
        Vec3::new(1.5, 2.0, -2.5),
        Vec3::new(1.5, 0.2, -1.5),
        Vec3::new(-1.3, 1.0, -1.5),
    ];
    for (i, pos) in pos_arr.into_iter().enumerate() {
        transform_arr.push(Transform::new(
            pos,
            Quat::from_axis_angle(axis, (20.0 * i as f32).to_radians()),
            Vec3::ONE,
        ));
    }
    transform_arr
}
