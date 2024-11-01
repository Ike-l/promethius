mod render_config;
mod state;

pub mod prelude {
    use super::*;

    pub use render_config::{
        PipelineType, RenderConfig
    };

    pub use super::{
        WindowDimensions, State
    };
}

use crate::prelude::{
    *, 
    camera_plugin::prelude::{
        CameraRenderComponent, CameraProjectionComponent
    },
};

pub use state::State;

use state::{
    create_states, render_system
};

pub struct RenderPlugin;

impl PluginTrait for RenderPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(0.001, create_states);
        app.add_system(1.500, render_system);
        app.add_system(1.001, input);

        app.add_resource(Vec::<State>::new());
        app.add_resource(WindowDimensions::default());
        
    }
    fn id(&self) -> PluginId {
        PluginId("slingshot_RenderPlugin".to_string())
    }
}

#[derive(Debug, Default)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

fn input(events: EventReader<WindowEventBus>, mut states: ResMut<Vec<State>>, world: RefWorld, window_dimensions: ResMut<WindowDimensions>) {
    let state = states.first_mut().expect("State not found.\nAdd it as a resource in build");
    
    for event in events.read() {
        match event.0 {
            winit::event::WindowEvent::Resized(physical_size) => {
                let mut query = world.query::<&mut CameraRenderComponent>();

                let mut persp_projections = Vec::new();
                for (_, render_component) in &mut query {
                    match &mut render_component.projection {
                        CameraProjectionComponent::Persp(projection) => {
                            persp_projections.push(projection)
                        },
                        CameraProjectionComponent::Ortho(_) => {},
                    }
                }    

                state.resize(physical_size, &mut persp_projections, window_dimensions.value);
            },
            _ => {}
        }
    }
}
