use std::{
    any::{
        Any, TypeId,
    }, 
    cell::UnsafeCell, 
    collections::{
        BTreeMap, HashMap, btree_map::Entry
    },
    ops::Bound::{
        Included, Excluded
    },
};

use ordered_float::OrderedFloat;

use super::{
    event::{
        EventQueue, EventQueueHandler
    }, 
    Access, CommandBuffer, Event, EventReader, EventWriter, IntoSystem, System, MutWorld, Res, ResMut,
};

type StoredSystem = Box<dyn System>;
pub type TypeMap = HashMap<TypeId, UnsafeCell<Box<dyn Any>>>;
pub type AccessMap = HashMap<TypeId, Access>;

#[derive(Debug, Default)]
pub struct Scheduler {
    systems: BTreeMap<OrderedFloat<f64>, Vec<StoredSystem>>,
    //systems: Vec<(f64, Vec<StoredSystem>)>,
    resources: TypeMap,
    accesses: AccessMap,
}

impl Scheduler {
    pub const START: f64 = 0.;
    pub const TICK: f64 = 1.;
    pub const END: f64 = 2.;
    pub const EXIT: f64 = 3.;
    
    pub fn run(
        &mut self, 
        start: f64,
        end_exclusive: f64
    ) {
        self.systems
            .range_mut((
                Included(OrderedFloat(start)), 
                Excluded(OrderedFloat(end_exclusive))
            ))
            .for_each(
                |(_, systems)| {
                    systems
                        .iter_mut()
                        .for_each(
                            |system| system.run(&self.resources, &mut self.accesses)
                        );
                        self.accesses.clear();
                });

        if start == Self::TICK { 
            self.process_event_queues();
            self.command_world();
        } else if end_exclusive == Self::TICK {
            self.command_world();
        }
    }

    fn command_world(&self) {
        let mut world = match self.get_world() {
            Some(world) => world,
            None => { log::warn!("Retrieving world"); return }
        };

        let mut command_buffer = match self.get_command_buffer() {
            Some(command_buffer) => command_buffer,
            None => { log::warn!("Retrieving command buffer"); return }
        };

        command_buffer.run_on(&mut world);
    }

    pub fn add_system<I, S: System + 'static>(
        &mut self,
        phase: f64,
        system: impl IntoSystem<I, System = S>
    ) {
        assert!(!phase.is_nan(), "expected a number x: 0 <= x < 4; found NAN");
        assert!(phase < 4. && phase >= 0., "expected a number x: 0 <= x < 4; found {phase}");

        self.systems
            .entry(OrderedFloat(phase))
            .or_insert_with(Vec::new)
            .push(Box::new(system.into_system()));
    }

    pub fn add_resource<R: 'static>(&mut self, res: R) {
        let value = UnsafeCell::new(Box::new(res));

        self.resources.insert(TypeId::of::<R>(), value);
    }

    pub fn remove_resource<R: 'static>(&mut self, _res: R) {
        self.resources.remove(&TypeId::of::<R>());
    }

    pub fn remove_resource_by_type<R: 'static>(&mut self) {
        self.resources.remove(&TypeId::of::<R>());
    }

    pub fn add_event<E: Event>(&mut self) {
        let event_queue: Box<dyn EventQueueHandler> = Box::new(EventQueue::<E>::new());
        self.resources.insert(
            TypeId::of::<EventQueue<E>>(),
            UnsafeCell::new(
                Box::new(event_queue) as Box<dyn Any>
            )
        );
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        unsafe {
            self.resources.get(&TypeId::of::<T>())
                .map(|cell| &*cell.get())
                .and_then(|boxed_resource| boxed_resource.downcast_ref::<T>())
        }
    }

    pub fn get_resource_mut<T: 'static>(&self) -> Option<&mut T> {
        unsafe {
            self.resources.get(&TypeId::of::<T>())
                .map(|cell| &mut *cell.get())
                .and_then(|boxed_resource| boxed_resource.downcast_mut::<T>())
        }
    }

    #[allow(dead_code)]
    fn get_resources<T: 'static>(&mut self) -> Vec<&T> {
        let mut resources = Vec::new();
        for (_, resource) in self.resources.iter() {
            let resource = unsafe { &*resource.get() };
            if let Some(resource) = resource.downcast_ref::<T>() {
                resources.push(resource);
            }
        }
        resources
    }

    fn get_resources_mut<T: 'static>(&mut self) -> Vec<&mut T> {      
        let mut resources = Vec::new();
        for (_, resource) in self.resources.iter_mut() {
            let resource = unsafe { &mut *resource.get() };
            if let Some(resource) = resource.downcast_mut::<T>() {
                resources.push(resource);
            }
        }
        resources
    }

    pub fn get_world<'a>(&'a self) -> Option<MutWorld<'a>> {
        self.get_resource_mut::<hecs::World>().map(|world| MutWorld { world: ResMut { value: world } })
    }

    pub fn get_command_buffer<'a>(&'a self) -> Option<CommandBuffer<'a>> {
        self.get_resource_mut::<hecs::CommandBuffer>().map(|command_buffer| CommandBuffer { command_buffer: ResMut { value: command_buffer } })
    }

    fn get_event_queue<E: Event>(&self) -> Option<&EventQueue<E>> {
        self.get_resource::<Box<dyn EventQueueHandler>>()
            .and_then(|handler| handler.as_any().downcast_ref::<EventQueue<E>>())
    }

    fn get_event_queue_mut<E: Event>(&self) -> Option<&mut EventQueue<E>> {
        self.get_resource_mut::<Box<dyn EventQueueHandler>>()
            .and_then(|handler| handler.as_any_mut().downcast_mut::<EventQueue<E>>())
    }

    pub fn get_event_reader<E: Event>(&self) -> Option<EventReader<E>> {
        self.get_event_queue::<E>().map(
            |event_queue| 
                EventReader {
                    events: Res { value: event_queue }
                }
            )
    }

    pub fn get_event_writer<E: Event>(&self) -> Option<EventWriter<E>> {
        self.get_event_queue_mut::<E>().map(
            |event_queue| 
                EventWriter {
                    events: ResMut { value: event_queue },
                }
            )
    }

    fn process_event_queues(&mut self) {
        self.get_resources_mut::<Box<dyn EventQueueHandler>>()
            .iter_mut()
            .for_each(
            |resource| resource.increment_and_cleanup() 
        );
    }
}


