#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TimeUniform {
    pub total_time: f32,
}

impl TimeUniform {
    pub fn new(total_time: f32) -> Self {
        Self { total_time }
    }
}
