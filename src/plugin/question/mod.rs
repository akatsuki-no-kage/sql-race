use bevy::app::{App, Plugin, Startup, Update};
use resource::{Config, Index};

pub mod component;
pub mod event;
pub mod resource;
pub mod system;

pub struct QuestionPlugin {
    pub question_count: usize,
}

impl Plugin for QuestionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<event::Next>()
            .add_event::<event::Reset>()
            .insert_resource(Config {
                question_count: self.question_count,
            })
            .init_resource::<Index>()
            .add_systems(Startup, system::init)
            .add_systems(Update, (system::next, system::reset));
    }
}
