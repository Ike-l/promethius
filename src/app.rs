use small_derive_deref::{
    Deref, DerefMut
};

use event_deriver::EventDeriver;

use winit::{
    application::ApplicationHandler, event::{
        DeviceEvent, WindowEvent
    }, 
    event_loop::{
        ActiveEventLoop, ControlFlow
    }, 
    window::WindowId, event::DeviceId
};

use super::{
    scheduler::{
        Scheduler, Event, EventReader, EventWriter, IntoSystem, System
    },
    core_plugins::render_plugin::prelude::State, plugins::PluginTrait
};

const CONTROL_FLOW: ControlFlow = ControlFlow::Poll;

#[derive(Debug, Deref, DerefMut, EventDeriver)]
pub struct WindowEventBus(pub winit::event::WindowEvent);

#[derive(Debug, Deref, DerefMut, EventDeriver)]
pub struct DeviceEventBus(pub winit::event::DeviceEvent);

#[derive(Debug, Deref, DerefMut, Default)]
pub struct AppBuilder {
    app: App,
}

impl AppBuilder {
    pub fn run(&mut self) -> Result<(), winit::error::EventLoopError> {
        env_logger::init();

        log::info!("Running app");

        let event_loop = winit::event_loop::EventLoop::new()
            .expect(&format!("Creating the event loop"));

        event_loop.set_control_flow(CONTROL_FLOW);
        event_loop.run_app(&mut self.app)
    }
}

#[derive(Debug, Default)]
pub struct App {
    scheduler: Scheduler,
}

impl App {   
    pub fn add_plugin(&mut self, plugin: Box<dyn PluginTrait>) {
        plugin.build(self);
    }

    pub fn add_plugins(&mut self, plugin: Vec<Box<dyn PluginTrait>>) {
        plugin
            .into_iter()
            .for_each(
            |plugin| self.add_plugin(plugin)
        );
    }
    
    pub fn add_system<I, S: System + 'static>(
        &mut self, 
        phase: f64, 
        system: impl IntoSystem<I, System = S>
    ) {
        self.scheduler.add_system(phase, system);
    }
    
    pub fn add_resource<R: 'static>(&mut self, res: R) {
        self.scheduler.add_resource(res);
    }
    
    pub fn remove_resource<R: 'static>(&mut self, res: R) {
        self.scheduler.remove_resource(res);
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
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
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
        log::info!("Running scheduler. Phases: START -> TICK");
        self.scheduler.run(Scheduler::START, Scheduler::TICK);
        self.remove_resource_by_type::<&ActiveEventLoop>();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        log::info!("Received window event: {:?}", event);
        let redraw_requested = match event {
            WindowEvent::RedrawRequested => true,
            WindowEvent::CloseRequested => {
                log::info!("Running scheduler. Phases: END -> EXIT");
                self.scheduler.run(Scheduler::END, Scheduler::EXIT);
                event_loop.exit();
                false
            },
            _ => {
                match self.get_event_writer::<WindowEventBus>() {
                    Some(mut window_event_bus) => {
                        window_event_bus.send(WindowEventBus(event));
                    },
                    None => { log::warn!("Event received before creation of 'WindowEventBus'") }
                };
                false
            }
        };

        if redraw_requested {
            log::info!("Running scheduler. Phases: TICK -> END");
            self.scheduler.run(Scheduler::TICK, Scheduler::END);
            match self.get_resource_mut::<Vec<State>>() {
                Some(states) => {
                    if let Some(state) = states.first() {
                        log::info!("Redraw requested");
                        state.window().request_redraw();
                    }
                },
                None => { log::warn!("Retrieving 'Vec<State>' from 'app'") }
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        log::info!("Received device event: {:?}", event);
        match self.get_event_writer::<DeviceEventBus>() {
            Some(mut device_event_bus) => {
                device_event_bus.send(DeviceEventBus(event))
            },
            None => { log::warn!("Event received before creation of 'DeviceEventBus'") }
        }
    }
}

