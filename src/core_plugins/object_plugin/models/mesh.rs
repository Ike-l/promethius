use crate::prelude::{
    acceleration_structures_plugin::prelude::AABB, 
    promethius_std::prelude::Position
};

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[derive(Debug, small_read_only::ReadOnly)]
pub struct Mesh {
    pub name: String,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    elements_count: u32,
}

impl Mesh {
    pub fn new(
        name: &str,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        elements_count: usize,
    ) -> Self {
        Self {
            name: format!("{name}, mesh"),
            vertex_buffer,
            index_buffer,
            elements_count: elements_count as u32
        }
    }
}

#[derive(Debug, small_read_only::ReadOnly)]
pub struct ColoredMesh {
    mesh: Mesh,
    raw_vertex_buffer: Vec<ColoredVertex>,
}

impl ColoredMesh {
    pub fn new(
        name: &str,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        elements_count: usize,
        raw_vertex_buffer: Vec<ColoredVertex>
    ) -> Self {
        Self {
            mesh: Mesh::new(
                name,
                vertex_buffer,
                index_buffer,
                elements_count
            ),
            raw_vertex_buffer,
        }
    }

    pub fn gen_aabb(&self) -> AABB {
        self.raw_vertex_buffer
            .iter()
            .fold(AABB::default(), |mut acc, cur| { acc.expand_pos(cur.into()); acc })
    }
}

#[derive(Debug, small_read_only::ReadOnly)]
pub struct MaterialMesh {
    mesh: Mesh,
    raw_vertex_buffer: Vec<TexturedVertex>,
    material_index: usize,
}

impl MaterialMesh {
    pub fn new(
        name: &str,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        elements_count: usize,
        raw_vertex_buffer: Vec<TexturedVertex>,
        material_index: usize,
    ) -> Self {
        Self {
            mesh: Mesh::new(
                name,
                vertex_buffer,
                index_buffer,
                elements_count
            ),
            raw_vertex_buffer,
            material_index,
        }
    }

    pub fn gen_aabb(&self) -> AABB {
        self.raw_vertex_buffer
            .iter()
            .fold(AABB::default(), |mut acc, cur| { acc.expand_pos(cur.into()); acc })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, small_read_only::ReadOnly)]
pub struct ColoredVertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Into<Position> for &ColoredVertex {
    fn into(self) -> Position {
        Position::new(self.position[0] as f64, self.position[1] as f64, self.position[2] as f64)
    }
}

impl Vertex for ColoredVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout { 
            array_stride: mem::size_of::<ColoredVertex>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ], 
        }
    }
}

impl ColoredVertex {
    pub fn new<T: Into<[f32; 3]>, Y: Into<[f32; 4]>>(position: T, color: Y) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, small_read_only::ReadOnly)]
pub struct TexturedVertex {
    position: [f32; 3],
    texture_coords: [f32; 2],
}

impl Into<Position> for &TexturedVertex {
    fn into(self) -> Position {
        Position::new(self.position[0] as f64, self.position[1] as f64, self.position[2] as f64)
    }
}

impl Vertex for TexturedVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout { 
            array_stride: mem::size_of::<TexturedVertex>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ], 
        }
    }
}

impl TexturedVertex {
    pub fn new(position: [f32; 3], texture_coords: [f32; 2]) -> Self {
        Self {
            position,
            texture_coords,
        }
    }
}