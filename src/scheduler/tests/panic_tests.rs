#![cfg(test)]

use core::f64;

use super::{
    super::{
        Res, ResMut, RefWorld, MutWorld, EventWriter, EventReader, Event
    }, 
    create_scheduler, run_scheduler_start,
};

fn wrong_assert_system() {
    panic!("System called");
}

#[test]
#[should_panic(expected = "System called")]
fn system_is_called() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., wrong_assert_system);

    run_scheduler_start(scheduler);
}


fn many_mut_res_system1(_: ResMut<i32>) {}
fn many_mut_res_system2(_: ResMut<i32>) {}

#[test]
#[should_panic(expected = "conflicting access in system; attempting to access i32 mutably twice; consider creating a new phase")]
fn many_mut_res() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., many_mut_res_system1);
    scheduler.add_system(0., many_mut_res_system2);
    scheduler.add_resource(1);
    
    run_scheduler_start(scheduler);
}

fn mut_and_ref_system1(_: Res<i32>) {}
fn mut_and_ref_system2(_: ResMut<i32>) {}

#[test]
#[should_panic(expected = "conflicting access in system; attempting to access i32 mutably and immutably at the same time; consider creating a new phase")]
fn mut_and_res() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., mut_and_ref_system1);
    scheduler.add_system(0., mut_and_ref_system2);
    scheduler.add_resource(1);

    run_scheduler_start(scheduler);
}

fn ref_and_mut_world_system1(_: RefWorld) {}
fn ref_and_mut_world_system2(_: MutWorld) {}

#[test]
#[should_panic(expected = "conflicting access in system; attempting to access hecs::world::World mutably and immutably at the same time; consider creating a new phase")]
fn ref_and_mut_world() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., ref_and_mut_world_system1);
    scheduler.add_system(0., ref_and_mut_world_system2);

    run_scheduler_start(scheduler);
}

fn mut_world_system1(_: MutWorld) {}
fn mut_world_system2(_: MutWorld) {}

#[test]
#[should_panic(expected = "conflicting access in system; attempting to access hecs::world::World mutably twice; consider creating a new phase")]
fn get_many_mut_world() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., mut_world_system1);
    scheduler.add_system(0., mut_world_system2);

    run_scheduler_start(scheduler);
}

struct Event1 {}
impl Event for Event1 {}

fn event_writer_system(_: EventWriter<Event1>) {}

fn event_reader_system(_: EventReader<Event1>) {}

#[test]
#[should_panic(expected = "assertion `left == right` failed: conflicting access in system; attempting to access promethius::scheduler::tests::panic_tests::Event1 mutably and immutably at the same time; consider creating a new phase\n  left: Write\n right: Read")]
fn event_send_read_same_phase() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., event_writer_system);
    scheduler.add_system(0., event_reader_system);
    scheduler.add_event::<Event1>();

    run_scheduler_start(scheduler);
}

#[test]
#[should_panic(expected = "conflicting access in system; attempting to access promethius::scheduler::tests::panic_tests::Event1 mutably and immutably at the same time; consider creating a new phase")]
fn event_read_send_same_phase() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., event_reader_system);
    scheduler.add_system(0., event_writer_system);
    scheduler.add_event::<Event1>();

    run_scheduler_start(scheduler);
}

fn dummy_system() {}

#[test]
#[should_panic(expected = "expected a number x: 0 <= x < 4; found NAN")]
fn nan_phase() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(f64::NAN, dummy_system);
}

#[test]
#[should_panic(expected = "expected a number x: 0 <= x < 4; found -0.1")]
fn out_of_bounds_phase1() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(-0.1, dummy_system)
}

#[test]
#[should_panic(expected = "expected a number x: 0 <= x < 4; found 4")]
fn out_of_bounds_phase2() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(4., dummy_system)
}