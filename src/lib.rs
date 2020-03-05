// Copyright 2020 PolkaX

use shrev::EventChannel;
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {}

pub trait Planner {
    fn plan(&self, events: &[Event]);
}

pub struct StateMachine {
    events_capacity: usize,
    sender: Option<Sender<Event>>,
}

impl Planner for StateMachine {
    fn plan(&self, events: &[Event]) {
        for event in events {
            self.send(event.clone());
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventError {}

impl StateMachine {
    pub fn new(&self, capacity: Option<usize>) -> Self {
        StateMachine {
            events_capacity: capacity.unwrap_or(20),
            sender: None,
        }
    }

    pub fn send(&self, event: Event) -> Option<EventError> {
        let sender = self.sender.as_ref().unwrap();
        let _error = sender.send(event);
        None
    }

    pub fn run(&mut self) {
        let events_capacity = self.events_capacity;
        let (sender, receiver) = channel();
        self.sender = Some(sender);
        let _handle = thread::spawn(move || {
            let mut pending_events = EventChannel::<Event>::with_capacity(events_capacity);
            loop {
                let iter = receiver.iter();
                for event in iter {
                    pending_events.single_write(event);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {}
