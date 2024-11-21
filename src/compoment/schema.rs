use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Clear, Row, Table, Tabs, Widget},
};

use crate::model;

#[derive(Default)]
pub struct Schema<'a> {
    schemas: &'a [model::Schema],
    selected_index: usize,

    border_color: Color,
}

impl<'a> Schema<'a> {
    pub fn schemas(mut self, schemas: &'a [model::Schema]) -> Self {
        self.schemas = schemas;
        self
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
}

impl Widget for Schema<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.schemas.is_empty() {
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
        .select(self.selected_index)
        .block(Block::default().title("Tables").borders(Borders::ALL))
        .highlight_style(Style::default().fg(self.border_color).bg(Color::Black));

        // Render the tabs at the top area
        tabs.render(layout[0], buf);

        // Get the selected table based on the current tab index
        let current_table = &self.schemas[self.selected_index.min(self.schemas.len() - 1)];

        // Define header row for the table columns
        let headers = ["ID", "Name", "Type", "Not Null", "Default", "PK"];
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
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.border_color));

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
