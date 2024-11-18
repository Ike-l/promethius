use wgpu::util::DeviceExt;

use super::{
    prelude::*, 
    TransformComposer
};

#[derive(Debug)]
pub enum CameraViewComponent {
    Ortho(OrthoView),
    Persp(PerspView),
}

#[derive(Debug)]
pub enum CameraProjectionComponent {
    Ortho(OrthoProjection),
    Persp(PerspProjection),
}

#[derive(Debug)]
pub enum CameraUniform {
    Ortho(OrthoUniform),
    Persp(PerspUniform),
}

impl CameraUniform {
    pub fn cast_slice(&self) -> Vec<u8> {
        match &self {
            CameraUniform::Ortho(ortho) => {
                ortho.cast_slice()
            },
            CameraUniform::Persp(persp) => {
                persp.cast_slice()
            }
        }
    }
}

#[derive(Debug)]
pub struct CameraRenderComponent {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pub view: CameraViewComponent,
    pub projection: CameraProjectionComponent,
}

impl CameraRenderComponent {
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn cast_slice(&self) -> Vec<u8> {
        match &self.view {
            CameraViewComponent::Ortho(view) => {
                match &self.projection {
                    CameraProjectionComponent::Ortho(projection) => {
                        OrthoUniform {
                            view_projection: (projection.compose_transform() * view.compose_transform()).into()
                        }.cast_slice()
                    },
                    CameraProjectionComponent::Persp(_) => {
                        panic!("Need an ortho projection with an ortho view");
                    },
                }
            },
            CameraViewComponent::Persp(view) => {
                match &self.projection {
                    CameraProjectionComponent::Persp(projection) => {
                        PerspUniform {
                            view_projection: (projection.compose_transform() * view.compose_transform()).into(),
                            view_position: view.position.to_homogeneous().into(),
                        }.cast_slice()
                    },
                    CameraProjectionComponent::Ortho(_) => {
                        panic!("Need a persp projection with a persp view");
                    },
                }
            },
        }
    }

    pub fn update_buffers(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, &self.cast_slice())
    }

    pub fn create_camera(
        view: CameraViewComponent,
        projection: CameraProjectionComponent,
        uniform: CameraUniform,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        label: &str
    ) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label}, camera_buffer")),
                contents: &uniform.cast_slice(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some(&format!("{label}, camera_bind_group")),
        });

        Self {
            view,
            projection,
            bind_group,
            buffer,
        }
    }
    
    pub fn ortho(
        view: OrthoView, 
        projection: OrthoProjection, 
        device: &wgpu::Device, 
        layout: &wgpu::BindGroupLayout, 
        label: &str
    ) -> Self {
        Self::create_camera(
            CameraViewComponent::Ortho(view), 
            CameraProjectionComponent::Ortho(projection),
            CameraUniform::Ortho(OrthoUniform::default()),
            device, 
            layout, 
            label
        )
    }

    pub fn persp(
        view: PerspView, 
        projection: PerspProjection, 
        device: &wgpu::Device, 
        layout: &wgpu::BindGroupLayout, 
        label: &str
    ) -> Self {
        Self::create_camera(
            CameraViewComponent::Persp(view), 
            CameraProjectionComponent::Persp(projection),
            CameraUniform::Persp(PerspUniform::default()),
            device, 
            layout, 
            label
        )
    }
}

