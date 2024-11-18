use camera_plugin::prelude::*;
use label_plugin::prelude::LabelComponent;
use render_plugin::prelude::State;

use crate::prelude::*;

pub struct DefaultCameraPlugin;

impl PluginTrait for DefaultCameraPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(0.1123, spawn_cameras);
    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_DefaultSpawnCameras")
    }
}

impl PluginCollisionHandler for DefaultCameraPlugin {
    fn handle_collision<T: std::any::Any>(&mut self, _phase: f64, _levels: u8) {
        // I have no resources
    }
}

pub fn spawn_cameras(mut world: MutWorld, states: Res<Vec<State>>) {
    
    let state = states.first().unwrap();

    let camera = Camera::new(
        CameraRenderComponent::ortho(
            OrthoView::new(
                cgmath::Point3 { 
                    x: 0.0, 
                    y: 0.0,
                    z: 0.0, 
                }
            ), 
            OrthoProjection::new_square(
                10.0, 
                0.1, 
                1000.0
            ),
            state.device(), &state.create_camera_layout("ortho"), "ortho"
        ),
        OrthoController::new(
            4.0, 
            1.0
        ),
        LabelComponent::new("ortho")
    );

    world.spawn(camera);

    let camera = Camera::new(
        CameraRenderComponent::persp(
            PerspView::new(
                cgmath::Point3 { 
                    x: 0.0, 
                    y: 0.0,
                    z: -2.0, 
                },
                0.0,
                0.0
            ), 
            PerspProjection::new(
                state.config().width, 
                state.config().height, 
                cgmath::Deg(45.0),
                0.1, 
                1000.0
            ),
            state.device(), &state.create_camera_layout("persp"), "persp" 
        ),
        PerspController::new(
            4.0, 
            1.0
        ),
        LabelComponent::new("persp")
    );

    world.spawn(camera);


    let camera = Camera::new(
        CameraRenderComponent::ortho(
            OrthoView::new(
                cgmath::Point3 { 
                    x: 0.0, 
                    y: 0.0,
                    z: 0.0, 
                }
            ), 
            OrthoProjection::new_square(
                10.0, 
                0.1, 
                1000.0
            ),
            state.device(), &state.create_camera_layout("stationary_ortho"), "stationary_ortho"
        ),
        OrthoController::new(
            0.0, 
            1.0
        ),
        LabelComponent::new("stationary_ortho")
    );

    world.spawn(camera);
}
