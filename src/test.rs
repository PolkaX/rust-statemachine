// Copyright 2020 PolkaX

use crate::{Event, EventError, EventRet, EventType, StateMachine};

#[test]
fn exit_test() {
    let mut state_machine = StateMachine::new();
    state_machine.run();
    let event = Event::new(EventType::Exit);
    state_machine.send(event);
}
