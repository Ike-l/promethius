use std::{
    fs::{
        read, read_to_string
    }, 
    io::{
        BufReader, Cursor
    }, 
    path::{
        Path, PathBuf
    }
};

use wgpu::util::DeviceExt;

use crate::prelude::{
    object_plugin::prelude::*, 
    texture::prelude::Texture,
};

#[derive(Debug)]
pub struct Resources {
    pub obj_path: String,
    pub mat_path: String,
}

impl Resources {
    pub fn new(obj_path: &str, mat_path: &str) -> Self {
        Self {
            obj_path: obj_path.to_string(),
            mat_path: mat_path.to_string(),
        }
    }

    fn path(&self, path: &str, file_name: &str) -> PathBuf {
        Path::new(path).join(file_name)
    }

    async fn load_string(file_path: &PathBuf) -> anyhow::Result<String> {
        let text = read_to_string(file_path)?;

        Ok(text)
    }

    async fn load_binary(file_path: &PathBuf) -> anyhow::Result<Vec<u8>> {
        let data = read(file_path)?;

        Ok(data)
    }

    async fn load_texture(
        file_path: &PathBuf,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> anyhow::Result<Texture> {
        let data = Self::load_binary(file_path).await?;
        Texture::from_bytes(device, queue, &data, file_path.to_str().unwrap())
    }

    pub fn load_from_colored_vertices(
        label: &str,
        vertices: Vec<ColoredVertex>,
        indices: Vec<u32>,
        device: &wgpu::Device,
    ) -> ColoredModel {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label}, vertex_buffer")),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label}, index_buffer")),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
    
        let mesh = ColoredMesh::new(
            label, 
            vertex_buffer, 
            index_buffer, 
            indices.len(),
            vertices
        );
    
        ColoredModel::new(vec![mesh])
    }

    pub async fn load_from_colored_model<C: Into<[f32; 4]> + Clone> (
        &self,
        file_name: &str,
        topology: wgpu::PrimitiveTopology,
        color: C,
        device: &wgpu::Device,
    ) -> anyhow::Result<ColoredModel> {
        let triangulate = match topology {
            crate::prelude::TriangleList => true,
            crate::prelude::LineList => false,
            _ => panic!("Topology not supported")
        };

        let obj_text = Self::load_string(&self.path(&self.obj_path, file_name)).await?;
        let obj_cursor = Cursor::new(obj_text);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, _) = tobj::load_obj_buf_async(
            &mut obj_reader, 
            &tobj::LoadOptions {
                triangulate,
                single_index: true,
                ..Default::default()
            },
            |material_file_name| async move {
                log::warn!("Material file found via load_from_colored_model. 
                Could cause performance bottleneck as is discarded after parsing");

                let material_text = Self::load_string(&self.path(&self.mat_path, &material_file_name)).await
                    .expect("Material file failed to load");
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(material_text)))
            }).await?;

        let meshes = models
            .into_iter()
            .map(|model| {
                let vertices = (0..model.mesh.positions.len() / 3)
                    .map(|i| ColoredVertex::new(
                        [
                            model.mesh.positions[i * 3],
                            model.mesh.positions[i * 3 + 1],
                            model.mesh.positions[i * 3 + 2],
                        ],
                        color.clone().into(),
                    )).collect::<Vec<_>>();
    
                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{file_name} vertex_buffer")),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{file_name} index_buffer")),
                    contents: bytemuck::cast_slice(&model.mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });
    
                ColoredMesh::new(
                    file_name,
                    vertex_buffer,
                    index_buffer,
                    model.mesh.indices.len(),
                    vertices
                )
            })
            .collect::<Vec<_>>();
    
        Ok (
            ColoredModel::new(meshes)
        )
    }

    pub async fn load_from_textured_model(
        &self,
        file_name: &str,
        topology: wgpu::PrimitiveTopology,
        layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> anyhow::Result<MaterialModel> {
        let triangulate = match topology {
            crate::prelude::TriangleList => true,
            crate::prelude::LineList => false,
            _ => panic!("Topology not supported")
        };

        let obj_text = Self::load_string(&self.path(&self.obj_path, file_name)).await?;
        let obj_cursor = Cursor::new(obj_text);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::load_obj_buf_async(
            &mut obj_reader, 
            &tobj::LoadOptions {
                triangulate,
                single_index: true,
                ..Default::default()
            },
            |material_file_name| async move {
                let material_text = Self::load_string(&self.path(&self.mat_path, &material_file_name)).await
                    .expect("Material file failed to load");
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(material_text)))
            }).await?;

        let mut materials = Vec::new();
        for material in obj_materials? {
            let diffuse_texture = Self::load_texture(
                &self.path(&self.mat_path, &material.diffuse_texture.unwrap()),
                device,
                queue
            ).await?;

            materials.push(Material::new(
                &material.name, 
                device, 
                diffuse_texture, 
                layout
            ));
        }

        let meshes = models
            .into_iter()
            .map(|model| {
                let material_index = model.mesh.material_id.unwrap_or(0);
                let vertices = (0..model.mesh.positions.len() / 3)
                    .map(|i| TexturedVertex::new(
                        [
                            model.mesh.positions[i * 3],
                            model.mesh.positions[i * 3 + 1],
                            model.mesh.positions[i * 3 + 2],
                        ],
                        [
                            model.mesh.texcoords[i * 2],
                            1.0 - model.mesh.texcoords[i * 2 + 1]
                        ]
                    )).collect::<Vec<_>>();

                    let vertex_buffer = device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(&format!("{file_name}, vertex_buffer")),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        },
                    );

                    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("{file_name}, index_buffer")),
                        contents: bytemuck::cast_slice(&model.mesh.indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });

                    MaterialMesh::new(
                        file_name,
                        vertex_buffer,
                        index_buffer,
                        model.mesh.indices.len(),
                        vertices,
                        material_index,
                    )
            }
        ).collect::<Vec<_>>();

        Ok(
            MaterialModel::new(
                meshes,
                materials,
            )
        )
    }
}