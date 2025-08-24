use bevy::ecs::event::Event;

#[derive(Debug, Event)]
pub struct Start;

#[derive(Debug, Default, Event)]
pub struct Finish;
