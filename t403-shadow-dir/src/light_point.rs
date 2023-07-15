#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct LightPoint {
    pub pos: [f32; 3],
    // 16 bytes padding
    _padding0: u32,

    pub color: [f32; 4],

    pub ambient: [f32; 3],
    // 16 bytes padding
    pub constant: f32,

    pub diffuse: [f32; 3],
    // 16 bytes padding
    pub linear: f32,

    pub specular: [f32; 3],
    // 16 bytes padding
    pub quadratic: f32,
}

impl LightPoint {
    pub fn new(
        pos: [f32; 3],
        color: [f32; 4],
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        constant: f32,
        linear: f32,
        quadratic: f32,
    ) -> Self {
        Self {
            pos,
            _padding0: 0,
            color,
            ambient,
            diffuse,
            specular,
            constant,
            linear,
            quadratic,
        }
    }

    pub fn zero() -> Self {
        Self::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            0.0,
            0.0,
            0.0,
        )
    }
}
