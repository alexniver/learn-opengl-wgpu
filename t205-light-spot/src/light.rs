#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub pos: [f32; 3],
    // 16 bytes padding
    _padding0: u32,

    pub front: [f32; 3],
    // 16 bytes padding
    _padding1: u32,

    pub color: [f32; 3],
    _padding2: u32,

    pub ambient: [f32; 3],
    // 16 bytes padding
    _padding3: u32,

    pub diffuse: [f32; 3],
    // 16 bytes padding
    _padding4: u32,

    pub specular: [f32; 3],
    pub cutoff: f32,
}

impl Light {
    pub fn new(
        pos: [f32; 3],
        front: [f32; 3],
        color: [f32; 3],
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        cutoff: f32,
    ) -> Self {
        Self {
            pos,
            _padding0: 0,
            front,
            _padding1: 0,
            color,
            _padding2: 0,
            ambient,
            _padding3: 0,
            diffuse,
            _padding4: 0,
            specular,
            cutoff,
        }
    }

    pub fn change_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
}
