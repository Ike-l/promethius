use super::{
    render_component::CameraRenderComponent, 
    DeviceEventBus, WindowEventBus
};

pub trait CameraController: std::fmt::Debug + Send + Sync {
    fn write_window_event(&mut self, event: &WindowEventBus);
    fn write_device_event(&mut self, event: &DeviceEventBus);
    fn update_camera(&mut self, camera: &mut CameraRenderComponent, dt: std::time::Duration);
}

