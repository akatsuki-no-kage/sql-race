use bevy::ecs::component::Component;

#[derive(Debug, Component)]
pub struct Question(pub String);
