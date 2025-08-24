use bevy::ecs::{event::EventReader, system::ResMut};

use super::{
    event::{Change, Restart},
    resource::User,
};

pub fn change(mut user: ResMut<User>, mut event_reader: EventReader<Change>) {
    for change in event_reader.read() {
        user.name = change.name.clone();
    }
}

pub fn restart(mut user: ResMut<User>, mut event_reader: EventReader<Restart>) {
    for _ in event_reader.read() {
        *user = User::default();
    }
}
