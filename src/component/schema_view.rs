use ratatui::{
    Frame,
    layout::{Constraint, Rect},
};
use tui_realm_stdlib::{Container, Radio, Table};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent, State,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, Color, Layout, TextSpan},
};

use crate::{app::Message, repository::question::TableInfo};

pub struct SchemaView {
    component: Container,
    table_infos: Vec<TableInfo>,
}

fn update_table(info: TableInfo, table: &mut dyn MockComponent) {
    table.attr(
        Attribute::Title,
        AttrValue::Title((info.name, Alignment::Center)),
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
        let radio_widget = Radio::default()
            .borders(Borders::default().sides(BorderSides::all()))
            .title("Table names", Alignment::Center)
            .rewind(true)
            .choices(table_names);

        let table_widget = Table::default()
            .borders(Borders::default().sides(BorderSides::all()))
            .scroll(true)
            .step(5)
            .highlighted_color(Color::Cyan)
            .row_height(1)
            .headers(["Name", "Primary key", "Type", "Nullable", "Default"]);

        let mut container = Container::default()
            .layout(
                Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints(&[Constraint::Length(3), Constraint::Min(0)]),
            )
            .children(vec![Box::new(radio_widget), Box::new(table_widget)]);
        update_table(table_infos[0].clone(), container.children[1].as_mut());

        Self {
            component: container,
            table_infos,
        }
    }
}

impl MockComponent for SchemaView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.component.view(frame, area)
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value)
    }

    fn state(&self) -> State {
        self.component.children[0].state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Left) | Cmd::Move(Direction::Right) => {
                let selected_index = self.state().unwrap_one().unwrap_usize();
                update_table(
                    self.table_infos[selected_index].clone(),
                    self.component.children[1].as_mut(),
                );
                self.component.children[0].perform(cmd)
            }
            _ => self.component.children[1].perform(cmd),
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

        self.perform(cmd);

        None
    }
}
