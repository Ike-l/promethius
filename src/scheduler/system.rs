// https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch

use std::{
    marker::PhantomData, fmt, 
    any::{
        type_name, TypeId
    }
};

use super::{
    AccessMap, TypeMap
};

pub trait System {
    fn run(
        &mut self, 
        resources: &TypeMap, 
        access: &mut AccessMap
    );
}

impl fmt::Debug for dyn System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "System trait object")
    }
}

pub trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

pub trait SystemParam {
    type Item<'new>;

    fn accesses(access: &mut AccessMap);

    unsafe fn retrieve<'r>(resources: &'r TypeMap) -> Self::Item<'r>;

    unsafe fn retrieve_by_type<'r, T: 'static>(resources: &'r TypeMap) -> &T {
        log::info!("Retrieving type immutably: {:?}", type_name::<T>());
        let unsafe_cell = resources
            .get(&TypeId::of::<T>())
            .expect(&format!("Retrieving resource: {:?}", type_name::<T>()));
        
        let value_box = &*unsafe_cell.get();
        let value = value_box.downcast_ref::<T>()
            .expect(&format!("Downcasting resource: {:?}", type_name::<T>()));

        value
    }

    unsafe fn retrieve_by_type_mut<'r, T: 'static>(resources: &'r TypeMap) -> &mut T {
        log::info!("Retrieving type mutably: {:?}", type_name::<T>());
        let unsafe_cell = resources
            .get(&TypeId::of::<T>())
            .expect(&format!("Retrieving resource: {:?}", type_name::<T>()));
        
        let value_box = &mut *unsafe_cell.get();
        let value = value_box.downcast_mut::<T>()
            .expect(&format!("Unboxing resource: {:?}", type_name::<T>()));

        value
    }

}

macro_rules! impl_system {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, $($params: SystemParam),*> System for FunctionSystem<($($params,)*), F>
            where
                for<'a, 'b> &'a mut F:
                    FnMut( $($params),* ) +
                    FnMut( $(<$params as SystemParam>::Item<'b>),* )
        {
            fn run(&mut self, resources: &TypeMap, accesses: &mut AccessMap) {
                fn call_inner<$($params),*>(
                    mut f: impl FnMut($($params),*),
                    $($params: $params),*
                ) {
                    f($($params),*)
                }

                $(
                    $params::accesses(accesses);
                )*

                $(
                    let $params = unsafe { $params::retrieve(resources) };
                )*

                call_inner(&mut self.f, $($params),*)
            }
        }
    }
}

macro_rules! impl_into_system {
    (
        $($params:ident),*
    ) => {
        impl<F, $($params: SystemParam),*> IntoSystem<($($params,)*)> for F
            where
                for<'a, 'b> &'a mut F:
                    FnMut( $($params),* ) +
                    FnMut( $(<$params as SystemParam>::Item<'b>),* )
        {
            type System = FunctionSystem<($($params,)*), Self>;

            fn into_system(self) -> Self::System {
                FunctionSystem {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}

pub struct FunctionSystem<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

// want to simplify this once i master macros 

impl_system!();
impl_system!(T1);
impl_system!(T1, T2);
impl_system!(T1, T2, T3);
impl_system!(T1, T2, T3, T4);
impl_system!(T1, T2, T3, T4, T5);
impl_system!(T1, T2, T3, T4, T5, T6);
impl_system!(T1, T2, T3, T4, T5, T6, T7);
impl_system!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

impl_into_system!();
impl_into_system!(T1);
impl_into_system!(T1, T2);
impl_into_system!(T1, T2, T3);
impl_into_system!(T1, T2, T3, T4);
impl_into_system!(T1, T2, T3, T4, T5);
impl_into_system!(T1, T2, T3, T4, T5, T6);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_into_system!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);