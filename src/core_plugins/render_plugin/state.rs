use crate::prelude::{
    object_plugin::prelude::{
        DrawModel, ObjectRegistry
    },
    camera_plugin::prelude::{
        PerspProjection, CameraRenderComponent
    },
    Res, ResMut, RefWorld
};

use super::{
    prelude::{
        PipelineType, RenderConfig, WindowDimensions
    },
    label_plugin::prelude::LabelComponent, 
};

use std::{
    collections::HashMap, 
    sync::Arc
};

use winit::window::Window;


pub struct State {
    pub label: String,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_config: RenderConfig,

    size: winit::dpi::PhysicalSize<u32>,
    
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Window, label: &str, window_dimensions: &mut WindowDimensions) -> Self {
        let clear_color = wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        };

        let window = Arc::new(window);
        let surface_window = Arc::clone(&window);

        let size = window.inner_size();

        window_dimensions.width = size.width;
        window_dimensions.height = size.height;

        let label = format!("Window: {label}");

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(surface_window).unwrap();
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some(&format!("{label}, DeviceDescriptor")),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration  {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let render_config = RenderConfig::new(&device, &config, &label, clear_color);

        Self {
            label,

            surface,
            device,
            queue,
            config,
            render_config,

            size,
            window
        }
    }

    pub fn create_texture_layout(&self, label: &str) -> wgpu::BindGroupLayout {
        RenderConfig::create_texture_layout(&self.device, label)
    }

    pub fn create_camera_layout(&self, label: &str) -> wgpu::BindGroupLayout {
        RenderConfig::create_camera_layout(&self.device, label)
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }
    
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn set_clear_color(&mut self, color: [f64; 4]) {
        self.render_config.clear_color = wgpu::Color {
            r: color[0], g: color[1], b: color[2], a: color[3]
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, persp_projections: &mut Vec<&mut PerspProjection>, window_dimensions: &mut WindowDimensions) {
        if new_size.width > 0 && new_size.height > 0 {
            persp_projections.iter_mut().for_each(|p| p.resize(new_size.width, new_size.height));

            window_dimensions.width = new_size.width;
            window_dimensions.height = new_size.height;

            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.render_config.create_depth_texture(&self.device, &self.config, &self.label);
        }
    }
}

pub fn create_states(e: Res<&winit::event_loop::ActiveEventLoop>, mut states: ResMut<Vec<State>>, window_dimensions: ResMut<WindowDimensions>) {
    let event_loop = *e;
    let window_dimensions = window_dimensions.value;

    let label = "Main";
    let attributes = Window::default_attributes()
        .with_title(label);
    let window = event_loop.create_window(attributes).unwrap();

    let state = pollster::block_on(
        State::new(window, label, window_dimensions)
    );

    states.push(state);
}

pub fn render_system(object_registry: ResMut<ObjectRegistry>, state: Res<Vec<State>>, world: RefWorld) {
    let state = state.first().unwrap();

    let output = state.surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some(&format!("{}, render_encoder", state.label))
    });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(&format!("{}, render_pass", state.label)),
            color_attachments: &[
                Some(
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(state.render_config.clear_color),
                            store: wgpu::StoreOp::Store,
                        }
                    }
                )
            ],
            depth_stencil_attachment: Some(
                wgpu::RenderPassDepthStencilAttachment {
                    view: state.render_config.depth_texture().view(),
                    depth_ops: Some(
                        wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store
                        }
                    ),
                    stencil_ops: None,
                }
            ),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let mut query = world.query::<(&LabelComponent, &CameraRenderComponent)>();

        let mut camera: HashMap<&LabelComponent, &wgpu::BindGroup> = HashMap::new();
        for (_, (camera_id, camera_render_component)) in &mut query {
            camera.insert(camera_id, camera_render_component.bind_group());
        }


        let mut object_list = PipelineType::to_hashmap(Vec::new());
        for object in object_registry.objects.values() {            
            object_list.get_mut(&object.pipeline).unwrap().push(object);
        }

        for objects in object_list.values_mut() {
            objects.sort_by(|a, b| b.min_a().partial_cmp(a.min_a()).unwrap());
        }

        for (pipeline_type, object_list) in &object_list {
            let pipeline = state.render_config.pipelines().get(pipeline_type).expect(&format!("No pipeline found: {:?}", pipeline_type));
            render_pass.set_pipeline(pipeline);
            println!("New pipeline type");
            for object in object_list {
                println!("Rendering object: {:?} with A: {:?}", object.label, object.min_a());
                let instance_buffer = object.instance_buffer();
                if instance_buffer.size() == 0 {
                    continue;
                }
                let camera_bind_group = camera.get(&object.camera_label).expect(&format!("No CameraId found: {:?}", object.camera_label));

                render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                render_pass.draw_model_instanced(&object.model, object.instance_count().clone(), *camera_bind_group);
            }
        }
    }

    state.queue.submit(std::iter::once(encoder.finish()));
    output.present();

}
