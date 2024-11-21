use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        self, Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, StatefulWidget, Widget,
    },
};
use sqlx::{sqlite::SqliteRow, Row as _};
use widgetui::{ResMut, State};

pub struct Table<'a> {
    pub headers: &'a [String],
    pub rows: &'a [SqliteRow],
    pub select_state: &'a widgets::TableState,
    pub scroll_state: &'a ScrollbarState,
    pub error: Option<String>,
}

impl Widget for Table<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        // if let Some(ref error) = self.state.error {
        //     let paragraph = Paragraph::new(error.clone())
        //         .block(Block::default().title("Error").borders(Borders::ALL))
        //         .style(Style::default().fg(Color::Red));
        //     paragraph.render(area, buf);
        //     return;
        // }
        //
        // let header_cells: Vec<_> = self
        //     .state
        //     .headers
        //     .iter()
        //     .map(|h| Cell::from(h.clone()).style(Style::default().fg(Color::Yellow)))
        //     .collect();
        // let header_row = Row::new(header_cells);
        //
        // let data_rows: Vec<Row> = self
        //     .state
        //     .rows
        //     .iter()
        //     .map(|row| {
        //         let cells: Vec<_> = (0..self.state.headers.len())
        //             .map(|col_idx| {
        //                 let value = match row.try_get::<String, _>(col_idx) {
        //                     Ok(val) => val,
        //                     Err(_) => match row.try_get::<i32, _>(col_idx) {
        //                         Ok(val) => val.to_string(),
        //                         Err(_) => "NULL".to_string(),
        //                     },
        //                 };
        //                 Cell::from(value)
        //             })
        //             .collect();
        //         Row::new(cells)
        //     })
        //     .collect();
        //
        // let column_widths = vec![Constraint::Length(10); self.state.headers.len()];
        // let table_block = Block::default()
        //     .title("Query Result (↑↓ to scroll)")
        //     .borders(Borders::ALL)
        //     .fg(Color::Green);
        //
        // let table =
        //     widgets::Table::new(vec![header_row].into_iter().chain(data_rows), column_widths)
        //         .block(table_block)
        //         .row_highlight_style(Style::default().fg(Color::White))
        //         .highlight_symbol(">> ");
        //
        // // Render the table with state
        // StatefulWidget::render(table, area, buf, &mut self.state.select_state);
        //
        // // Add scrollbar if there are rows
        // if !self.state.rows.is_empty() {
        //     let scrollbar = Scrollbar::default()
        //         .orientation(ScrollbarOrientation::VerticalRight)
        //         .begin_symbol(Some("↑"))
        //         .end_symbol(Some("↓"));
        //
        //     let scroll_area = Rect {
        //         x: area.right() - 1,
        //         y: area.y + 1,
        //         width: 1,
        //         height: area.height - 2,
        //     };
        //
        //     let mut scrollbar_state = self.state.scroll_state.clone();
        //     let _ = scrollbar_state.content_length(self.state.rows.len());
        //
        //     StatefulWidget::render(scrollbar, scroll_area, buf, &mut scrollbar_state);
        // }
    }
}
