use glam::{Mat4, Quat, Vec3};

const MAT4_NUM: usize = 2;

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn to_raw(&self) -> TransformRaw {
        let mat =
            Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation);
        let mut combine: [[f32; 4]; MAT4_NUM * 4] = [[0.0; 4]; MAT4_NUM * 4];
        combine[..4].copy_from_slice(&mat.to_cols_array_2d());
        let it_mat = mat.inverse().transpose();
        combine[4..8].copy_from_slice(&it_mat.to_cols_array_2d());

        TransformRaw { model: combine }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformRaw {
    model: [[f32; 4]; MAT4_NUM * 4],
}

impl TransformRaw {
    const ATTRS: [wgpu::VertexAttribute; (MAT4_NUM * 4)] = wgpu::vertex_attr_array![5 => Float32x4, 6 => Float32x4, 7 => Float32x4, 8 => Float32x4, 9 => Float32x4, 10 => Float32x4, 11 => Float32x4, 12 => Float32x4];
    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: 4 * 4 * (MAT4_NUM as u64 * 4),
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }
}
