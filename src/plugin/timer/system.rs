use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::{Res, ResMut},
    },
    time::Time,
};

use super::{
    event::{Finish, Start},
    resource::Timer,
};

pub fn start(mut timer: ResMut<Timer>, mut event_reader: EventReader<Start>) {
    for _ in event_reader.read() {
        timer.reset();
    }
}

pub fn tick(mut timer: ResMut<Timer>, time: Res<Time>) {
    timer.tick(time.delta());
}

pub fn finish(mut timer: ResMut<Timer>, mut event_writer: EventWriter<Finish>) {
    if !timer.finished() {
        return;
    }

    timer.pause();
    event_writer.write_default();
}
