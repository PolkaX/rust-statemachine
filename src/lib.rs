// Copyright 2020 PolkaX

mod event;
mod handler;
mod state_machine;
mod state;
mod sector_info;
#[cfg(test)]
mod test;

pub use event::{Event, EventError, EventRet, EventType};
pub use handler::Handler;
pub use state_machine::StateMachine;
pub use state::SectorState;

pub trait Planner {
    fn plan(&self, events: &[Event]);
}
