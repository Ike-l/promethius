use crate::prelude::object_plugin::prelude::Vertex;

use super::{
    FloatPrecision, VERTEX_FORMAT
};

#[repr(C)]
#[derive(Debug, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct RawRenderComponent {
    model: [[FloatPrecision; 4]; 4],
    tint: [FloatPrecision; 4],
    highlight: [FloatPrecision; 4],
}

impl Vertex for RawRenderComponent {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawRenderComponent>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[FloatPrecision; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[FloatPrecision; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[FloatPrecision; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[FloatPrecision; 16]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[FloatPrecision; 20]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: VERTEX_FORMAT,
                },
            ],
        }
    }
}

impl RawRenderComponent {
    pub fn new(model: [[FloatPrecision; 4]; 4], tint: [FloatPrecision; 4], highlight: [FloatPrecision; 4]) -> Self {
        Self {
            model,
            tint,
            highlight,
        }
    }
}