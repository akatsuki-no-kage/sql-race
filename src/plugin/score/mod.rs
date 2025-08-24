pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::app::{App, Plugin, Startup, Update};
use event::Create;
use resource::Connection;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Create>()
            .init_non_send_resource::<Connection>()
            .add_systems(Startup, system::init)
            .add_systems(Update, system::create);
    }
}
