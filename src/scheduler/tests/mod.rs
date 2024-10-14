#![cfg(test)]

use super::Scheduler;

mod pass_tests;
mod panic_tests;

fn run_scheduler_tick(mut scheduler: Scheduler) -> Scheduler {
    scheduler.run(Scheduler::TICK, Scheduler::END);
    scheduler.run(Scheduler::TICK, Scheduler::END);
    scheduler.run(Scheduler::TICK, Scheduler::END);
 
    scheduler
}

fn run_scheduler_start(mut scheduler: Scheduler) -> Scheduler {
    scheduler.run(Scheduler::START, Scheduler::TICK);
    scheduler
}

fn create_scheduler() -> Scheduler {
    let mut scheduler = Scheduler::default();
    scheduler.add_resource(hecs::World::new());
    scheduler.add_resource(hecs::CommandBuffer::new());
    scheduler
}