use std::time::Instant;

use glam::{Mat4, Quat, Vec3};
use wgpu::util::DeviceExt;
use winit::{event_loop::EventLoop, window::Window};

use crate::{
    texture::{
        gen_texture_sampler, gen_texture_view, gen_texture_view_depth, texture_bind_group_layout,
    },
    transform::Transform,
    vertex::Vertex,
};

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    let window_size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("create surface fail")
    };

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
        .await
        .expect("request device fail");

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

    let texture_bind_group_layout = texture_bind_group_layout(&device);

    let v_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("View Bind Group Layout"),
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
    let p_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Proj Bind Group Layout"),
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

    let render_pipline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipline Layout"),
        bind_group_layouts: &[
            &texture_bind_group_layout,
            &v_bind_group_layout,
            &p_bind_group_layout,
        ],
        push_constant_ranges: &[],
    });

    let shader = std::fs::read_to_string("assets/shader/shader.wgsl").unwrap();
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(shader.into()),
    });

    let render_pipline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipline"),
        layout: Some(&render_pipline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                Vertex::vertex_buffer_layout(),
                Transform::vertex_buffer_layout(),
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
            format: wgpu::TextureFormat::Depth32Float,
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
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_caps.formats[0],
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

    let texture_sampler = gen_texture_sampler(&device);
    let texture_view_container = gen_texture_view("assets/texture/container.jpg", &device, &queue);
    let texture_view_huaji = gen_texture_view("assets/texture/huaji.jpg", &device, &queue);

    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture Bind Group"),
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&texture_sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture_view_container),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&texture_view_huaji),
            },
        ],
    });

    let mut texture_view_depth = gen_texture_view_depth(&device, &surface_config);

    let mut transform_arr = vec![];
    let axis = Vec3::new(1.0, 0.3, 0.5);
    transform_arr.push(Transform::new(
        Vec3::new(0.0, 0.0, 0.0),
        Quat::from_axis_angle(axis, (20.0 * 0.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(2.0, 5.0, -15.0),
        Quat::from_axis_angle(axis, (20.0 * 1.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(-1.5, -2.2, -2.5),
        Quat::from_axis_angle(axis, (20.0 * 2.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(-3.8, -2.0, -12.3),
        Quat::from_axis_angle(axis, (20.0 * 3.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(2.4, -0.4, -3.5),
        Quat::from_axis_angle(axis, (20.0 * 4.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(-1.7, 3.0, -7.5),
        Quat::from_axis_angle(axis, (20.0 * 5.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(1.3, -2.0, -2.5),
        Quat::from_axis_angle(axis, (20.0 * 6.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(1.5, 2.0, -2.5),
        Quat::from_axis_angle(axis, (20.0 * 7.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(1.5, 0.2, -1.5),
        Quat::from_axis_angle(axis, (20.0 * 8.0_f32).to_radians()),
        Vec3::ONE,
    ));
    transform_arr.push(Transform::new(
        Vec3::new(-1.3, 1.0, -1.5),
        Quat::from_axis_angle(axis, (20.0 * 9.0_f32).to_radians()),
        Vec3::ONE,
    ));

    let transform_mat_arr = transform_arr
        .iter()
        .map(|t| t.to_mat4().to_cols_array_2d())
        .collect::<Vec<_>>();

    let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Transform Buffer"),
        contents: bytemuck::cast_slice(&transform_mat_arr),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let mut eye_pos = Vec3::new(0.0, 0.0, 3.0);

    let mut fov = 45.0_f32;
    let mut aspect_ratio = surface_config.width as f32 / surface_config.height as f32;

    let mut last_time = Instant::now();
    let mut delta_time = 0.0;
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
            delta_time = (Instant::now() - last_time).as_secs_f32();
            last_time = Instant::now();

            let current_texture = surface.get_current_texture().unwrap();
            let texture_view = current_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            let mut view = Mat4::look_to_rh(eye_pos, Vec3::NEG_Z, Vec3::Y);
            let view_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("View Buffer"),
                contents: bytemuck::cast_slice(&view.to_cols_array_2d()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("View Bind Group"),
                layout: &v_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(view_buffer.as_entire_buffer_binding()),
                }],
            });

            let proj = Mat4::perspective_rh(fov, aspect_ratio, 0.1, 100.0);

            let proj_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Proj Buffer"),
                contents: bytemuck::cast_slice(&proj.to_cols_array_2d()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let proj_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Proj Bind Group"),
                layout: &p_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(proj_buffer.as_entire_buffer_binding()),
                }],
            });

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
                        view: &texture_view_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                });

                render_pass.set_pipeline(&render_pipline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, transform_buffer.slice(..));
                render_pass.set_bind_group(0, &texture_bind_group, &[]);
                render_pass.set_bind_group(1, &view_bind_group, &[]);
                render_pass.set_bind_group(2, &proj_bind_group, &[]);
                render_pass.draw(0..vertices.len() as u32, 0..transform_mat_arr.len() as u32);
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
                    texture_view_depth = gen_texture_view_depth(&device, &surface_config);
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
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed {
                        match input.virtual_keycode {
                            Some(winit::event::VirtualKeyCode::W) => {
                                eye_pos.z -= delta_time;
                            }
                            Some(winit::event::VirtualKeyCode::S) => {
                                eye_pos.z += delta_time;
                            }
                            Some(winit::event::VirtualKeyCode::A) => {
                                eye_pos.x -= delta_time;
                            }
                            Some(winit::event::VirtualKeyCode::D) => {
                                eye_pos.x += delta_time;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        winit::event::Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });
}
