use std::ops::Range;

use crate::prelude::acceleration_structures::prelude::AABB;

use super::{
    mesh::{
        ColoredMesh, MaterialMesh
    },
    material::Material, 
};

#[derive(Debug)]
pub enum Model {
    Material(MaterialModel),
    Colored(ColoredModel),
}

impl Model {
    pub fn gen_aabb(&self) -> AABB {
        match &self {
            Model::Colored(m) => m.gen_aabb(),
            Model::Material(m) => m.gen_aabb(),
        }
    }
}

#[derive(Debug)]
pub struct MaterialModel {
    meshes: Vec<MaterialMesh>,
    materials: Vec<Material>,
}

impl MaterialModel {
    pub fn new(
        meshes: Vec<MaterialMesh>, 
        materials: Vec<Material>,
    ) -> Self {
        Self {
            meshes,
            materials,
        }
    }

    pub fn gen_aabb(&self) -> AABB {
        let mut aabb = AABB::default();
        for mesh in &self.meshes {
            aabb.expand(&mesh.gen_aabb());
        }

        aabb
    }
}

#[derive(Debug, small_read_only::ReadOnly)]
pub struct ColoredModel {
    meshes: Vec<ColoredMesh>,
}

impl ColoredModel {
    pub fn new(meshes: Vec<ColoredMesh>) -> Self {
        Self {
            meshes,
        }
    }

    fn gen_aabb(&self) -> AABB {
        let mut aabb = AABB::default();
        for mesh in &self.meshes {
            aabb.expand(&mesh.gen_aabb());
        }

        aabb
    }
}

pub trait DrawModel<'a> {
    fn draw_colored_mesh_instanced(
        &mut self,
        mesh: &'a ColoredMesh,
        instances: Range<u32>,
        bind_groups: Vec<&'a wgpu::BindGroup>,
    );

    fn draw_material_mesh_instanced(
        &mut self,
        mesh: &'a MaterialMesh,
        instances: Range<u32>,
        bind_groups: Vec<&'a wgpu::BindGroup>
    );

    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'a> for wgpu::RenderPass<'b> 
    where 'a: 'b 
{
    fn draw_colored_mesh_instanced(
            &mut self,
            mesh: &'a ColoredMesh,
            instances: Range<u32>,
            bind_groups: Vec<&'a wgpu::BindGroup>,
        ) {
        self.set_vertex_buffer(0, mesh.mesh().vertex_buffer().slice(..));
        self.set_index_buffer(mesh.mesh().index_buffer().slice(..), wgpu::IndexFormat::Uint32);

        for (index, bind_group) in bind_groups.iter().enumerate() {
            self.set_bind_group(index as u32, bind_group, &[]);
        }
        self.draw_indexed(0..*mesh.mesh().elements_count(), 0, instances);
    } 

    fn draw_material_mesh_instanced(
            &mut self,
            mesh: &'a MaterialMesh,
            instances: Range<u32>,
            bind_groups: Vec<&'a wgpu::BindGroup>,
        )  {
        self.set_vertex_buffer(0, mesh.mesh().vertex_buffer().slice(..));
        self.set_index_buffer(mesh.mesh().index_buffer().slice(..), wgpu::IndexFormat::Uint32);

        for (index, bind_group) in bind_groups.iter().enumerate() {
            self.set_bind_group(index as u32, bind_group, &[]);
        }
        
        self.draw_indexed(0..*mesh.mesh().elements_count(), 0, instances);
    } 

    fn draw_model_instanced(
            &mut self,
            model: &'a Model,
            instances: Range<u32>,
            camera_bind_group: &'a wgpu::BindGroup,
        ) {
            match model {
                Model::Material(model) => {
                    for mesh in &model.meshes {
                        let material = &model.materials[*mesh.material_index()];
                        let new_bind_groups = vec![&material.bind_group(), camera_bind_group];
                        self.draw_material_mesh_instanced(mesh, instances.clone(), new_bind_groups);
                    }
                },
                Model::Colored(model) => {
                    for mesh in &model.meshes {
                        self.draw_colored_mesh_instanced(mesh, instances.clone(), vec![camera_bind_group]);
                    }
                },
            }   
    }
}