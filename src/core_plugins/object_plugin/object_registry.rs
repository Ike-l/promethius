use std::collections::{
    hash_map::Entry, HashMap
};

use small_derive_deref::{
    Deref, DerefMut
};

use wgpu::util::DeviceExt;

use crate::prelude::{
    entity::prelude::{
        RawRenderComponent, InstanceRenderComponent
    },
    camera_plugin::prelude::CameraId,
    render_plugin::prelude::{
        State, PipelineType
    },
    RefWorld, Res, ResMut
};

use super::{acceleration_structures::prelude::AABB, models::model::Model};

#[derive(Debug, Hash, PartialEq, Eq, Clone, DerefMut, Deref)]
pub struct ObjectId(pub String);

#[derive(Debug)]
pub struct Object {
    instance_count: std::ops::Range<u32>,
    instance_buffer: wgpu::Buffer,
    aabb: AABB,
    pub id: ObjectId,
    pub camera_id: CameraId,
    pub pipeline: PipelineType,
    pub model: Model,
}

impl Object {
    pub fn new(id: ObjectId, camera_id: CameraId, pipeline: PipelineType, model: Model, device: &wgpu::Device) -> Self {
        let instance_count = 0..0;
        let instance_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some(&format!("{:?}, instance_buffer", id)),
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
            id,
            camera_id,
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
    pub objects: HashMap<ObjectId, Object>,
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
        let ids = self.objects.iter().map(|object| object.0.clone()).collect::<Vec<ObjectId>>();

        for id in ids {
            self.update_object_buffer(device, queue, &id, vec![]);
        }
    }

    fn update_object_buffer(
        &mut self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue,
        object_id: &ObjectId, 
        data: Vec<RawRenderComponent>
    ) {
        let object = self.objects.get_mut(object_id).expect(&format!("No object found: {:?}", object_id));

        object.instance_count = 0..data.len() as u32;

        let data = bytemuck::cast_slice(&data);

        object.instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?}, instance_buffer", object.id)),
                contents: data,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        queue.write_buffer(&object.instance_buffer, 0, data);
    }
    
    pub fn insert(&mut self, object: Object) -> Option<Object> {
        match self.objects.entry(object.id.clone()) {
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

    let mut objects: HashMap<&ObjectId, Vec<RawRenderComponent>> = HashMap::new();

    let query = &mut world.query::<&InstanceRenderComponent>();

    for (_, render_component) in query {
        if !render_component.visible {
            continue   
        }
        
        match objects.entry(&render_component.object_id) {
            Entry::Occupied(mut entry) => { 
                entry.get_mut().push(render_component.to_raw()); 
            },
            Entry::Vacant(entry) => {
                entry.insert(vec![render_component.to_raw()]);
            },
        }
    }

    for (id, data) in objects {
        object_registry.update_object_buffer(&state.device(), &state.queue(), id, data);
    }
}