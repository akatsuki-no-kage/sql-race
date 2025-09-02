use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use tui_realm_stdlib::{Radio, Table};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent, State,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, Color, Style, TextSpan},
};

use crate::{app::Message, repository::question::TableInfo};

pub struct SchemaView {
    radio: Radio,
    table: Table,
    table_infos: Vec<TableInfo>,
}

fn update_table(info: TableInfo, table: &mut Table) {
    table.attr(
        Attribute::Title,
        AttrValue::Title((format!("Table: {}", info.name), Alignment::Center)),
    );

    let rows = info
        .columns
        .into_iter()
        .map(|column| {
            vec![
                TextSpan::new(column.name),
                TextSpan::new(column.is_primary_key.to_string()),
                TextSpan::new(column.data_type),
                TextSpan::new(column.is_nullable.to_string()),
                TextSpan::new(column.default_value.unwrap_or_default()),
            ]
        })
        .collect();
    table.attr(Attribute::Content, AttrValue::Table(rows));
}

impl SchemaView {
    pub fn new(table_infos: Vec<TableInfo>) -> Self {
        let table_names = table_infos.iter().map(|t| t.name.as_str());
        let radio = Radio::default()
            .borders(Borders::default().sides(BorderSides::all()))
            .title("Table names", Alignment::Center)
            .foreground(Color::Cyan)
            .inactive(Style::reset())
            .rewind(true)
            .choices(table_names);

        let mut table = Table::default()
            .borders(
                Borders::default()
                    .sides(BorderSides::all())
                    .color(Color::Green),
            )
            .inactive(Style::reset())
            .scroll(true)
            .step(5)
            .highlighted_color(Color::Cyan)
            .row_height(1)
            .rewind(true)
            .headers(["Name", "Primary key", "Type", "Nullable", "Default"]);
        update_table(table_infos[0].clone(), &mut table);

        Self {
            radio,
            table,
            table_infos,
        }
    }
}

impl MockComponent for SchemaView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

        self.radio.view(frame, chunks[0]);
        self.table.view(frame, chunks[1]);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.table.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.table.attr(attr, value)
    }

    fn state(&self) -> State {
        self.radio.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Left) | Cmd::Move(Direction::Right) => {
                let result = self.radio.perform(cmd);

                let selected_index = self.state().unwrap_one().unwrap_usize();
                update_table(self.table_infos[selected_index].clone(), &mut self.table);

                result
            }
            _ => self.table.perform(cmd),
        }
    }
}

impl Component<Message, NoUserEvent> for SchemaView {
    fn on(&mut self, event: Event<NoUserEvent>) -> Option<Message> {
        let cmd = match event {
            Event::Keyboard(KeyEvent {
                code: Key::Left | Key::BackTab,
                ..
            }) => Cmd::Move(Direction::Left),
            Event::Keyboard(KeyEvent {
                code: Key::Right | Key::Tab,
                ..
            }) => Cmd::Move(Direction::Right),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => Cmd::Scroll(Direction::Down),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => Cmd::Scroll(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => Cmd::GoTo(Position::Begin),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => Cmd::GoTo(Position::End),
            _ => Cmd::None,
        };

        match self.perform(cmd) {
            CmdResult::None => None,
            _ => Some(Message::None),
        }
    }
}
