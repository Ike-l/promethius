use std::any::{
    type_name, TypeId
};

use small_derive_deref::{
    Deref, DerefMut
};

use super::{
    Access, AccessMap, Res, ResMut, SystemParam, TypeMap
};

#[derive(Deref)]
pub struct RefWorld<'a> {
    pub world: Res<'a, hecs::World>,
}

#[derive(Deref, DerefMut)]
pub struct MutWorld<'a> {
    pub world: ResMut<'a, hecs::World>,
}

#[derive(Deref, DerefMut)]
pub struct CommandBuffer<'a> {
    pub command_buffer: ResMut<'a, hecs::CommandBuffer>,
}

impl<'a> SystemParam for RefWorld<'a> {
    type Item<'new> = RefWorld<'new>;

    fn accesses(access: &mut AccessMap) {
        assert_eq!(
            *access.entry(TypeId::of::<hecs::World>()).or_insert(Access::Read),
            Access::Read,
            "conflicting access in system; attempting to access {} mutably and immutably at the same
            time; consider creating a new phase",
            type_name::<hecs::World>(),
        );
    }

    unsafe fn retrieve<'r>(resources: &'r TypeMap) -> Self::Item<'r> {
        let value = Self::retrieve_by_type::<hecs::World>(resources);
        RefWorld { 
            world: Res { value }
        }
    }
}

impl<'a> SystemParam for MutWorld<'a> {
    type Item<'new> = MutWorld<'new>;

    fn accesses(access: &mut AccessMap) {
        match access.insert(TypeId::of::<hecs::World>(), Access::Write) {
            Some(Access::Write) => panic!(
                "conflicting access in system; attempting to access {} mutably twice; consider creating a new phase", 
                type_name::<hecs::World>()
            ),
            Some(Access::Read) => panic!(
                "conflicting access in system; attempting to access {} mutably and immutably at the same time; consider creating a new phase", 
                type_name::<hecs::World>()
            ),
            None => (),
        }
    }

    unsafe fn retrieve<'r>(resources: &'r TypeMap) -> Self::Item<'r> {
        let value = Self::retrieve_by_type_mut::<hecs::World>(resources);
        MutWorld { 
            world: ResMut { value }
        }
    }
}

impl<'a> SystemParam for CommandBuffer<'a> {
    type Item<'new> = CommandBuffer<'new>;

    fn accesses(_access: &mut AccessMap) {
        // any number of mutable or single immutable but can only retrieve mutably so doesnt matter.
    }

    unsafe fn retrieve<'r>(resources: &'r TypeMap) -> Self::Item<'r> {
        let value = Self::retrieve_by_type_mut::<hecs::CommandBuffer>(resources);
        CommandBuffer {
            command_buffer: ResMut { value }
        }
    }
}

