#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug, Default)]
pub struct PerspUniform {
    pub view_projection: [[f32; 4]; 4],
    pub view_position: [f32; 4],
}

impl PerspUniform {
    pub fn cast_slice(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }
}