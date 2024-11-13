use std::{
    time::Duration,
    f32::consts::FRAC_PI_2,
};

use cgmath::InnerSpace;

use winit::{
    dpi::PhysicalPosition, 
    event::{
        ElementState, MouseScrollDelta
    }, 
    keyboard::KeyCode,
};

use super::super::{
    render_component::{
        CameraProjectionComponent, CameraRenderComponent, CameraViewComponent
    }, 
    controller_component::CameraController, 
    WindowEventBus, 
    DeviceEventBus,
};

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub struct PerspController {
    pub left: f32,
    pub right: f32,
    pub forward: f32,
    pub backward: f32,
    pub up: f32,
    pub down: f32,
    pub rotate_horizontal: f32,
    pub rotate_vertical: f32,
    pub scroll: f32,
    pub mouse_pressed: bool,
    pub speed: f32,
    pub sensitivity: f32,
}

impl PerspController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            forward: 0.0,
            backward: 0.0,
            up: 0.0,
            down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            mouse_pressed: false,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) {
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            KeyCode::KeyW => {
                self.forward = amount;
            }
            KeyCode::KeyS => {
                self.backward = amount;
            }
            KeyCode::KeyA => {
                self.left = amount;
            }
            KeyCode::KeyD => {
                self.right = amount;
            }
            KeyCode::Space => {
                self.up = amount;
            }
            KeyCode::ShiftLeft => {
                self.down = amount;
            }
            _ => {}
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        if self.mouse_pressed {
            self.rotate_horizontal = mouse_dx as f32;
            self.rotate_vertical = mouse_dy as f32;
        }
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = match delta {
            MouseScrollDelta::LineDelta(_, scroll) => {
                -scroll * 0.5
            },
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => {
                -*scroll as f32
            },
        };
        self.speed -= self.scroll;
        self.scroll = 0.0;
    }
       
}

impl CameraController for PerspController {
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
            winit::event::WindowEvent::MouseInput {
                button: winit::event::MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = state == ElementState::Pressed;
            },
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                self.process_scroll(&delta);
            },
            _ => {}
        }
    }

    fn write_device_event(&mut self, event: &DeviceEventBus) {
        match event.0 {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                self.process_mouse(delta.0, delta.1)
            },
            _ => {}
        }
    }

    fn update_camera(&mut self, camera: &mut CameraRenderComponent, dt: Duration) {
        if let CameraViewComponent::Persp(view) = &mut camera.view {
            if let CameraProjectionComponent::Persp(_) = &mut camera.projection {
                let dt = dt.as_secs_f32();

                let (yaw_sin, yaw_cos) = view.yaw.0.sin_cos();
                let forward = cgmath::Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
                let right = cgmath::Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
                view.position += forward * (self.forward - self.backward) * self.speed * dt;
                view.position += right * (self.right - self.left) * self.speed * dt;

                let (pitch_sin, pitch_cos) = view.pitch.0.sin_cos();

                let _scrollward = cgmath::Vector3::new(
                    pitch_cos * yaw_cos,
                    pitch_sin, 
                    pitch_cos * yaw_sin
                ).normalize();

                
                //view.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
                view.position.y += (self.up - self.down) * self.speed * dt;

                //self.scroll = 0.0;

                view.yaw += cgmath::Rad(self.rotate_horizontal) * self.sensitivity * dt;
                view.pitch += cgmath::Rad(-self.rotate_vertical) * self.sensitivity * dt;


                self.rotate_horizontal = 0.0;
                self.rotate_vertical = 0.0;

                if view.pitch < -cgmath::Rad(SAFE_FRAC_PI_2) {
                    view.pitch = -cgmath::Rad(SAFE_FRAC_PI_2);
                } else if view.pitch > cgmath::Rad(SAFE_FRAC_PI_2) {
                    view.pitch = cgmath::Rad(SAFE_FRAC_PI_2);
                }
            }
        }
    }
}