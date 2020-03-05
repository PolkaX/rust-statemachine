// Copyright 2020 PolkaX

use crate::EventError;

pub trait Handle {
    fn handle(&self) -> Option<EventError>;
}


