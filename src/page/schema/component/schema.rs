use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Clear, Row, Table, Tabs, Widget},
};
use widgetui::{Chunks, Events, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::{
    page::in_game::component::question,
    state::{GlobalState, Screen},
};

pub struct Chunk;

#[derive(Default, State)]
pub struct CustomState {
    pub selected_schema: usize,
}

impl CustomState {
    fn next(&mut self, schema_count: usize) {
        self.selected_schema = (self.selected_schema + 1) % schema_count;
    }

    fn prev(&mut self, schema_count: usize) {
        self.selected_schema = (self.selected_schema + schema_count - 1) % schema_count;
    }
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<CustomState>,
    question_state: Res<question::CustomState>,
) -> WidgetResult {
    let Ok(chunk) = chunks.get_chunk::<Chunk>() else {
        return Ok(());
    };

    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunk);

    let schemas = &question_state.questions[question_state.selected_question].schemas;
    // TODO: dont return early
    if schemas.is_empty() {
        return Ok(());
    }

    Clear.render(frame.size(), frame.buffer_mut());

    // Get tab labels for each table
    let tab_labels: Vec<_> = schemas.iter().map(|table| table.name.clone()).collect();

    // Define styling for the tabs to show focus and selected tab
    let tabs = Tabs::new(
        tab_labels
            .into_iter()
            .map(|title| title.into())
            .collect::<Vec<String>>(),
    )
    .select(state.selected_schema)
    .block(Block::default().title("Tables").borders(Borders::ALL))
    .highlight_style(Style::default().fg(Color::Gray).bg(Color::Black));

    // Render the tabs at the top area
    frame.render_widget(tabs, sub_chunks[0]);

    // Define header row for the table columns
    let headers = vec!["ID", "Name", "Type", "Not Null", "Default", "PK"];
    let header_cells: Vec<_> = headers
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)))
        .collect();
    let header_row = Row::new(header_cells);

    let schema = &schemas[state.selected_schema];

    // Define rows based on QuestionRow data within the selected QuestionTable
    let data_rows: Vec<_> = schema
        .columns
        .iter()
        .map(|row| {
            let cells = vec![
                Cell::from(row.id.to_string()),
                Cell::from(row.name.clone()),
                Cell::from(row.data_type.clone()),
                Cell::from(row.is_nullable.to_string()),
                Cell::from(
                    row.default_value
                        .clone()
                        .unwrap_or_else(|| "NULL".to_string()),
                ),
                Cell::from(row.primary_key.to_string()),
            ];
            Row::new(cells)
        })
        .collect();

    // Define table block with focus styling
    let table_block = Block::default()
        .title(format!("Schema: {}", schema.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));

    // Set column widths
    let column_widths = vec![
        Constraint::Length(5),
        Constraint::Length(15),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(15),
        Constraint::Length(5),
    ];

    // Render the table for the currently selected tab in the lower layout area
    let schema_table = Table::new(vec![header_row].into_iter().chain(data_rows), column_widths)
        .block(table_block)
        .row_highlight_style(Style::default().fg(Color::Green));

    frame.render_widget(schema_table, sub_chunks[1]);

    Ok(())
}

pub fn event_handler(
    events: Res<Events>,
    mut state: ResMut<CustomState>,
    question_state: Res<question::CustomState>,
    mut global_state: ResMut<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::Schema {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    let schema_count = question_state.questions[question_state.selected_question]
        .schemas
        .len();

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => global_state.screen = Screen::InGame,
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            ..
        }) => state.next(schema_count),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            ..
        }) => state.prev(schema_count),
        _ => {}
    }

    Ok(())
}
