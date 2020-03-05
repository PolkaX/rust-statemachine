// Copyright 2020 PolkaX

use crate::Handler;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventRet {
    Exit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Exit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    event_type: EventType,
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Event{
            event_type,
        }
    } 
}

impl Handler for Event {
    fn handle(&self) -> Result<EventRet, EventError> {
        match &self.event_type {
            Exit => Ok(EventRet::Exit),
        }
    }
}
