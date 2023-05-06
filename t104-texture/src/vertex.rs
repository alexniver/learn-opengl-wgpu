#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 3],
    tex_coord: [f32; 2],
}

impl Vertex {
    fn new(pos: [f32; 3], tex_coord: [f32; 2]) -> Self {
        Self { pos, tex_coord }
    }

    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0=> Float32x3, 1=> Float32x2];
    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    pub fn triangle() -> ([Self; 3], [u32; 3]) {
        (
            [
                Self::new([-0.5, -0.5, 0.0], [0.0, 0.0]),
                Self::new([0.5, -0.5, 0.0], [1.0, 0.0]),
                Self::new([0.0, 0.5, 0.0], [0.5, 1.0]),
            ],
            [0, 1, 2],
        )
    }

    pub fn square() -> ([Self; 4], [u32; 6]) {
        (
            [
                Self::new([-0.5, -0.5, 0.0], [0.0, 1.0]),
                Self::new([0.5, -0.5, 0.0], [1.0, 1.0]),
                Self::new([0.5, 0.5, 0.0], [1.0, 0.0]),
                Self::new([-0.5, 0.5, 0.0], [0.0, 0.0]),
            ],
            [0, 1, 3, 1, 2, 3],
        )
    }
}
