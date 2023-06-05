#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightDirection {
    pub dir: [f32; 3],
    // 16 bytes padding
    _padding0: u32,

    pub color: [f32; 3],
    // 16 bytes padding
    _padding1: u32,

    pub ambient: [f32; 3],
    // 16 bytes padding
    _padding2: u32,

    pub diffuse: [f32; 3],
    // 16 bytes padding
    _padding3: u32,

    pub specular: [f32; 3],
    // 16 bytes padding
    _padding4: u32,
}

impl LightDirection {
    pub fn new(
        dir: [f32; 3],
        color: [f32; 3],
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
    ) -> Self {
        Self {
            dir,
            _padding0: 0,
            color,
            _padding1: 0,
            ambient,
            _padding2: 0,
            diffuse,
            _padding3: 0,
            specular,
            _padding4: 0,
        }
    }
}
