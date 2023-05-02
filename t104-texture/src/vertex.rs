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

    pub fn triangle() -> ([Self; 3], [u16; 3]) {
        (
            [
                Self::new([-0.5, -0.5, 0.0], [0.0, 0.0]),
                Self::new([0.5, -0.5, 0.0], [1.0, 0.0]),
                Self::new([0.0, 0.5, 0.0], [0.5, 1.0]),
            ],
            [0, 1, 2],
        )
    }
}
