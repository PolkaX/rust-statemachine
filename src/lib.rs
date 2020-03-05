// Copyright 2020 PolkaX

mod event;
mod handle;
mod state_machine;

pub use event::{Event, EventError};
pub use state_machine::{StateMachine};

pub trait Planner {
    fn plan(&self, events: &[Event]);
}


#[cfg(test)]
mod tests {}
