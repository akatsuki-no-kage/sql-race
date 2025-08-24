use bevy::ecs::resource::Resource;

#[derive(Debug, Resource)]
pub struct Config {
    pub question_count: usize,
}

#[derive(Debug, Default, Resource)]
pub struct Index(pub usize);
