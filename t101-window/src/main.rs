use wgpu::{
    Backends, Color, DeviceDescriptor, Features, InstanceDescriptor, RenderPassDescriptor,
    RequestAdapterOptionsBase,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("build window error");
    pollster::block_on(run(event_loop, window));
}

async fn run(even_loop: EventLoop<()>, window: Window) {
    let window_size = window.inner_size();
    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });

    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("create surface error")
    };

    let adapter = instance
        .request_adapter(&RequestAdapterOptionsBase {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .expect("create adapter error");

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("Device Descriptor"),
                features: Features::empty(),
                limits: Default::default(),
            },
            None,
        )
        .await
        .expect("create device error");

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

    even_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                // window_size = size;
                surface_config.width = size.width;
                surface_config.height = size.height;

                surface.configure(&device, &surface_config);
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            //redraw
            let current_texture = surface
                .get_current_texture()
                .expect("get current texture error");
            let view = current_texture.texture.create_view(&Default::default());

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let mut _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(Color {
                                r: 0.1,
                                g: 0.1,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
            }

            queue.submit(std::iter::once(encoder.finish()));
            current_texture.present();
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
