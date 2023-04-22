use wgpu::{
    util::DeviceExt, Backends, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, Features, Instance, PrimitiveState, PrimitiveTopology,
    RenderPassDescriptor, TextureUsages,
};
use winit::{
    event::{self, ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::vertex::Vertex;

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });

    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("create surface error")
    };

    let window_size = window.inner_size();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("request adapter error");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                features: Features::empty(),
                limits: todo!(),
            },
            None,
        )
        .await
        .expect("request device error");

    let surface_caps = surface.get_capabilities(&adapter);
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_caps.formats[0],
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &surface_config);

    let shader =
        std::fs::read_to_string("assets/shader/shader.wgsl").expect("read shader.wgsl error");
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(shader.into()),
    });

    let (vertices, indices) = Vertex::triangle();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: BufferUsages::INDEX,
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::buffer_layout()],
        },
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
            let current_texture = surface
                .get_current_texture()
                .expect("get current texture error");
            let texture_view = current_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(Color {
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

                render_pass.set_pipeline(&render_pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
            }
        }
        winit::event::Event::WindowEvent { window_id, event } if window_id == window.id() => {
            match event {
                WindowEvent::Resized(new_size) => {
                    surface_config.width = new_size.width;
                    surface_config.height = new_size.height;
                    surface.configure(&device, &surface_config);
                }
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => {
                    if virtual_keycode == Some(event::VirtualKeyCode::Escape)
                        && state == ElementState::Released
                    {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            }
        }
        winit::event::Event::MainEventsCleared => window.request_redraw(),

        _ => {}
    })
}