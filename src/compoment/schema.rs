use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Clear, Row, Table, Tabs, Widget},
};
use widgetui::State;

use crate::model;

#[derive(State)]
pub struct SchemaState {
    pub schemas: Vec<model::Schema>,
    pub selected_index: usize,
}

pub struct Schema<'a> {
    state: &'a SchemaState,
}

impl Widget for Schema<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.state.schemas.is_empty() {
            return; // No tables to render
        }

        Clear.render(area, buf);

        // Define the layout to separate the tabs from the table display
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        // Get tab labels for each table
        let tab_labels: Vec<_> = self
            .state
            .schemas
            .iter()
            .map(|table| table.name.clone())
            .collect();

        // Define styling for the tabs to show focus and selected tab
        let tabs = Tabs::new(
            tab_labels
                .into_iter()
                .map(|title| title.into())
                .collect::<Vec<String>>(),
        )
        .select(self.state.selected_index)
        .block(Block::default().title("Tables").borders(Borders::ALL));
        // .highlight_style(if self.state.is_focus {
        //     Style::default().fg(Color::Green).bg(Color::Black)
        // } else {
        //     Style::default().fg(Color::Gray).bg(Color::Black)
        // });

        // Render the tabs at the top area
        tabs.render(layout[0], buf);

        // Get the selected table based on the current tab index
        let current_table = &self.state.schemas[self.state.selected_index];

        // Define header row for the table columns
        let headers = vec!["ID", "Name", "Type", "Not Null", "Default", "PK"];
        let header_cells: Vec<Cell> = headers
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)))
            .collect();
        let header_row = Row::new(header_cells);

        // Define rows based on QuestionRow data within the selected QuestionTable
        let data_rows: Vec<_> = current_table
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
            .title(format!("Schema: {}", current_table.name))
            .borders(Borders::ALL);
        // .border_style(if self.is_focus {
        //     Style::default().fg(Color::Green)
        // } else {
        //     Style::default().fg(Color::Gray)
        // });

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
