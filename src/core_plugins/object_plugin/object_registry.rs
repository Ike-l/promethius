use std::collections::{
    hash_map::Entry, 
    HashMap
};

use wgpu::util::DeviceExt;

use crate::prelude::{
    entity::prelude::{
        RawRenderComponent, InstanceRenderComponent
    },
    render_plugin::prelude::{
        State, PipelineType
    },
    RefWorld, Res, ResMut
};

use super::{
    acceleration_structures_plugin::prelude::AABB, 
    label_plugin::prelude::LabelComponent, 
    models::model::Model
};

#[derive(Debug)]
pub struct Object {
    instance_count: std::ops::Range<u32>,
    instance_buffer: wgpu::Buffer,
    aabb: AABB,
    pub label: LabelComponent,
    pub camera_label: LabelComponent,
    pub pipeline: PipelineType,
    pub model: Model,
}

impl Object {
    pub fn new(label: LabelComponent, camera_label: LabelComponent, pipeline: PipelineType, model: Model, device: &wgpu::Device) -> Self {
        let instance_count = 0..0;
        let instance_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some(&format!("{:?}, instance_buffer", label)),
                size: 0,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }
        );

        let aabb = model.gen_aabb();

        Self {
            instance_count,
            instance_buffer,
            aabb,
            label,
            camera_label,
            pipeline,
            model,
        }
    }

    pub fn instance_count(&self) -> &std::ops::Range<u32> {
        &self.instance_count
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

#[derive(Debug)]
pub struct ObjectRegistry {
    pub objects: HashMap<LabelComponent, Object>,
}

impl Default for ObjectRegistry {
    fn default() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
}

impl ObjectRegistry {
    pub const LINE: wgpu::PrimitiveTopology = wgpu::PrimitiveTopology::LineList;
    pub const TRIANGLE: wgpu::PrimitiveTopology = wgpu::PrimitiveTopology::TriangleList;

    fn clear(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let ids = self.objects.iter().map(|object| object.0.clone()).collect::<Vec<LabelComponent>>();

        for id in ids {
            self.update_object_buffer(device, queue, &id, vec![]);
        }
    }

    fn update_object_buffer(
        &mut self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue,
        object_label: &LabelComponent, 
        data: Vec<RawRenderComponent>
    ) {
        let object = self.objects.get_mut(object_label).expect(&format!("No object found: {:?}", object_label));

        object.instance_count = 0..data.len() as u32;

        let data = bytemuck::cast_slice(&data);

        object.instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?}, instance_buffer", object.label)),
                contents: data,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        queue.write_buffer(&object.instance_buffer, 0, data);
    }
    
    pub fn insert(&mut self, object: Object) -> Option<Object> {
        match self.objects.entry(object.label.clone()) {
            Entry::Occupied(_) => Some(object),
            Entry::Vacant(entry) => {
                entry.insert(object);
                None
            },
        }
    }
}

pub fn update_registry_instances(
    world: RefWorld,
    mut object_registry: ResMut<ObjectRegistry>,
    state: Res<Vec<State>>
) {
    let state = state.first().unwrap();

    object_registry.clear(&state.device(), &state.queue());

    let mut objects: HashMap<&LabelComponent, Vec<RawRenderComponent>> = HashMap::new();

    let query = &mut world.query::<(&InstanceRenderComponent, &LabelComponent)>();

    for (_, (render, label)) in query {
        if !render.visible {
            continue   
        }
        
        match objects.entry(&label) {
            Entry::Occupied(mut entry) => { 
                entry.get_mut().push(render.to_raw()); 
            },
            Entry::Vacant(entry) => {
                entry.insert(vec![render.to_raw()]);
            },
        }
    }

    for (id, data) in objects {
        object_registry.update_object_buffer(&state.device(), &state.queue(), id, data);
    }
}