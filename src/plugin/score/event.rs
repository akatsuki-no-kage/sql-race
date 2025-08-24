use bevy::ecs::event::Event;

#[derive(Debug, Event)]
pub struct Create {
    pub name: String,
    pub score: i64,
}
