use bevy::ecs::component::Component;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub is_primary_key: bool,
    pub data_type: String,
    pub is_nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Debug)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<Column>,
}

impl TableInfo {
    fn new(name: String, conn: &Connection) -> rusqlite::Result<Self> {
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", &name))?;
        let columns = stmt
            .query_and_then((), |raw| {
                Ok::<_, rusqlite::Error>(Column {
                    name: raw.get("name")?,
                    is_primary_key: raw.get("pk")?,
                    data_type: raw.get("type")?,
                    is_nullable: raw.get("notnull")?,
                    default_value: raw.get("dflt_value")?,
                })
            })?
            .collect::<Result<_, _>>()?;

        Ok(Self { name, columns })
    }
}

#[derive(Debug, Component)]
pub struct Schema {
    pub raw: String,
    pub table_infos: Vec<TableInfo>,
}

impl Schema {
    pub fn new(raw: String) -> rusqlite::Result<Self> {
        let conn = Connection::open_in_memory()?;

        let mut stmt = conn.prepare("SELECT name FROM sqlite_schema WHERE type = 'table'")?;
        let table_infos = stmt
            .query_and_then((), |raw| {
                raw.get::<_, String>("name")
                    .and_then(|name| TableInfo::new(name, &conn))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { raw, table_infos })
    }
}
