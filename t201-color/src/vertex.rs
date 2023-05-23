#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
}

impl Vertex {
    fn new(pos: [f32; 3]) -> Self {
        Self { pos }
    }

    const ATTRS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    pub fn cube() -> [Self; 36] {
        [
            // front
            Self::new([-0.5, -0.5, 0.0]),
            Self::new([0.5, -0.5, 0.0]),
            Self::new([-0.5, 0.5, 0.0]),
            Self::new([0.5, -0.5, 0.0]),
            Self::new([0.5, 0.5, 0.0]),
            Self::new([-0.5, 0.5, 0.0]),
            // right
            Self::new([0.5, -0.5, 0.0]),
            Self::new([0.5, -0.5, -1.0]),
            Self::new([0.5, 0.5, 0.0]),
            Self::new([0.5, -0.5, -1.0]),
            Self::new([0.5, 0.5, -1.0]),
            Self::new([0.5, 0.5, 0.0]),
            // back
            Self::new([0.5, -0.5, -1.0]),
            Self::new([-0.5, -0.5, -1.0]),
            Self::new([0.5, 0.5, -1.0]),
            Self::new([-0.5, -0.5, -1.0]),
            Self::new([-0.5, 0.5, -1.0]),
            Self::new([0.5, 0.5, -1.0]),
            // left
            Self::new([-0.5, -0.5, -1.0]),
            Self::new([-0.5, -0.5, 0.0]),
            Self::new([-0.5, 0.5, -1.0]),
            Self::new([-0.5, -0.5, 0.0]),
            Self::new([-0.5, 0.5, 0.0]),
            Self::new([-0.5, 0.5, -1.0]),
            // bottom
            Self::new([-0.5, -0.5, -1.0]),
            Self::new([0.5, -0.5, -1.0]),
            Self::new([-0.5, -0.5, 0.0]),
            Self::new([0.5, -0.5, -1.0]),
            Self::new([0.5, -0.5, 0.0]),
            Self::new([-0.5, -0.5, 0.0]),
            // top
            Self::new([-0.5, 0.5, 0.0]),
            Self::new([0.5, 0.5, 0.0]),
            Self::new([-0.5, 0.5, -1.0]),
            Self::new([0.5, 0.5, 0.0]),
            Self::new([0.5, 0.5, -1.0]),
            Self::new([-0.5, 0.5, -1.0]),
        ]
    }
}
