use std::collections::HashMap;

use small_iter_fields::{
    IterFields, HashFields
};

use crate::prelude::{
    object_plugin::prelude::{
        ColoredVertex, TexturedVertex, Vertex
    },
    entity::prelude::RawRenderComponent,
    texture::prelude::Texture,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum BindGroupType {
    Texture,
    Camera,
}

#[derive(Debug, Hash, PartialEq, Eq, IterFields, HashFields)]
pub enum PipelineType {
    TwoDColoredLineList,
    TwoDColoredTriangleList,
    TwoDTexturedTriangleList,

    ThreeDColoredLineList,
    ThreeDColoredTriangleList,
    ThreeDTexturedTriangleList,
}

#[derive(Debug)]
pub struct RenderConfig {
    pipelines: HashMap<PipelineType, wgpu::RenderPipeline>,
    depth_texture: Texture,
    pub clear_color: wgpu::Color,
}

impl RenderConfig {
    pub fn new(
        device: &wgpu::Device, 
        config: &wgpu::SurfaceConfiguration, 
        label: &str, 
        clear_color: wgpu::Color
    ) -> Self {
        let depth_texture = Texture::create_depth_texture(device, config, &format!("{label}, depth_texture"));
        
        let camera_bind_group_layout = Self::create_camera_layout(device, label);
        let texture_bind_group_layout = Self::create_texture_layout(device, label);

        let mut pipelines = HashMap::with_capacity(4);
        for model_type_config in PipelineTypeConfig::get_configs(label) {
            let mut bind_group_layouts = Vec::with_capacity(2);
            for bind_group in model_type_config.bind_group_layouts {
                match bind_group {
                    BindGroupType::Texture => bind_group_layouts.push(&texture_bind_group_layout),
                    BindGroupType::Camera => bind_group_layouts.push(&camera_bind_group_layout),
                }
            }

            let render_pipeline_layout = device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some(&model_type_config.pipeline_layout_label),
                    bind_group_layouts: &bind_group_layouts,
                    push_constant_ranges: &[],
                }
            );

            let render_pipeline = {
                let shader = wgpu::ShaderModuleDescriptor {
                    label: Some(&model_type_config.shader_label),
                    source: wgpu::ShaderSource::Wgsl(model_type_config.shader_path.into()),
                };

                Self::create_render_pipeline(
                    &device, 
                    &render_pipeline_layout, 
                    config.format,
                    Some(Texture::DEPTH_FORMAT),
                    &model_type_config.vertex_layouts,
                    model_type_config.topology,
                    shader,
                    &model_type_config.pipeline_label
                )
            };

            pipelines.insert(model_type_config.pipeline_type, render_pipeline);
        } 
        
        Self {
            pipelines,
            depth_texture,            
            clear_color,
        }
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        vertex_layouts: &[wgpu::VertexBufferLayout],
        topology: wgpu::PrimitiveTopology, 
        shader: wgpu::ShaderModuleDescriptor,
        label: &str,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(shader);
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: vertex_layouts,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: color_format,
                    blend: Some(wgpu::BlendState {
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add
                        },
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                //cull_mode: Some(wgpu::Face::Back),
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }

    pub fn pipelines(&self) -> &HashMap<PipelineType, wgpu::RenderPipeline> {
        &self.pipelines
    }

    pub fn depth_texture(&self) -> &Texture {
        &self.depth_texture
    }

    pub fn create_depth_texture(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, label: &str) {
        self.depth_texture = Texture::create_depth_texture(device, config, &format!("{label}, depth_texture"));
    }

    pub fn create_texture_layout(device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some(&format!("{label}, texture_bind_group_layout"))
            }
        )
    }
    pub fn create_camera_layout(device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some(&format!("{label}, camera_bind_group_layout")),
            }
        )
    }
}

struct PipelineTypeConfig {
    pipeline_type: PipelineType,
    topology: wgpu::PrimitiveTopology,
    shader_label: String,
    shader_path: &'static str,
    pipeline_label: String,
    pipeline_layout_label: String,
    bind_group_layouts: Vec<BindGroupType>,
    vertex_layouts: [wgpu::VertexBufferLayout<'static>; 2],
}

impl PipelineTypeConfig {
    fn new(
        pipeline_type: PipelineType,
        topology: wgpu::PrimitiveTopology,
        shader_label: String,
        shader_path: &'static str,
        pipeline_label: String,
        pipeline_layout_label: String,
        bind_group_layouts: Vec<BindGroupType>,
        vertex_layouts: [wgpu::VertexBufferLayout<'static>; 2],
    ) -> Self {
        Self {
            pipeline_type,
            topology,
            shader_label,
            shader_path,
            pipeline_label,
            pipeline_layout_label,
            bind_group_layouts,
            vertex_layouts,
        }
    }
    fn get_configs(label: &str) -> Vec<PipelineTypeConfig> {
        let mut pipeline_type_configs = Vec::with_capacity(4);
        pipeline_type_configs.push(
            PipelineTypeConfig::new(
                PipelineType::TwoDTexturedTriangleList,
                wgpu::PrimitiveTopology::TriangleList,
                format!("{label}, textured_triangle_list_shader"),
                include_str!("../../../res/shaders/two_d/textured_triangle_list_shader.wgsl"),
                format!("{label}, textured_triangle_list_pipeline"),
                format!("{label}, textured_triangle_list_pipeline_layout"),
                vec![BindGroupType::Texture, BindGroupType::Camera],
                [TexturedVertex::desc(), RawRenderComponent::desc()],
            )
        );
        pipeline_type_configs.push(
            PipelineTypeConfig::new(
                PipelineType::TwoDColoredTriangleList,
                wgpu::PrimitiveTopology::TriangleList,
                format!("{label}, colored_triangle_list_shader"),
                include_str!("../../../res/shaders/two_d/colored_triangle_list_shader.wgsl"),
                format!("{label}, colored_triangle_list_pipeline"),
                format!("{label}, colored_triangle_list_pipeline_layout"),
                vec![BindGroupType::Camera],
                [ColoredVertex::desc(), RawRenderComponent::desc()],
            )
        );

        pipeline_type_configs.push(
            PipelineTypeConfig::new(
                PipelineType::TwoDColoredLineList,
                wgpu::PrimitiveTopology::LineList,
                format!("{label}, colored_line_list_shader"),
                include_str!("../../../res/shaders/two_d/colored_line_list_shader.wgsl"),
                format!("{label}, colored_line_list_pipeline"),
                format!("{label}, colored_line_list_pipeline_layout"),
                vec![BindGroupType::Camera],
                [ColoredVertex::desc(), RawRenderComponent::desc()],
            )
        );

        pipeline_type_configs.push(
            PipelineTypeConfig::new(
                PipelineType::ThreeDColoredTriangleList,
                wgpu::PrimitiveTopology::TriangleList,
                format!("{label}, colored_triangle_list_shader"),
                include_str!("../../../res/shaders/three_d/colored_triangle_list_shader.wgsl"),
                format!("{label}, colored_triangle_list_pipeline"),
                format!("{label}, colored_triangle_list_pipeline_layout"),
                vec![BindGroupType::Camera],
                [ColoredVertex::desc(), RawRenderComponent::desc()],
            )
        );

        pipeline_type_configs.push(
            PipelineTypeConfig::new(
                PipelineType::ThreeDTexturedTriangleList,
                wgpu::PrimitiveTopology::TriangleList,
                format!("{label}, textured_triangle_list_shader"),
                include_str!("../../../res/shaders/three_d/textured_triangle_list_shader.wgsl"),
                format!("{label}, textured_triangle_list_pipeline"),
                format!("{label}, textured_triangle_list_pipeline_layout"),
                vec![BindGroupType::Texture, BindGroupType::Camera],
                [TexturedVertex::desc(), RawRenderComponent::desc()],
            )
        );

        pipeline_type_configs.push(
            PipelineTypeConfig::new(
                PipelineType::ThreeDColoredLineList,
                wgpu::PrimitiveTopology::LineList,
                format!("{label}, colored_line_list_shader"),
                include_str!("../../../res/shaders/three_d/colored_line_list_shader.wgsl"),
                format!("{label}, colored_line_list_pipeline"),
                format!("{label}, colored_line_list_pipeline_layout"),
                vec![BindGroupType::Camera],
                [ColoredVertex::desc(), RawRenderComponent::desc()],
            )
        );

        pipeline_type_configs
    }
}