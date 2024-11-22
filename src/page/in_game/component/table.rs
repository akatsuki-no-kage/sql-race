use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        self, Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        StatefulWidget, Widget,
    },
};
use sqlx::Row as _;
use widgetui::{Events, Res, ResMut, WidgetResult};

use crate::{
    page::in_game::InGameState,
    state::{GlobalState, Screen},
};

const ID: usize = 2;

pub struct Table<'a> {
    pub in_game_state: &'a InGameState,
}

impl Widget for Table<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &self.in_game_state.table_rows {
            Err(error) => {
                let paragraph = Paragraph::new(error.to_string())
                    .block(Block::default().title("Error").borders(Borders::ALL))
                    .style(Style::default().fg(Color::Red));
                paragraph.render(area, buf);
            }
            Ok(rows) => {
                let headers = &self.in_game_state.table_headers;
                let header_count = headers.len();

                let header_cells: Vec<_> = headers
                    .iter()
                    .map(|name| Cell::from(name.clone()).style(Style::default().fg(Color::Yellow)))
                    .collect();
                let header_row = Row::new(header_cells);

                let data_rows: Vec<_> = rows
                    .iter()
                    .map(|row| {
                        let cells: Vec<Cell> = (0..header_count)
                            .map(|column_id| {
                                let value = match row.try_get::<String, _>(column_id) {
                                    Ok(val) => val,
                                    Err(_) => match row.try_get::<i32, _>(column_id) {
                                        Ok(val) => val.to_string(),
                                        Err(_) => "NULL".to_string(),
                                    },
                                };
                                Cell::from(value)
                            })
                            .collect();
                        Row::new(cells)
                    })
                    .collect();

                let column_widths = vec![Constraint::Length(10); header_count];
                let table_block = if self.in_game_state.focused_element == ID {
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

                let table = widgets::Table::new(
                    vec![header_row].into_iter().chain(data_rows),
                    column_widths,
                )
                .block(table_block)
                .row_highlight_style(Style::default().fg(Color::White))
                .highlight_symbol(">> ");

                // Render the table with state
                StatefulWidget::render(
                    table,
                    area,
                    buf,
                    &mut self.in_game_state.table_state.clone(),
                );

                if rows.is_empty() {
                    return;
                }

                // Add scrollbar if there are rows
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

                let _ = self
                    .in_game_state
                    .table_scroll_state
                    .content_length(rows.len());

                StatefulWidget::render(
                    scrollbar,
                    scroll_area,
                    buf,
                    &mut self.in_game_state.table_scroll_state.clone(),
                );
            }
        }
    }
}

pub fn event_handler(
    events: Res<Events>,
    mut in_game_state: ResMut<InGameState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame || in_game_state.focused_element != ID {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        }) => {
            if let Ok(ref rows) = in_game_state.table_rows {
                let length = rows.len();
                let i = (in_game_state.table_state.selected().unwrap_or(length - 1) + 1) % length;
                in_game_state.table_state.select(Some(i));
                in_game_state.table_scroll_state = in_game_state.table_scroll_state.position(i);
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        }) => {
            if let Ok(ref rows) = in_game_state.table_rows {
                let length = rows.len();
                let i = (in_game_state.table_state.selected().unwrap_or(1) + length - 1) % length;
                in_game_state.table_state.select(Some(i));
                in_game_state.table_scroll_state = in_game_state.table_scroll_state.position(i);
            }
        }
        _ => {}
    }

    Ok(())
}
