use std::{
    any::{
        Any, TypeId, type_name
    }, 
    collections::VecDeque
};

use super::{
    Access, AccessMap, Res, ResMut, SystemParam, TypeMap
};

pub trait Event: 'static {}

#[derive(Debug)]
pub struct EventQueue<E: Event> {
    // tick life only needs a single bit so u8 is fine
    events: VecDeque<(E, u8)>,
}

impl<E: Event> Event for EventQueue<E> {}

impl<E: Event> EventQueue<E> {
    pub fn new() -> Self {
        EventQueue {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: E) {
        // tick life starts at 0
        self.events.push_back((event, 0));
    }

    // gets the queue without the tick life
    pub fn copy_of_events(&self) -> impl Iterator<Item = &E> + '_ {
        self.events.iter()
            .map(|(e, _)| e)
    }

    fn increment_tick_count(&mut self) {
        self.events.iter_mut().for_each(|(_, t)| *t += 1);
    }

    // clear a queue after 1 tick so events can't be processed twice
    // will need to research why bevy does it after 2
    fn clean_up(&mut self) {
        self.events.retain(|&(_, t)| t < 1)
    }
}

// trait so i can get the queues from within scheduler
pub trait EventQueueHandler {
    fn increment_and_cleanup(&mut self);

    // needed in res
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<E: Event> EventQueueHandler for EventQueue<E> {
    fn increment_and_cleanup(&mut self) {
        self.increment_tick_count();
        self.clean_up();
    }
    fn as_any(&self) -> &dyn Any {
        self   
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self   
    }
}


#[derive(Debug)]
pub struct EventReader<'a, E: Event> {
    // what semantic purpose would writing to an already sent event do? 
    // would just introduce race conditions
    pub(crate) events: Res<'a, EventQueue<E>>,
}

#[derive(Debug)]
pub struct EventWriter<'a, E: Event> {
    pub(crate) events: ResMut<'a, EventQueue<E>>,
}

impl<'res, E: Event> SystemParam for EventReader<'res, E> {
    type Item<'new> = EventReader<'new, E>;

    fn accesses(access: &mut AccessMap) {
        assert_eq!(
            *access.entry(TypeId::of::<EventQueue<E>>()).or_insert(Access::Read), Access::Read,
            "conflicting access in system; attempting to access {} mutably and immutably at the same time; consider creating a new phase",
            type_name::<E>(),
        );
    }

    unsafe fn retrieve<'r>(resources: &'r TypeMap) -> Self::Item<'r> {
        let unsafe_cell = resources
            .get(&TypeId::of::<EventQueue<E>>())
            .expect(&format!("Retrieving event: {:?}", type_name::<E>()));
        
        let value_box = &*unsafe_cell.get();
        let value = value_box
            .downcast_ref::<Box<dyn EventQueueHandler>>()
            .expect(&format!("Downcasting event: {:?}", type_name::<E>()))
            .as_any()
            .downcast_ref::<EventQueue<E>>()
            .expect(&format!("Downcasting event: {:?}", type_name::<E>()));

        EventReader {
            events: Res { value }
        }
    }
}

impl<'res, E: Event> SystemParam for EventWriter<'res, E> {
    type Item<'new> = EventWriter<'new, E>;

    fn accesses(access: &mut AccessMap) {
        match access.insert(TypeId::of::<EventQueue<E>>(), Access::Write) {
            Some(Access::Read) => panic!(
                "conflicting access in system; attempting to access {} mutably and immutably at the same time; consider creating a new phase", 
                type_name::<E>()
            ),
            Some(Access::Write) => (),
            None => (),
        }
    }

    unsafe fn retrieve<'r>(resources: &'r TypeMap) -> Self::Item<'r> {
        let unsafe_cell = resources
            .get(&TypeId::of::<EventQueue<E>>())
            .expect(&format!("Retrieving event: {:?}", type_name::<E>()));
        
        let value_box = &mut *unsafe_cell.get();
        let value = value_box
            .downcast_mut::<Box<dyn EventQueueHandler>>()
            .expect(&format!("Downcasting event: {:?}", type_name::<E>()))
            .as_any_mut()
            .downcast_mut::<EventQueue<E>>()
            .expect(&format!("Downcasting event: {:?}", type_name::<E>()));

        EventWriter {
            events: ResMut { value }
        }
    }
}

impl<'a, E: Event> EventReader<'a, E> {
    pub fn read(&self) -> impl Iterator<Item = &E> + '_ {
        // multiple systems can "own" the events
        self.events.copy_of_events()        
    }
}

impl<'a, E: Event> EventWriter<'a, E> {
    pub fn send(&mut self, event: E) {
        self.events.push(event);
    }
}
