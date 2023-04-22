use wgpu::VertexBufferLayout;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
}

impl Vertex {
    fn new(pos: [f32; 3]) -> Self {
        Self { pos }
    }

    pub fn triangle() -> ([Self; 3], [u16; 3]) {
        (
            [
                Self::new([-0.5, -0.5, 0.0]),
                Self::new([0.5, -0.5, 0.0]),
                Self::new([0.0, 0.5, 0.0]),
            ],
            [0, 1, 2],
        )
    }

    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0=> Float32x3];
    pub fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
