#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Default)]
pub struct OrthoUniform {
    pub view_projection: [[f32; 4]; 4],
}

impl OrthoUniform {
    pub fn cast_slice(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }
}