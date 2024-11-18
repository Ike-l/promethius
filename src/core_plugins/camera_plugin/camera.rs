use crate::prelude::{
    render_plugin::prelude::State,
    time_plugin::prelude::Time,
    label_plugin::prelude::LabelComponent,
    EventReader, MutWorld, RefWorld, Res, ResMut, WindowEventBus, DeviceEventBus
};

use super::{
    render_component::{
        CameraRenderComponent, CameraProjectionComponent, CameraViewComponent
    }, 
    controller_component::CameraController, 
};

#[derive(Debug, hecs_macros::Bundle)]
pub struct Camera {
    pub label: LabelComponent,
    render_component: CameraRenderComponent,
    controller: Box<dyn CameraController>,
}

impl Camera {
    pub fn new<T: CameraController + 'static>(
        render_component: CameraRenderComponent, 
        controller: T, 
        label: LabelComponent
    ) -> Self {
        match &render_component.projection {
            CameraProjectionComponent::Ortho(_) => {
                if let CameraViewComponent::Ortho(_) = &render_component.view {} 
                else { panic!("ortho projection needs an ortho view") }
            },
            CameraProjectionComponent::Persp(_) => {
                if let CameraViewComponent::Persp(_) = &render_component.view {} 
                else { panic!("persp projection needs a persp view") }
            }
        }           

        Self {
            render_component,
            controller: Box::new(controller),
            label,
        }
    }
}

pub fn update_camera_bind_group(mut states: ResMut<Vec<State>>, world: RefWorld) {
    let state = states.first_mut().unwrap();

    for (_, render_component) in &mut world.query::<&CameraRenderComponent>() {
        render_component.update_buffers(state.queue());
    }
}

pub fn input(
    window_events: EventReader<WindowEventBus>, 
    device_events: EventReader<DeviceEventBus>, 
    world: MutWorld
) {
    for (_, controller) in &mut world.query::<&mut Box<dyn CameraController>>() {
        for event in window_events.read() {
            controller.write_window_event(event);
        }
    
        for event in device_events.read() {
            controller.write_device_event(event);
        }
    }
}

pub fn update_camera(world: MutWorld, time: Res<Time>) {
    for (_, (controller, camera)) in &mut world.query::<(&mut Box<dyn CameraController>, &mut CameraRenderComponent)>() {
        controller.update_camera(camera, time.dt);
    }
}