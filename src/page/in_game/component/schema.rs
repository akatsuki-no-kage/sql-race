use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Clear, Row, Table, Tabs, Widget},
};
use widgetui::{Events, Res, ResMut, WidgetResult};

use crate::{
    page::in_game::InGameState,
    state::{GlobalState, Screen},
};

pub struct Schema<'a> {
    pub in_game_state: &'a InGameState,
}

impl Widget for Schema<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let schemas = &self.in_game_state.questions[self.in_game_state.question_index].schemas;
        if schemas.is_empty() {
            return;
        }

        let popup_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(area)[1]; // Centered vertically
        let popup_area_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(popup_area)[1]; // Centered horizontally
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(popup_area_horizontal);

        Clear.render(area, buf);

        // Get tab labels for each table
        let tab_labels: Vec<_> = schemas.iter().map(|table| table.name.clone()).collect();

        // Define styling for the tabs to show focus and selected tab
        let tabs = Tabs::new(
            tab_labels
                .into_iter()
                .map(|title| title.into())
                .collect::<Vec<String>>(),
        )
        .select(self.in_game_state.schema_index)
        .block(Block::default().title("Tables").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Gray).bg(Color::Black));

        // Render the tabs at the top area
        tabs.render(layout[0], buf);

        // Define header row for the table columns
        let headers = vec!["ID", "Name", "Type", "Not Null", "Default", "PK"];
        let header_cells: Vec<_> = headers
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)))
            .collect();
        let header_row = Row::new(header_cells);

        let schema = &schemas[self.in_game_state.schema_index];

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

        schema_table.render(layout[1], buf);
    }
}

pub fn event_handler(
    events: Res<Events>,
    mut in_game_state: ResMut<InGameState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame || !in_game_state.is_popup_visible {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => in_game_state.is_popup_visible = false,
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            ..
        }) => in_game_state.next_schema(),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            ..
        }) => in_game_state.previous_schema(),
        _ => {}
    }

    Ok(())
}
