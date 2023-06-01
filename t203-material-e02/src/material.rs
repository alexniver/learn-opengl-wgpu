#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    color: [f32; 3],
    // 16 bytes padding
    _padding0: u32,

    ambient: [f32; 3],
    // 16 bytes padding
    _padding1: u32,

    diffuse: [f32; 3],
    // 16 bytes padding
    _padding2: u32,

    specular: [f32; 3],
    shininess: f32,
}

impl Material {
    pub fn new(
        color: [f32; 3],
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        shininess: f32,
    ) -> Self {
        Self {
            color,
            _padding0: 0,
            ambient,
            _padding1: 0,
            diffuse,
            _padding2: 0,
            specular,
            shininess,
        }
    }
}
