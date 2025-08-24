use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct User {
    pub name: String,
    pub score: i64,
}
