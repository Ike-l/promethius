use std::{
    any::{
        type_name, TypeId
    }, 
    ops::{
        Deref, DerefMut
    }
};

use super::{
    SystemParam, AccessMap, TypeMap
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
}

#[derive(Debug)]
pub struct Res<'a, T: 'static> {
    pub value: &'a T,
}

#[derive(Debug)]
pub struct ResMut<'a, T: 'static> {
    pub value: &'a mut T,
}

impl<T: 'static> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: 'static> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: 'static> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}


impl<'res, T: 'static> SystemParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn accesses(access: &mut AccessMap) {
        assert_eq!(
            *access.entry(TypeId::of::<T>()).or_insert(Access::Read),
            Access::Read,
            "conflicting access in system; attempting to access {} mutably and immutably at the same
            time; consider creating a new phase",
            type_name::<T>(),
        );
    }

    unsafe fn retrieve<'r>(resources: &'r TypeMap) -> Self::Item<'r> {
        let value = Self::retrieve_by_type::<T>(resources);
        Res { value }
    }
}

impl<'res, T: 'static> SystemParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn accesses(access: &mut AccessMap) {
        match access.insert(TypeId::of::<T>(), Access::Write) {
            Some(Access::Read) => panic!(
                "conflicting access in system; attempting to access {} mutably and immutably at the same time; consider creating a new phase", 
                type_name::<T>()
            ),
            Some(Access::Write) => panic!(
                "conflicting access in system; attempting to access {} mutably twice; consider creating a new phase", 
                type_name::<T>()
            ),
            None => (),
        }
    }

    unsafe fn retrieve<'r>(resources: &'r TypeMap) -> Self::Item<'r> {
        let value = Self::retrieve_by_type_mut::<T>(resources);
        ResMut { value }
    }
}
