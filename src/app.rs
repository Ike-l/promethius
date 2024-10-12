use std::collections::HashMap;

use small_derive_deref::{
    Deref, 
    DerefMut,
};

use winit::{
    application::ApplicationHandler, 
    event_loop::ActiveEventLoop, 
    window::WindowId,
    event::{
        DeviceEvent, 
        WindowEvent
    }, 
};

const CONTROL_FLOW: winit::event_loop::ControlFlow = winit::event_loop::ControlFlow::Poll;

#[derive(Debug, Deref, DerefMut, Event)]
pub struct WindowEventBus(pub WindowEvent);

#[derive(Debug, Deref, DerefMut, Event)]
pub struct DeviceEventBus(pub DeviceEvent);

#[derive(Debug, Deref, DerefMut)]
pub struct AppBuilder {
    app: App,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            app: App::default()
        }
    }
}

impl AppBuilder {
    pub fn run(&mut self) -> Result<(), winit::error::EventLoopError> {
        match winit::event_loop::EventLoop::new() {
            Ok(event_loop) => {
                event_loop.set_control_flow(CONTROL_FLOW);
                event_loop.run_app(&mut self.app)
            },
            Err(e) => {
                panic!("Creating the event loop\n{:?}", e)
            }
        }
    }
}

#[derive(Debug)]
pub struct App {
    scheduler: Scheduler
}

impl Default for App {
    fn default() -> Self {
        Self {
            scheduler: Scheduler::default()
        }
    }
}

impl App {
    pub fn add_plugin(&mut self, plugin: Box<dyn PluginTrait>) {
        plugin.build(self);
    }

    pub fn add_plugins(&mut self, plugins: Vec<Box<dyn PluginTrait>>) {
        plugins
            .into_iter()
            .for_each(|plugin| self.add_plugin(plugin)
        );
    }

    pub fn add_system<I, S: System + 'static>(
        &mut self,
        phase: f64,
        system: impl IntoSystem<I, System = S>
    ) {
        self.scheduler.add_system(phase, system);
    }

    pub fn add_resource<R: 'static>(&mut self, resource: R) {
        self.scheduler.add_resource(resource);
    }

    pub fn remove_resource<R: 'static>(&mut self, resource: R) {
        self.scheduler.remove_resource(resource);
    }

    pub fn remove_resource_by_type<R: 'static>(&mut self) {
        self.scheduler.remove_resource_by_type::<R>();
    }

    pub fn add_event<E: Event>(&mut self) {
        self.scheduler.add_event::<E>();
    }

    pub fn get_resource_mut<T: 'static>(&self) -> Option<&mut T> {
        self.scheduler.get_resource_mut::<T>()
    }

    pub fn get_event_reader<E: Event>(&self) -> Option<EventReader<E>> {
        self.scheduler.get_event_reader::<E>()
    }

    pub fn get_event_writer<E: Event>(&self) -> Option<EventWriter<E>> {
        self.scheduler.get_event_writer::<E>()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Needed for creating "states", for the window
        let raw_event_loop: *const ActiveEventLoop = event_loop;
        unsafe {
            let event_loop_ref: &ActiveEventLoop = &*raw_event_loop;
            self.add_resource(event_loop_ref);
        }

        self.add_resource(hecs::World::new());
        self.add_resource(hecs::CommandBuffer::new());

        self.add_event::<WindowEventBus>();
        self.add_event::<DeviceEventBus>();

        self.scheduler.run(Scheduler::START, Scheduler::TICK);
        self.remove_resource_by_type::<&ActiveEventLoop>();
    }
    
    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        let redraw_requested = match event {
            WindowEvent::RedrawRequested => true,
            WindowEvent::CloseRequested => {
                self.scheduler.run(Scheduler::END, Scheduler::EXIT);
                event_loop.exit();
                false
            },
            _ => {
                match self.get_event_writer::<WindowEventBus>() {
                    Some(window_event_bus) => {
                        window_event_bus.send(WindowEventBus(event));
                    },
                    None => {
                        panic!("Event received before creation of 'WindowEventBus'")
                    }
                };
                false
            }
        };

        if redraw_requested {
            self.scheduler.run(Scheduler::TICK, Scheduler::END);
            match self.get_resource_mut::<HashMap<WindowId, State>>() {
                Some(states) => {
                    match states.get(&window_id) {
                        Some(state) => {
                            state.window().reqest_redraw();
                        },
                        None => {
                            panic!("Retrieving 'State' using 'WindowId': 'fn window_event'")
                        }
                    }
                },
                None => {
                    panic!("Retriving 'Vec<State>' from 'app'")
                }
            }
        }
    }

    fn device_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        match self.get_event_writer::<DeviceEventBus>() {
            Some(mut device_event_bus) => {
                device_event_bus.send(DeviceEventBus(event))
            },
            None => {
                panic!("Event received before creation of 'DeviceEventBus'")
            }
        }
    }
}