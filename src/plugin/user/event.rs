use bevy::ecs::event::Event;

#[derive(Debug, Event)]
pub struct Change {
    pub name: String,
}

#[derive(Debug, Event)]
pub struct Restart;
