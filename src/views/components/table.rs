use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::Paragraph;
use ratatui::widgets::{Block, Borders, Cell, Row as TuiRow, Table, TableState};
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};
use sqlx::sqlite::SqliteRow;
use sqlx::Column;
use sqlx::Row;

pub struct TableComponent {
    rows: Vec<SqliteRow>,
    headers: Vec<String>,
    error_message: Option<String>,
    pub is_focus: bool,
    state: TableState,
    scroll_state: ScrollbarState,
}

impl TableComponent {
    pub fn new(rows: Vec<SqliteRow>) -> Self {
        let headers = if let Some(row) = rows.first() {
            row.columns()
                .iter()
                .map(|col| col.name().to_string())
                .collect()
        } else {
            vec![]
        };

        Self {
            rows,
            headers,
            error_message: None,
            is_focus: false,
            state: TableState::default(),
            scroll_state: ScrollbarState::default(),
        }
    }

    pub fn with_error(error: String) -> Self {
        Self {
            rows: vec![],
            headers: vec!["Error".to_string()],
            error_message: Some(error),
            is_focus: false,
            state: TableState::default(),
            scroll_state: ScrollbarState::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.rows.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.rows.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl Widget for &TableComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(ref error) = self.error_message {
            let paragraph = Paragraph::new(error.clone())
                .block(Block::default().title("Error").borders(Borders::ALL))
                .style(Style::default().fg(Color::Red));
            paragraph.render(area, buf);
        } else {
            let header_cells: Vec<Cell> = self
                .headers
                .iter()
                .map(|h| Cell::from(h.clone()).style(Style::default().fg(Color::Yellow)))
                .collect();
            let header_row = TuiRow::new(header_cells);

            let data_rows: Vec<TuiRow> = self
                .rows
                .iter()
                .map(|row| {
                    let cells: Vec<Cell> = (0..self.headers.len())
                        .map(|col_idx| {
                            let value = match row.try_get::<String, _>(col_idx) {
                                Ok(val) => val,
                                Err(_) => match row.try_get::<i32, _>(col_idx) {
                                    Ok(val) => val.to_string(),
                                    Err(_) => "NULL".to_string(),
                                },
                            };
                            Cell::from(value)
                        })
                        .collect();
                    TuiRow::new(cells)
                })
                .collect();

            let column_widths = vec![Constraint::Length(10); self.headers.len()];
            let table_block = if self.is_focus {
                Block::default()
                    .title("Query Result (↑↓ to scroll)")
                    .borders(Borders::ALL)
                    .fg(Color::Green)
            } else {
                Block::default()
                    .title("Query Result")
                    .borders(Borders::ALL)
                    .fg(Color::White)
            };

            let table = Table::new(vec![header_row].into_iter().chain(data_rows), column_widths)
                .block(table_block)
                .row_highlight_style(Style::default().fg(Color::White))
                .highlight_symbol(">> ");

            // Render the table with state
            StatefulWidget::render(table, area, buf, &mut self.state.clone());

            // Add scrollbar if there are rows
            if !self.rows.is_empty() {
                let scrollbar = Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓"));

                let scroll_area = Rect {
                    x: area.right() - 1,
                    y: area.y + 1,
                    width: 1,
                    height: area.height - 2,
                };

                let mut scrollbar_state = self.scroll_state.clone();
                let _ = scrollbar_state.content_length(self.rows.len());

                StatefulWidget::render(scrollbar, scroll_area, buf, &mut scrollbar_state);
            }
        }
    }
}
