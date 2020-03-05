// Copyright 2020 PolkaX

use crate::{Event, EventError, EventRet, Handler, Planner};
use shrev::EventChannel;
use std::sync::mpsc::{channel, Sender};
use std::thread;

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
    pub fn new() -> Self {
        StateMachine { sender: None }
    }

    pub fn send(&self, event: Event) -> Option<EventError> {
        let sender = self.sender.as_ref().unwrap();
        let _error = sender.send(event);
        None
    }

    pub fn run(&mut self) {
        let (sender, receiver) = channel();
        self.sender = Some(sender);
        thread::spawn(move || {
            let mut pending_events = EventChannel::<Event>::with_capacity(EVENTS_CAPACITY);
            let mut reader = pending_events.register_reader();
            loop {
                let iter = receiver.iter();
                for event in iter {
                    pending_events.single_write(event);
                }
                let iter = pending_events.read(&mut reader);
                for event in iter {
                    let ret = event.handle();
                    if ret == Ok(EventRet::Exit) {
                        // To do: 退出清理操作
                        println!("recv Exit signal");
                        break;
                    }
                }
            }
        });
    }
}
