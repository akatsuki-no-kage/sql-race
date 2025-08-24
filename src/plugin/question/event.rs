use bevy::ecs::event::Event;

#[derive(Debug, Event)]
pub struct Next;

#[derive(Debug, Event)]
pub struct Reset;
