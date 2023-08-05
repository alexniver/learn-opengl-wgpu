use glam::{Mat4, Vec3};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, CommandEncoder, Device, IndexFormat,
    RenderPipeline, Texture,
};

use crate::{
    light_point::LightPoint,
    material::Material,
    model::DrawMethod,
    texture::{gen_texture_cube, gen_texture_depth, DEPTH_FORMAT},
    transform::TransformRawIT,
    vertex::Vertex,
};

const SAMPLE_COUNT: u32 = 1;

pub struct PipeShadow {
    pub render_pipeline: RenderPipeline,
    pub texture_cube: Texture,
    pub texture_depth: Texture,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group_view_arr: Option<([BindGroup; 6], [Mat4; 6])>,
    pub buffer_near_far: wgpu::Buffer,
    pub proj: Mat4,
    pub width: u32,
    pub height: u32,
}

impl PipeShadow {
    pub fn new(
        device: &Device,
        surface_config: &wgpu::SurfaceConfiguration,
        width: u32,
        height: u32,
    ) -> Self {
        let texture_cube = gen_texture_cube(device, &surface_config, width, height);
        let texture_depth = gen_texture_depth(device, width, height, 1);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout Shadow"),
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
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        // let proj_size = 20.0;
        // let proj = Mat4::orthographic_rh(-proj_size, proj_size, -proj_size, proj_size, 0.1, 70.0);
        let near_far = [0.1, 100.0];
        let proj = Mat4::perspective_rh(
            90.0_f32.to_radians(),
            (width / height) as f32,
            near_far[0],
            near_far[1],
        );
        let bind_group_view_arr: Option<([BindGroup; 6], [Mat4; 6])> = None;
        let buffer_near_far = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer Near Far"),
            contents: bytemuck::cast_slice(&near_far),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        Self {
            render_pipeline,
            texture_cube,
            texture_depth,
            bind_group_layout,
            bind_group_view_arr,
            proj,
            width,
            height,
            buffer_near_far,
        }
    }

    pub fn render(
        &mut self,
        encoder: &mut CommandEncoder,
        surface_config: &wgpu::SurfaceConfiguration,
        material_arr: &Vec<Material>,
    ) {
        if let Some((bind_group_arr, _)) = &self.bind_group_view_arr {
            for (idx, bind_group) in bind_group_arr.iter().enumerate() {
                let texture_view = self.texture_cube.create_view(&wgpu::TextureViewDescriptor {
                    label: Some("Texture View Shadow Map"),
                    format: Some(surface_config.format),
                    dimension: Some(wgpu::TextureViewDimension::D2),
                    aspect: wgpu::TextureAspect::All,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: idx as u32,
                    array_layer_count: Some(1),
                });
                let texture_view_depth =
                    &self
                        .texture_depth
                        .create_view(&wgpu::TextureViewDescriptor {
                            label: Some("Texture View Shadow Map Depth"),
                            format: Some(DEPTH_FORMAT),
                            dimension: Some(wgpu::TextureViewDimension::D2),
                            aspect: wgpu::TextureAspect::DepthOnly,
                            base_mip_level: 0,
                            mip_level_count: None,
                            base_array_layer: 0,
                            array_layer_count: None,
                        });
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass Shadow"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: f64::MAX,
                                g: f64::MAX,
                                b: f64::MAX,
                                a: f64::MAX,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: texture_view_depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &bind_group, &[]);

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
                            render_pass.set_index_buffer(
                                model.index_buffer.slice(..),
                                IndexFormat::Uint32,
                            );
                            render_pass.draw_indexed(
                                0..model.indices_len,
                                0,
                                0..model.instance_num,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        self.texture_depth = gen_texture_depth(device, width, height, SAMPLE_COUNT);
    }

    pub fn set_light_point(&mut self, device: &Device, light_point: &LightPoint) {
        // cube map order
        let view_mat4_arr: [Mat4; 6] = [
            // Right
            Mat4::look_to_rh(light_point.pos.into(), Vec3::new(1.0, 0.0, 0.0), Vec3::Y),
            // Left
            Mat4::look_to_rh(light_point.pos.into(), Vec3::new(-1.0, 0.0, 0.0), Vec3::Y),
            // Up
            Mat4::look_to_rh(light_point.pos.into(), Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
            // Bottom
            Mat4::look_to_rh(
                light_point.pos.into(),
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::NEG_Z,
            ),
            // Back
            Mat4::look_to_rh(light_point.pos.into(), Vec3::new(0.0, 0.0, -1.0), Vec3::Y),
            // Front
            Mat4::look_to_rh(light_point.pos.into(), Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
        ];
        let buffer_light_point_pos = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer Light Point Pos"),
            contents: bytemuck::cast_slice(&light_point.pos),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group_arr = view_mat4_arr
            .iter()
            .map(|view| {
                let buffer_view_proj =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Shadow Map View Proj Buffer"),
                        contents: bytemuck::cast_slice(
                            &self.proj.mul_mat4(view).to_cols_array_2d(),
                        ),
                        usage: wgpu::BufferUsages::UNIFORM,
                    });

                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Bind Group Shadow Map"),
                    layout: &self.bind_group_layout,
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
                                buffer_light_point_pos.as_entire_buffer_binding(),
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::Buffer(
                                self.buffer_near_far.as_entire_buffer_binding(),
                            ),
                        },
                    ],
                })
            })
            .collect::<Vec<BindGroup>>()
            .try_into()
            .unwrap();

        self.bind_group_view_arr = Some((bind_group_arr, view_mat4_arr));
    }
}
