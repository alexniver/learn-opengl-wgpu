pub struct Material {
    pub sampler: wgpu::Sampler,
    pub diffuse: wgpu::TextureView,
    pub specular: wgpu::TextureView,
    pub shininess: f32,
}

impl Material {
    pub fn new(
        sampler: wgpu::Sampler,
        diffuse: wgpu::TextureView,
        specular: wgpu::TextureView,
        shininess: f32,
    ) -> Self {
        Self {
            sampler,
            diffuse,
            specular,
            shininess,
        }
    }
}
