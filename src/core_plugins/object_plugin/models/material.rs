use crate::prelude::texture::prelude::Texture;

#[derive(Debug)]
pub struct Material {
    pub name: String,
    texture: Texture,
    bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        name: &str,
        device: &wgpu::Device,
        texture: Texture,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            label: Some(&format!("{name}, bind_group")),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture.view())
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(texture.sampler()),
                },
            ],
        });

        Self {
            name: name.to_string(),
            texture,
            bind_group
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}