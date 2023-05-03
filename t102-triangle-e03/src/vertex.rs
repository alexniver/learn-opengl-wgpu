#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn new(pos: [f32; 3], color: [f32; 3]) -> Self {
        Self { pos, color }
    }

    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    pub fn triangle1() -> ([Self; 3], [u32; 3]) {
        (
            [
                Self::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0]),
                Self::new([0.0, -0.5, 0.0], [0.0, 1.0, 0.0]),
                Self::new([-0.25, 0.5, 0.0], [0.0, 0.0, 1.0]),
            ],
            [0, 1, 2],
        )
    }

    pub fn triangle2() -> ([Self; 3], [u32; 3]) {
        (
            [
                Self::new([0.0, -0.5, 0.0], [1.0, 0.0, 0.0]),
                Self::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
                Self::new([0.25, 0.5, 0.0], [0.0, 0.0, 1.0]),
            ],
            [0, 1, 2],
        )
    }
}
