use std::{sync::mpsc, time::Duration};

use tuirealm::{
    Event, ListenerError,
    listener::{ListenerResult, Poll},
};

use crate::config::CONFIG;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum UserEvent {
    Start,
    End,
}

pub struct UserEventPort {
    pub rx: mpsc::Receiver<UserEvent>,
}

impl Poll<UserEvent> for UserEventPort {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        match self
            .rx
            .recv_timeout(Duration::from_millis(CONFIG.event_time_out))
        {
            Ok(event) => ListenerResult::Ok(Some(Event::User(event))),
            Err(_) => ListenerResult::Err(ListenerError::PollFailed),
        }
    }
}
