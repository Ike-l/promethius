use std::time::Duration;

use winit::{
    event::ElementState, 
    keyboard::KeyCode
};

use super::super::{
    render_component::{
        CameraProjectionComponent, CameraRenderComponent, CameraViewComponent
    }, 
    controller_component::CameraController, 
    WindowEventBus, 
    DeviceEventBus,
};


#[derive(Debug)]
pub struct OrthoController {
    speed: f32,
    sensitivity: f32,
    up: f32,
    down: f32,
    left: f32,
    right: f32,
    forward: f32,
    backward: f32
}

impl OrthoController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            up: 0.0,
            down: 0.0,
            left: 0.0,
            right: 0.0,
            forward: 0.0,
            backward: 0.0,
        }
    }
    
    pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) {
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };

        match key {
            KeyCode::KeyW => {
                self.up = amount;
            },
            KeyCode::KeyS => {
                self.down = amount
            },
            KeyCode::KeyD => {
                self.right = amount;
            },
            KeyCode::KeyA => {
                self.left = amount;
            },
            KeyCode::Space => {
                self.backward = amount;
            },
            KeyCode::ShiftLeft => {
                self.forward = amount;
            },
            _ => {}
        }
    }
}

impl CameraController for OrthoController {
    fn write_window_event(&mut self, event: &WindowEventBus) {
        match event.0 {
            winit::event::WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => self.process_keyboard(key, state),
            _ => {}
        }
    }

    fn write_device_event(&mut self, _event: &DeviceEventBus) {
        
    }

    fn update_camera(&mut self, camera: &mut CameraRenderComponent, dt: Duration) {
        if let CameraViewComponent::Ortho(view) = &mut camera.view {
            if let CameraProjectionComponent::Ortho(projection) = &mut camera.projection {
                let dt = dt.as_secs_f32();
                view.position.x += (self.right - self.left) * self.speed * self.sensitivity * dt;
                view.position.y += (self.up - self.down) * self.speed * self.sensitivity * dt;

                let zoom_factor = (self.backward - self.forward) * self.speed * self.sensitivity * dt;
                if zoom_factor != 0.0 {
                    projection.bottom *= 1.0 + zoom_factor;
                    projection.top *= 1.0 + zoom_factor;
                    projection.left *= 1.0 + zoom_factor;
                    projection.right *= 1.0 + zoom_factor;
                }
            }
        }
    }
}