#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightSpot {
    pub pos: [f32; 3],
    // 16 bytes padding
    _padding0: u32,

    pub front: [f32; 3],
    // 16 bytes padding
    _padding1: u32,

    pub color: [f32; 4],

    pub ambient: [f32; 3],
    // 16 bytes padding
    _padding3: u32,

    pub diffuse: [f32; 3],
    pub in_cutoff: f32,

    pub specular: [f32; 3],
    pub out_cutoff: f32,
}

impl LightSpot {
    pub fn new(
        pos: [f32; 3],
        front: [f32; 3],
        color: [f32; 4],
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        in_cutoff: f32,
        out_cutoff: f32,
    ) -> Self {
        Self {
            pos,
            _padding0: 0,
            front,
            _padding1: 0,
            color,
            ambient,
            _padding3: 0,
            diffuse,
            in_cutoff,
            specular,
            out_cutoff,
        }
    }

    pub fn zero() -> Self {
        Self::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            0.0,
            0.0,
        )
    }
}
