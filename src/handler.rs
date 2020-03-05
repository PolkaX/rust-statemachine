// Copyright 2020 PolkaX

use crate::{EventError, EventRet};

pub trait Handler {
    fn handle(&self) -> Result<EventRet, EventError>;
}
