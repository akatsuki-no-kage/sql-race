use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Cell, Clear, Row as TuiRow, Table, Tabs};

use crate::models::schema::{QuestionRow, QuestionTable};

pub struct SchemaComponent {
    tables: Vec<QuestionTable>, // Each table corresponds to a tab
    selected_index: usize,      // Index of the currently selected tab
    is_focus: bool,
}

impl SchemaComponent {
    pub fn new(tables: Vec<QuestionTable>) -> Self {
        Self {
            tables,
            selected_index: 0,
            is_focus: false,
        }
    }

    // Navigate to the next tab
    pub fn next_tab(&mut self) {
        if !self.tables.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.tables.len();
        }
    }

    // Navigate to the previous tab
    pub fn previous_tab(&mut self) {
        if !self.tables.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.tables.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.is_focus = focus;
    }
}

impl Widget for &SchemaComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.tables.is_empty() {
            return; // No tables to render
        }

        Clear.render(area, buf);

        // Define the layout to separate the tabs from the table display
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        // Get tab labels for each table
        let tab_labels: Vec<_> = self.tables.iter().map(|table| table.name.clone()).collect();

        // Define styling for the tabs to show focus and selected tab
        let tabs = Tabs::new(
            tab_labels
                .into_iter()
                .map(|title| title.into())
                .collect::<Vec<String>>(),
        )
        .select(self.selected_index)
        .block(Block::default().title("Tables").borders(Borders::ALL))
        .highlight_style(if self.is_focus {
            Style::default().fg(Color::Green).bg(Color::Black)
        } else {
            Style::default().fg(Color::Gray).bg(Color::Black)
        });

        // Render the tabs at the top area
        tabs.render(layout[0], buf);

        // Get the selected table based on the current tab index
        let current_table = &self.tables[self.selected_index];

        // Define header row for the table columns
        let headers = vec!["ID", "Name", "Type", "Not Null", "Default", "PK"];
        let header_cells: Vec<Cell> = headers
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)))
            .collect();
        let header_row = TuiRow::new(header_cells);

        // Define rows based on QuestionRow data within the selected QuestionTable
        let data_rows: Vec<TuiRow> = current_table
            .rows
            .iter()
            .map(|row| {
                let cells = vec![
                    Cell::from(row.col_id.to_string()),
                    Cell::from(row.name.clone()),
                    Cell::from(row.data_type.clone()),
                    Cell::from(row.not_null.to_string()),
                    Cell::from(
                        row.default_value
                            .clone()
                            .unwrap_or_else(|| "NULL".to_string()),
                    ),
                    Cell::from(row.primary_key.to_string()),
                ];
                TuiRow::new(cells)
            })
            .collect();

        // Define table block with focus styling
        let table_block = Block::default()
            .title(format!("Schema: {}", current_table.name))
            .borders(Borders::ALL)
            .border_style(if self.is_focus {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Gray)
            });

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
