#![cfg(test)]

use core::f64;

use small_derive_deref::{
    Deref, DerefMut
};

use super::{
    super::{ 
        Res,  ResMut,  RefWorld, MutWorld, CommandBuffer, EventWriter, EventReader, Event,
    }, 
    create_scheduler, run_scheduler_tick, run_scheduler_start
};

#[derive(Deref)]
struct Res1(i32);

fn single_res_system(res1: Res<Res1>) {
    assert_eq!(**res1, 1);
}

#[test]
fn single_res() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., single_res_system);
    scheduler.add_resource(Res1(1));

    run_scheduler_start(scheduler);
}

#[derive(Deref)]
struct Res2(i32);

fn many_res_system1(res1: Res<Res1>, res2: Res<Res2>) {
    assert_eq!(**res1, 1);
    assert_eq!(**res2, 3);
}

fn many_res_system2(res1: Res<Res1>, res2: Res<Res2>) {
    assert_eq!(**res1, 1);
    assert_eq!(**res2, 3);
}

#[test]
fn multiple_res() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., many_res_system1);
    scheduler.add_system(0., many_res_system2);
    scheduler.add_resource(Res1(1));
    scheduler.add_resource(Res2(3));

    run_scheduler_start(scheduler);
}

#[derive(Deref, DerefMut)]
struct ResMut1(i32);

fn single_mut_system(mut mut_res1: ResMut<ResMut1>) {
    assert_eq!(**mut_res1, 1);
    **mut_res1 = 2;
}

fn check_mutate_phase_single_mut_system(mut_res1: ResMut<ResMut1>) {
    assert_eq!(**mut_res1, 2);
}

fn check_mutate_tick_single_mut_system(mut_res1: ResMut<ResMut1>) {
    assert_eq!(**mut_res1, 2);
}

#[test]
fn mut_res() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., single_mut_system);
    scheduler.add_system(0.1, check_mutate_phase_single_mut_system);
    scheduler.add_system(1., check_mutate_tick_single_mut_system);
    scheduler.add_resource(ResMut1(1));

    let scheduler = run_scheduler_start(scheduler);
    run_scheduler_tick(scheduler);
}

fn access_mut_over_phases_system1(_: ResMut<ResMut1>) {}
fn access_mut_over_phases_system2(_: ResMut<ResMut1>) {}

#[test]
fn access_mut_over_phases() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., access_mut_over_phases_system1);
    scheduler.add_system(0.1, access_mut_over_phases_system2);
    scheduler.add_resource(ResMut1(1));

    run_scheduler_start(scheduler);
}

fn spawn_into_world_system(mut world: MutWorld) {
    world.spawn((1, true));
}

fn check_world_phase_system(world: RefWorld) {
    for (_, (n, b)) in world.query::<(&i32, &bool)>().iter() {
        assert_eq!(n, &1);
        assert_eq!(b, &true);
    }
}

fn check_world_tick_system(world: RefWorld) {
    for (_, (n, b)) in world.query::<(&i32, &bool)>().iter() {
        assert_eq!(n, &1);
        assert_eq!(b, &true);
    }
}

#[test]
fn persistent_world() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., spawn_into_world_system);
    scheduler.add_system(0.1, check_world_phase_system);
    scheduler.add_system(1., check_world_tick_system);

    let scheduler = run_scheduler_start(scheduler);
    run_scheduler_tick(scheduler);
}

fn get_ref_world_system1(_: RefWorld) {}
fn get_ref_world_system2(_: RefWorld) {}

#[test]
fn get_many_ref_world() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., get_ref_world_system1);
    scheduler.add_system(0., get_ref_world_system2);

    run_scheduler_start(scheduler);
}

fn command_buffer_spawn_system(mut command: CommandBuffer) {
    command.spawn((3, false));
}

fn check_command_buffer_phase_system(world: RefWorld) {
    assert_eq!(world.len(), 0);
}

fn check_command_buffer_tick_system(world: RefWorld) {
    for (_, (n, b)) in world.query::<(&i32, &bool)>().iter() {
        assert_eq!(n, &3);
        assert_eq!(b, &false);
    }
}

#[test]
fn command_buffer_on_world() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., command_buffer_spawn_system);
    scheduler.add_system(0.1, check_command_buffer_phase_system);
    scheduler.add_system(1., check_command_buffer_tick_system);

    let scheduler = run_scheduler_start(scheduler);
    run_scheduler_tick(scheduler);
}

fn get_ref_command_buffer_system1(_: CommandBuffer) {}
fn get_ref_command_buffer_system2(_: CommandBuffer) {}

#[test]
fn get_many_ref_command_buffer() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., get_ref_command_buffer_system1);
    scheduler.add_system(0., get_ref_command_buffer_system2);

    run_scheduler_start(scheduler);
}

#[derive(Debug)]
struct Event1 {
    payload: bool,
}
impl Event for Event1 {}

fn single_event_writer_system(mut event1: EventWriter<Event1>) {
    event1.send(Event1 {
        payload: true,
    })
}

fn single_event_reader_system(event1: EventReader<Event1>) {
    let vec_event: Vec<_> = event1.read().collect();
    assert_eq!(vec_event.len(), 1);
    for e in event1.read() {
        assert_eq!(e.payload, true);
    }
}

#[test]
fn event_send_read_works() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., single_event_writer_system);
    scheduler.add_system(0.1, single_event_reader_system);
    scheduler.add_event::<Event1>();

    run_scheduler_start(scheduler);
}

fn many_event_writer_system1(mut event1: EventWriter<Event1>) {
    event1.send(Event1 {
        payload: true,
    })
}

fn many_event_writer_system2(mut event1: EventWriter<Event1>) {
    event1.send(Event1 {
        payload: false,
    })
}

fn many_event_reader_system1(event1: EventReader<Event1>) {
    let vec_event: Vec<_> = event1.read().collect();
    assert_eq!(vec_event.len(), 2);
    let mut event1 = event1.read().into_iter();
    assert_eq!(event1.next().unwrap().payload, true);
    assert_eq!(event1.next().unwrap().payload, false);
}

fn many_event_reader_system2(event1: EventReader<Event1>) {
    let vec_event: Vec<_> = event1.read().collect();
    assert_eq!(vec_event.len(), 2);
    let mut event1 = event1.read().into_iter();
    assert_eq!(event1.next().unwrap().payload, true);
    assert_eq!(event1.next().unwrap().payload, false);
}

#[test]
fn many_reader_writer_works() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., many_event_writer_system1);
    scheduler.add_system(0., many_event_writer_system2);
    scheduler.add_system(0.1, many_event_reader_system1);
    scheduler.add_system(0.1, many_event_reader_system2);
    scheduler.add_event::<Event1>();

    run_scheduler_start(scheduler);
}

fn event_queue_cleared_writer_system(mut event1: EventWriter<Event1>) {
    event1.send(Event1 {
        payload: true
    })
}

fn event_queue_cleared_reader_system(event1: EventReader<Event1>, mut count: ResMut<i32>) {
    let event1: Vec<_> = event1.read().collect();
    let expected_length = if *count == 0 { 1 } else { 0 };
    assert_eq!(event1.len(), expected_length);
    *count += 1;
}

#[test]
fn event_queue_cleared() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., event_queue_cleared_writer_system);
    scheduler.add_system(1., event_queue_cleared_reader_system);
    scheduler.add_resource(0);
    scheduler.add_event::<Event1>();

    let scheduler = run_scheduler_start(scheduler);
    run_scheduler_tick(scheduler);
}

fn event_queue_auto_cleared_writer_system(mut event1: EventWriter<Event1>) {
    event1.send(Event1 {
        payload: true
    })
}

fn event_queue_auto_cleared_reader_system(event1: EventReader<Event1>) {
    let event1: Vec<_> = event1.read().collect();
    assert_eq!(event1.len(), 1);
    assert_eq!(event1.first().unwrap().payload, true);
}

#[test]
fn event_queue_auto_cleared_over_tick() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(1., event_queue_auto_cleared_writer_system);
    scheduler.add_system(1.1, event_queue_auto_cleared_reader_system);
    scheduler.add_event::<Event1>();

    run_scheduler_tick(scheduler);
}

fn float_precision_test_system1(mut res: ResMut<i32>) {
    assert_eq!(*res, 1);
    *res *= 2;
}
fn float_precision_test_system2(mut res: ResMut<i32>) {
    assert_eq!(*res, 2);
    *res *= 2;
}
fn float_precision_test_system3(mut res: ResMut<i32>) {
    assert_eq!(*res, 4);
    *res *= 2;
}
fn float_precision_test_system4(mut res: ResMut<i32>) {
    assert_eq!(*res, 8);
    *res *= 2;
}
fn float_precision_test_system5(mut res: ResMut<i32>) {
    assert_eq!(*res, 16);
    *res *= 2;
}
fn float_precision_test_system6(res: ResMut<i32>) {
    assert_eq!(*res, 32);
}

#[test]
fn float_precision_order_test() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0.5000000000000001 + 2. * f64::EPSILON, float_precision_test_system6);
    scheduler.add_system(0.49999999999999994, float_precision_test_system3);
    scheduler.add_system(0.5000000000000001, float_precision_test_system4);
    scheduler.add_system(f64::MIN_POSITIVE / 2.0, float_precision_test_system2);
    scheduler.add_system(0.5000000000000001 + f64::EPSILON, float_precision_test_system5);
    scheduler.add_system(0., float_precision_test_system1);
    scheduler.add_resource(1);

    run_scheduler_start(scheduler);
}

fn add_same_type_system(res1: Res<i32>, res2: Res<i32>) {
    assert_eq!(*res1, 2);
    assert_eq!(*res2, 2);
}

#[test]
fn add_same_type() {
    let mut scheduler = create_scheduler();
    scheduler.add_system(0., add_same_type_system);
    scheduler.add_resource(1 as i32);
    scheduler.add_resource(2 as i32);

    run_scheduler_start(scheduler);
}