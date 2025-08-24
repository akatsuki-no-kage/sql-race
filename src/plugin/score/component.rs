use bevy::ecs::component::Component;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, PartialOrd, Component)]
pub struct Score {
    pub username: String,
    pub score: i64,
    pub created_at: NaiveDateTime,
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for Score {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row<'a>) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            username: row.get("username")?,
            score: row.get("score")?,
            created_at: row.get("created_at")?,
        })
    }
}
