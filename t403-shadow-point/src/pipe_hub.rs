use std::time::Instant;

use wgpu::{
    util::DeviceExt, Adapter, Backends, Device, DeviceDescriptor, Features, Instance, PresentMode,
    Queue, Surface, SurfaceConfiguration, TextureUsages,
};
use winit::{
    dpi::PhysicalPosition, event::MouseScrollDelta, event_loop::EventLoop, window::Window,
};

use crate::{
    input::Input, light_point::LightPoint, model_light::ModelLight, pipe_depth::PipeDepth,
    pipe_mesh::PipeMesh, pipe_shadow::PipeShadow, texture::DEPTH_FORMAT,
};

pub struct PipeHub {
    pub window: Window,
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface_config: SurfaceConfiguration,
    pub input: Input,

    pub pipe_shadow: PipeShadow,
    pub pipe_mesh: PipeMesh,
    pub pipe_depth: PipeDepth,

    pub model_light_arr: Vec<ModelLight>,

    pub start_time: Instant,
    pub last_time: Instant,
}

impl PipeHub {
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

        let input = Input::new();

        let pipe_shadow = PipeShadow::new(&device, &surface_config, 1024 * 2, 1024 * 2);
        let pipe_mesh = PipeMesh::new(
            &device,
            &surface_config,
            &pipe_shadow
                .texture_cube
                .create_view(&wgpu::TextureViewDescriptor {
                    label: Some("Texture View Shadow Map"),
                    dimension: Some(wgpu::TextureViewDimension::Cube),
                    ..Default::default()
                }),
        );
        let pipe_depth = PipeDepth::new(
            &device,
            &surface_config,
            &pipe_shadow
                .texture_depth
                .create_view(&wgpu::TextureViewDescriptor {
                    label: Some("Pipe Depth"),
                    format: Some(DEPTH_FORMAT),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    aspect: wgpu::TextureAspect::DepthOnly,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: 0,
                    array_layer_count: Some(1),
                }),
            false,
        );

        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            input,

            pipe_shadow,
            pipe_mesh,
            pipe_depth,

            model_light_arr: vec![],

            start_time: Instant::now(),
            last_time: Instant::now(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);

        self.pipe_mesh.resize(&self.device, &self.surface_config);

        self.pipe_depth.set_texture_view_depth(
            &self.device,
            &self
                .pipe_shadow
                .texture_depth
                .create_view(&wgpu::TextureViewDescriptor {
                    label: Some("Texture View Shadow Map Depth"),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    format: Some(DEPTH_FORMAT),
                    aspect: wgpu::TextureAspect::DepthOnly,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: 0,
                    array_layer_count: Some(1),
                }),
        );
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

        self.pipe_shadow.render(
            &mut encoder,
            &self.surface_config,
            &self.pipe_mesh.material_arr,
        );
        self.pipe_mesh.render(&mut encoder, &texture_view);
        self.pipe_depth.render(&mut encoder, &texture_view);

        self.queue.submit(std::iter::once(encoder.finish()));

        current_texture.present();
    }

    fn update(&mut self) {
        self.window
            .set_cursor_position(winit::dpi::LogicalPosition::new(
                self.surface_config.width / 2,
                self.surface_config.height / 2,
            ))
            .unwrap();

        // let total_time = (Instant::now() - self.start_time).as_secs_f32();
        let delta_time = (Instant::now() - self.last_time).as_secs_f32();
        self.last_time = Instant::now();

        self.pipe_mesh
            .update(&mut self.queue, &self.input, delta_time);
    }

    fn cursor_moved(&mut self, x: f32, y: f32) {
        self.pipe_mesh.camera.yaw_pitch(x, y);
    }

    fn mouse_wheel(&mut self, delta: MouseScrollDelta) {
        match delta {
            winit::event::MouseScrollDelta::LineDelta(_, y) => self.pipe_mesh.camera.fov(y),
            winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition { x: _x, y }) => {
                self.pipe_mesh.camera.fov(y as f32);
            }
        }
    }

    pub fn add_light_point(&mut self, light_point: LightPoint) {
        self.pipe_mesh.add_light_point(&mut self.queue, light_point);
        self.pipe_shadow
            .set_light_point(&self.device, &self.pipe_mesh.light_point_arr[0]);
        if let Some((_, view_mat4_arr)) = &self.pipe_shadow.bind_group_view_arr {
            let view_proj_mat4_arr = view_mat4_arr
                .iter()
                .map(|view| self.pipe_shadow.proj.mul_mat4(&view).to_cols_array_2d())
                .collect::<Vec<[[f32; 4]; 4]>>();
            let buffer_view_proj_mat4_arr =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Buffer View Proj Mat4 Array"),
                        contents: bytemuck::cast_slice(&view_proj_mat4_arr),
                        usage: wgpu::BufferUsages::STORAGE,
                    });
            let buffer_light_point_pos =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Buffer Light Point Pos"),
                        contents: bytemuck::cast_slice(&self.pipe_mesh.light_point_arr[0].pos),
                        usage: wgpu::BufferUsages::UNIFORM,
                    });
            self.pipe_mesh.set_shadow_map(
                &self.device,
                buffer_view_proj_mat4_arr,
                buffer_light_point_pos,
                &self
                    .pipe_shadow
                    .texture_cube
                    .create_view(&wgpu::TextureViewDescriptor {
                        label: Some("Texture View Shadow Map Depth"),
                        dimension: Some(wgpu::TextureViewDimension::Cube),
                        ..Default::default()
                    }),
            );
        }
    }

    pub fn block_loop(event_loop: EventLoop<()>, mut hub: PipeHub) {
        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::RedrawRequested(window_id) if window_id == hub.window.id() => {
                hub.update();
                hub.render();
            }
            winit::event::Event::WindowEvent { window_id, event }
                if window_id == hub.window.id() =>
            {
                match event {
                    winit::event::WindowEvent::Resized(new_size) => {
                        hub.resize(new_size.width, new_size.height);
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
                    winit::event::WindowEvent::MouseWheel { delta, .. } => {
                        hub.mouse_wheel(delta);
                    }
                    winit::event::WindowEvent::KeyboardInput {
                        input: keyboard_input,
                        ..
                    } => {
                        hub.input.on_input(keyboard_input);
                    }
                    _ => {}
                }
            }
            winit::event::Event::MainEventsCleared => hub.window.request_redraw(),
            winit::event::Event::DeviceEvent {
                device_id: _,
                event,
            } => match event {
                winit::event::DeviceEvent::MouseMotion { delta } => {
                    hub.cursor_moved(delta.0 as f32, delta.1 as f32);
                }
                _ => {}
            },
            _ => {}
        });
    }
}
