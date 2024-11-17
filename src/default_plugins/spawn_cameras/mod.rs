use camera_plugin::prelude::*;
use render_plugin::prelude::State;

use crate::prelude::*;

pub struct DefaultCameraPlugin;

impl PluginTrait for DefaultCameraPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(0.1123, spawn_cameras);
    }
    fn id(&self) -> PluginId {
        PluginId("slingshot_DefaultSpawnCameras".to_string())
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
        CameraId("ortho".to_string())
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
        CameraId("persp".to_string())
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
        CameraId("stationary_ortho".to_string())
    );

    world.spawn(camera);
}
