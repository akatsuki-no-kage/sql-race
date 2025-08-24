use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use bevy::{ecs::resource::Resource, time::TimerMode};

#[derive(Debug, Resource)]
pub struct Timer(pub bevy::time::Timer);

impl Deref for Timer {
    type Target = bevy::time::Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Timer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self(bevy::time::Timer::new(duration, TimerMode::Once))
    }
}
