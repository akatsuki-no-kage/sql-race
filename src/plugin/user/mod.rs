pub mod event;
pub mod resource;
pub mod system;

use bevy::app::{App, Plugin, Update};
use resource::User;

pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<event::Change>()
            .add_event::<event::Restart>()
            .init_resource::<User>()
            .add_systems(Update, (system::change, system::restart));
    }
}
