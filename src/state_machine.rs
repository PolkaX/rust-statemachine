// Copyright 2020 PolkaX

use shrev::EventChannel;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use crate::{Event, EventError, Planner};

const EVENTS_CAPACITY: usize = 20;

pub struct StateMachine {
    sender: Option<Sender<Event>>,
}

impl Planner for StateMachine {
    fn plan(&self, events: &[Event]) {
        for event in events {
            self.send(event.clone());
        }
    }
}

impl StateMachine {
    pub fn new(&self) -> Self {
        StateMachine {
            sender: None,
        }
    }

    pub fn send(&self, event: Event) -> Option<EventError> {
        let sender = self.sender.as_ref().unwrap();
        let _error = sender.send(event);
        None
    }

    pub fn run(&mut self) {
        let (sender, receiver) = channel();
        self.sender = Some(sender);
        let _handle = thread::spawn(move || {
            let mut pending_events = EventChannel::<Event>::with_capacity(EVENTS_CAPACITY);
            loop {
                let iter = receiver.iter();
                for event in iter {
                    pending_events.single_write(event);
                }
            }
        });
    }
}

