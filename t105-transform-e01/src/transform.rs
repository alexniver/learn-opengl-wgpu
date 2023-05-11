use glam::{Mat4, Quat, Vec3};

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    const ATTRS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![5 => Float32x4, 6 => Float32x4, 7 => Float32x4, 8 => Float32x4];
    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }

    pub fn mat4(&self) -> Mat4 {
        let tran_mat = Mat4::from_translation(self.translation);
        let rotation_mat = Mat4::from_quat(self.rotation);
        let scale_mat = Mat4::from_scale(self.scale);

        // scale_mat * tran_mat * rotation_mat
        scale_mat * rotation_mat * tran_mat
    }
}
