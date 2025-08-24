use bevy::ecs::{
    event::EventReader,
    system::{Commands, NonSend},
};
use chrono::Local;

use super::{component::Score, event::Create, resource::Connection};

pub fn init(
    mut commands: Commands,
    conn: NonSend<Connection>,
) -> Result<(), bevy::prelude::BevyError> {
    let mut stmt = conn.prepare("SELECT * FROM scores ORDER BY score DESC")?;
    let scores = stmt.query_and_then((), |row| Score::try_from(row))?;

    for score in scores {
        commands.spawn(score?);
    }

    Ok(())
}

pub fn create(
    mut commands: Commands,
    mut event_reader: EventReader<Create>,
    conn: NonSend<Connection>,
) -> Result<(), bevy::prelude::BevyError> {
    for data in event_reader.read() {
        let score = Score {
            username: data.name.clone(),
            score: data.score,
            created_at: Local::now().naive_local(),
        };

        conn.execute(
            "INSERT INTO scores (username, score, created_at) VALUES (?, ?, ?)",
            (score.username.as_str(), score.score, score.created_at),
        )?;

        commands.spawn((score,));
    }

    Ok(())
}
