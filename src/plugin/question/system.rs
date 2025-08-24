use std::fs;

use bevy::ecs::{
    event::EventReader,
    system::{Commands, Res, ResMut},
};
use rand::seq::IteratorRandom;

use super::{
    component::{Answer, Question, Schema},
    event::{Next, Reset},
    resource::{Config, Index},
};

const QUESTION_PACK_DIR: &str = "questions";

pub fn init(mut commands: Commands, config: Res<Config>) -> bevy::prelude::Result {
    let mut question_dirs = fs::read_dir(QUESTION_PACK_DIR)?
        .filter_map(|x| x.ok().map(|x| x.path()))
        .choose_multiple(&mut rand::rng(), config.question_count);
    question_dirs.sort();

    for question_dir in question_dirs {
        let question = Question(fs::read_to_string(question_dir.join("question.txt"))?);
        let answer = Answer(fs::read_to_string(question_dir.join("answer.sql"))?);
        let schema = Schema::new(fs::read_to_string(question_dir.join("schema.sql"))?)?;

        commands.spawn((question, answer, schema));
    }

    Ok(())
}

pub fn next(mut index: ResMut<Index>, mut event_reader: EventReader<Next>, config: Res<Config>) {
    for _ in event_reader.read() {
        *index = Index((index.0 + 1) % config.question_count);
    }
}

pub fn reset(mut index: ResMut<Index>, mut event_read: EventReader<Reset>) {
    for _ in event_read.read() {
        *index = Index::default();
    }
}
