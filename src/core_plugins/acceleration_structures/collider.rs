use super::{
    aabb::AABB, 
    entity::prelude::InstanceRenderComponent, 
    label_plugin::prelude::LabelComponent, 
    object_plugin::prelude::ObjectRegistry, 
    promethius_std::prelude::Position, 
    RefWorld, Res
};

#[derive(Debug, Clone, PartialEq)]
pub struct ColliderComponent {
    pub bbox: AABB,
}

impl ColliderComponent {
    pub fn new(bbox: AABB) -> Self {
        Self { bbox }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Collider {
    pub collider: ColliderComponent,
    pub entity_label: LabelComponent,
}

impl Collider {
    pub fn new(collider: ColliderComponent, entity_label: LabelComponent) -> Self {
        Self {
            collider,
            entity_label,
        }
    }
}

pub fn update_colliders(world: RefWorld, object_registry: Res<ObjectRegistry>) {
    let mut query = world.query::<(&InstanceRenderComponent, &LabelComponent, &mut ColliderComponent)>();
    for (_, (
        render,
        label, 
        collider)
    ) in &mut query {
        let aabb = match object_registry.objects.get(&label) {
            Some(o) => {
                let old_aabb = o.aabb();

                let transform_position = |pos: &Position| {
                    let transformed = render.model_vertex(cgmath::Vector4 {
                        x: pos.x as f32,
                        y: pos.y as f32,
                        z: pos.z as f32,
                        w: 1.0,
                    });
                    Position::new(transformed.x as f64, transformed.y as f64, transformed.z as f64)
                };

                let new_min = transform_position(&old_aabb.min);
                let new_max = transform_position(&old_aabb.max);

                AABB::new(new_min, new_max)
            },
            None => panic!("No object found with label: {:?}", label)
        };

        collider.bbox = aabb;
    }
}