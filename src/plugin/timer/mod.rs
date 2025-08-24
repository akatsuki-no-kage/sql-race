pub mod event;
pub mod resource;
pub mod system;

use std::time::Duration;

use bevy::app::{App, Plugin, Update};
use resource::Timer;

pub struct TimerPlugin {
    duration: Duration,
}

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<event::Start>()
            .add_event::<event::Finish>()
            .insert_resource(Timer::new(self.duration))
            .add_systems(Update, (system::start, system::tick, system::finish));
    }
}
