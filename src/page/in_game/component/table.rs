use anyhow::Result;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        self, Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, StatefulWidget, TableState,
    },
};
use sqlx::{sqlite::SqliteRow, Row as _};
use widgetui::{Chunks, Events, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::{
    page::in_game::FocusState,
    state::{GlobalState, Screen},
};

const ID: usize = 2;

pub struct Chunk;

#[derive(State)]
pub struct CustomState {
    table_headers: Vec<String>,
    table_rows: Result<Vec<SqliteRow>>,
    inner_state: TableState,
    table_scroll_state: ScrollbarState,
}

impl Default for CustomState {
    fn default() -> Self {
        Self {
            table_headers: Default::default(),
            table_rows: Ok(Default::default()),
            inner_state: Default::default(),
            table_scroll_state: Default::default(),
        }
    }
}

impl CustomState {
    fn _next(&self) -> Option<usize> {
        self.table_rows
            .as_ref()
            .map(|rows| (self.inner_state.selected().unwrap_or(0) + 1) % rows.len())
            .ok()
    }
    fn next(&mut self) {
        if let Some(i) = self._next() {
            self.inner_state.select(Some(i));
            self.table_scroll_state = self.table_scroll_state.position(i);
        }
    }

    fn _prev(&self) -> Option<usize> {
        self.table_rows
            .as_ref()
            .map(|rows| {
                let length = rows.len();
                (self.inner_state.selected().unwrap_or(1) + length - 1) % length
            })
            .ok()
    }

    fn prev(&mut self) {
        if let Some(i) = self._prev() {
            self.inner_state.select(Some(i));
            self.table_scroll_state = self.table_scroll_state.position(i);
        }
    }
}

fn render_error(error: String, frame: &mut WidgetFrame, chunk: Rect) -> WidgetResult {
    let error = Paragraph::new(error)
        .block(Block::default().title("Error").borders(Borders::ALL))
        .style(Style::default().fg(Color::Red));
    frame.render_widget(error, chunk);

    Ok(())
}

fn render_table(
    rows: &[SqliteRow],
    frame: &mut WidgetFrame,
    chunk: Rect,
    state: &CustomState,
    focus_state: &FocusState,
) -> WidgetResult {
    let headers = &state.table_headers;
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
    let table_block = if focus_state.focused_element == ID {
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

    let table = widgets::Table::new(vec![header_row].into_iter().chain(data_rows), column_widths)
        .block(table_block)
        .row_highlight_style(Style::default().fg(Color::White))
        .highlight_symbol(">> ");

    StatefulWidget::render(
        table,
        chunk,
        frame.buffer_mut(),
        &mut state.inner_state.clone(),
    );

    if rows.is_empty() {
        return Ok(());
    }

    // Add scrollbar if there are rows
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let scroll_area = Rect {
        x: chunk.right() - 1,
        y: chunk.y + 1,
        width: 1,
        height: chunk.height - 2,
    };

    StatefulWidget::render(
        scrollbar,
        scroll_area,
        frame.buffer_mut(),
        &mut state.table_scroll_state.clone(),
    );

    Ok(())
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<CustomState>,
    focus_state: Res<FocusState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
        return Ok(());
    }

    let chunk = chunks.get_chunk::<Chunk>()?;

    match &state.table_rows {
        Err(error) => render_error(error.to_string(), &mut frame, chunk),
        Ok(rows) => render_table(rows, &mut frame, chunk, &state, &focus_state),
    }
}

pub fn event_handler(
    events: Res<Events>,
    mut state: ResMut<CustomState>,
    focus_state: Res<FocusState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame || focus_state.focused_element != ID {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        }) => state.next(),
        Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        }) => state.prev(),
        _ => {}
    }

    Ok(())
}
