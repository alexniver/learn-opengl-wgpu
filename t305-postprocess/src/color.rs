#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    color: [f32; 3],
}

impl Color {
    pub fn new(color: [f32; 3]) -> Self {
        Self { color }
    }

    const ATTR: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![15 => Float32x3];
    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: 4 * 3,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTR,
        }
    }
}

