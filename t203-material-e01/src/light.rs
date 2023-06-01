#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pos: [f32; 3],
    // 16 bytes padding
    _padding0: u32,

    pub color: [f32; 3],
    _padding1: u32,

    ambient: [f32; 3],
    // 16 bytes padding
    _padding2: u32,

    diffuse: [f32; 3],
    // 16 bytes padding
    _padding3: u32,

    specular: [f32; 3],
    // 16 bytes padding
    _padding4: u32,
}

impl Light {
    pub fn new(
        pos: [f32; 3],
        color: [f32; 3],
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
    ) -> Self {
        Self {
            pos,
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

    pub fn change_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
}
