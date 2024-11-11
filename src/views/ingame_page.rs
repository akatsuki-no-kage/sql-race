use crate::app::App;
use crate::controllers::{check_exist_username, get_question, get_score, run_query, view_schemas};
use crate::models::schema::QuestionTable;
use crate::models::score::Score;
use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Cell, Clear, List, ListItem, Row, Table};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::Widget,
    widgets::{Block, Borders, Paragraph},
};
use sqlx::{Column, Row as SqlRow, SqlitePool};
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct InGamePage {
    score: i64,
    time_left: f32,
    popup_visible: bool,
    tables_info: Vec<QuestionTable>,
    tab_idx: usize,
    last_instant: Instant,
    input: String,
    cursor_position: usize,
    selected_block: usize,
    question: String,
    question_idx: u8,
    selected_option: usize,
    options: Vec<String>,
    result: String,
    current_is_done: bool,
    text_selected: bool,
    clipboard: String,
}

impl InGamePage {
    pub fn next_tab(&mut self) {
        if self.tab_idx == self.tables_info.len() - 1 {
            self.tab_idx = 0;
        } else {
            self.tab_idx += 1;
        }
    }

    pub fn previous_tab(&mut self) {
        if self.tab_idx == 0 {
            self.tab_idx = self.tables_info.len() - 1;
        } else {
            self.tab_idx -= 1;
        }
    }

    pub fn next_option(&mut self) {
        if self.selected_option == 0 {
            self.selected_option = self.options.len() - 1;
        } else {
            self.selected_option -= 1;
        }
    }

    pub fn previous_option(&mut self) {
        self.selected_option = (self.selected_option + 1) % self.options.len();
    }

    pub fn next_block(&mut self) {
        self.selected_block = (self.selected_block + 1) % 4;
    }

    pub fn previous_block(&mut self) {
        if self.selected_block == 0 {
            self.selected_block = 3;
        } else {
            self.selected_block -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        let current_line_start = self.input[..self.cursor_position]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);

        if let Some(prev_line_start) = self.input[..current_line_start.saturating_sub(1)]
            .rfind('\n')
            .map(|pos| pos + 1)
        {
            let current_column = self.cursor_position - current_line_start;
            let prev_line_length = current_line_start - prev_line_start - 1;
            self.cursor_position = prev_line_start + current_column.min(prev_line_length);
        }
    }

    pub fn move_cursor_down(&mut self) {
        if let Some(current_line_end) = self.input[self.cursor_position..].find('\n') {
            let next_line_start = self.cursor_position + current_line_end + 1;
            if next_line_start < self.input.len() {
                let current_line_start = self.input[..self.cursor_position]
                    .rfind('\n')
                    .map(|pos| pos + 1)
                    .unwrap_or(0);
                let current_column = self.cursor_position - current_line_start;

                let next_line_length = self.input[next_line_start..]
                    .find('\n')
                    .unwrap_or(self.input.len() - next_line_start);

                self.cursor_position = next_line_start + current_column.min(next_line_length);
            }
        }
    }

    pub fn select_all(&mut self) {
        self.text_selected = true;
    }

    pub fn copy_selected(&mut self) {
        if self.text_selected {
            self.clipboard = self.input.clone();
        }
    }

    pub fn paste(&mut self) {
        if !self.clipboard.is_empty() {
            if self.text_selected {
                self.input = self.clipboard.clone();
                self.cursor_position = self.input.len();
                self.text_selected = false;
            } else {
                self.input.insert_str(self.cursor_position, &self.clipboard);
                self.cursor_position += self.clipboard.len();
            }
        }
    }

    pub fn delete_selected(&mut self) {
        if self.text_selected {
            self.input.clear();
            self.cursor_position = 0;
            self.text_selected = false;
        }
    }

    pub fn update_states(&mut self) {
        if self.last_instant.elapsed() >= Duration::from_secs(1) {
            self.time_left -= 1.0;
            self.last_instant = Instant::now();
        }

        if self.tab_idx >= self.tables_info.len() {
            self.tab_idx = 0;
        }
    }

    pub async fn handle_key_events(&mut self, app: &mut App, db: &SqlitePool) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                        app.exit = true;
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('a')) => {
                        if self.selected_block == 0 {
                            self.select_all();
                        }
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        if self.selected_block == 0 {
                            self.copy_selected();
                        }
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('v')) => {
                        if self.selected_block == 0 {
                            self.paste();
                        }
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                        self.run_query().await?;
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                        self.view_schema(db).await?;
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                        let question = get_question(Path::new(&format!(
                            "./questions/question-{}",
                            self.question_idx
                        )))
                        .await?;

                        if get_score(&self.input, &question.answer, &question.schema).await? {
                            self.score += self.question_idx as i64 * 10;
                            println!("{:?}", self.score);
                            if check_exist_username(&app.pool, app.username.clone()).await? {
                                Score::update_score(&app.pool, app.username.clone(), self.score)
                                    .await?;
                                self.current_is_done = true;
                            };
                        }
                    }
                    (KeyModifiers::NONE, KeyCode::Left) => {
                        if self.popup_visible {
                            self.previous_tab();
                        } else if self.selected_block == 0 {
                            self.move_cursor_left();
                        }
                    }
                    (KeyModifiers::CONTROL, KeyCode::Left) => {
                        self.previous_block();
                    }
                    (KeyModifiers::NONE, KeyCode::Right) => {
                        if self.popup_visible {
                            self.next_tab();
                        } else if self.selected_block == 0 {
                            self.move_cursor_right();
                        }
                    }
                    (KeyModifiers::CONTROL, KeyCode::Right) => {
                        self.next_block();
                    }
                    (_, KeyCode::Up) => {
                        if self.selected_block == 3 {
                            self.next_option();
                        } else if self.selected_block == 0 {
                            self.move_cursor_up();
                        }
                    }
                    (_, KeyCode::Down) => {
                        if self.selected_block == 3 {
                            self.previous_option();
                        } else if self.selected_block == 0 {
                            self.move_cursor_down();
                        }
                    }
                    (_, KeyCode::Char(c)) => {
                        if self.selected_block == 0 {
                            if self.text_selected {
                                self.input.clear();
                                self.text_selected = false;
                            }
                            self.input.insert(self.cursor_position, c);
                            self.cursor_position += 1;
                        }
                    }
                    (_, KeyCode::Backspace) => {
                        if self.selected_block == 0 {
                            if self.text_selected {
                                self.delete_selected();
                            } else if self.cursor_position > 0 {
                                self.input.remove(self.cursor_position - 1);
                                self.cursor_position -= 1;
                            }
                        }
                    }
                    (_, KeyCode::Delete) => {
                        if self.selected_block == 0 {
                            if self.text_selected {
                                self.delete_selected();
                            } else if self.cursor_position < self.input.len() {
                                self.input.remove(self.cursor_position);
                            }
                        }
                    }
                    (_, KeyCode::Enter) => {
                        if self.selected_block == 3 && self.selected_option == 1 {
                            self.view_schema(db).await?;
                        } else if self.selected_block == 3 && self.selected_option == 0 {
                            self.run_query().await?;
                        } else if self.selected_block == 3 && self.selected_option == 2 {
                            let question = get_question(Path::new(&format!(
                                "./questions/question-{}",
                                self.question_idx
                            )))
                            .await?;

                            if get_score(&self.input, &question.answer, &question.schema).await? {
                                app.score += self.question_idx as i64 * 10;
                                Score::update_score(db, app.username.clone(), app.score).await?;
                                self.score = app.score;
                                self.current_is_done = true;
                            }
                        } else if self.selected_block == 3 && self.selected_option == 3 {
                            app.exit = true;
                        } else if self.selected_block == 0 {
                            self.input.insert(self.cursor_position, '\n');
                            self.cursor_position += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    async fn view_schema(&mut self, db: &SqlitePool) -> Result<()> {
        if !self.popup_visible {
            self.tables_info = view_schemas(&db).await?;
            self.popup_visible = true;
        } else {
            self.popup_visible = false;
        }
        Ok(())
    }

    async fn run_query(&mut self) -> Result<()> {
        let schema = get_question(Path::new(&format!(
            "./questions/question-{}",
            self.question_idx
        )))
        .await?
        .schema;
        self.result = match run_query(&self.input, &schema).await {
            Ok(rows) => {
                if let Some(first_row) = rows.first() {
                    let num_columns = first_row.len();
                    let mut column_widths = vec![0; num_columns];
                    let column_names: Vec<String> = first_row
                        .columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect();

                    for row in &rows {
                        for col_idx in 0..num_columns {
                            let value = match row.try_get::<String, _>(col_idx) {
                                Ok(val) => val,
                                Err(_) => match row.try_get::<i32, _>(col_idx) {
                                    Ok(val) => val.to_string(),
                                    Err(_) => "NULL".to_string(),
                                },
                            };
                            column_widths[col_idx] = column_widths[col_idx].max(value.len());
                        }
                    }

                    let mut result_string = String::new();

                    for (i, width) in column_widths.iter().enumerate() {
                        let header_name = &column_names[i];
                        result_string.push_str(&format!(
                            "{:<width$} | ",
                            header_name,
                            width = width
                        ));
                    }
                    result_string.push_str("\n");
                    result_string.push_str(&"-".repeat(result_string.len()));
                    result_string.push_str("\n");

                    for row in rows {
                        for (col_idx, width) in column_widths.iter().enumerate() {
                            let value = match row.try_get::<String, _>(col_idx) {
                                Ok(val) => val,
                                Err(_) => match row.try_get::<i32, _>(col_idx) {
                                    Ok(val) => val.to_string(),
                                    Err(_) => "NULL".to_string(),
                                },
                            };
                            result_string.push_str(&format!("{:<width$} | ", value, width = width));
                        }
                        result_string.push_str("\n");
                    }

                    result_string
                } else {
                    "No rows returned".to_string()
                }
            }
            Err(err) => format!("Query failed: {:?}", err.to_string()),
        };
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            selected_block: 0,
            input: String::new(),
            cursor_position: 0,
            time_left: 30.0,
            popup_visible: false,
            tables_info: vec![],
            tab_idx: 0,
            last_instant: Instant::now(),
            question: String::new(),
            question_idx: 0,
            options: vec![
                "Run (Ctrl + R)".to_string(),
                "View Schema (Ctrl + H)".to_string(),
                "Submit (Ctrl + S)".to_string(),
                "Exit (Ctrl + Q)".to_string(),
            ],
            selected_option: 0,
            score: 0,
            result: "None".to_string(),
            current_is_done: true,
            text_selected: false,
            clipboard: String::new(),
        }
    }

    pub async fn update_question(&mut self) -> Result<()> {
        if self.current_is_done {
            self.question_idx += 1;
            let current_question_path = format!("./questions/question-{}", self.question_idx);
            let question = get_question(Path::new(&current_question_path)).await?;
            self.question = question.question;
            self.current_is_done = false;
        }

        Ok(())
    }
}

impl Widget for &InGamePage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(7),
                    Constraint::Percentage(68),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(area);
        let time_and_score_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(5),
                    Constraint::Percentage(10),
                    Constraint::Percentage(85),
                ]
                .as_ref(),
            )
            .split(main_area[0]);
        let query_and_question_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(main_area[1]);
        let result_and_features_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(main_area[2]);

        let chunks = vec![
            query_and_question_area[0],
            query_and_question_area[1],
            result_and_features_area[0],
        ];
        let blocks = vec![
            ("Query".to_string(), self.input.as_str(), Color::Red),
            ("Question".to_string(), self.question.as_str(), Color::Green),
            ("Result".to_string(), self.result.as_str(), Color::Blue),
        ];

        for (i, (chunk, (title, content, color))) in chunks.iter().zip(blocks.iter()).enumerate() {
            let mut display_content = content.to_string();
            if i == 0 && self.selected_block == 0 {
                // Insert cursor for the Query block when it's selected
                if self.text_selected {
                    display_content = format!("\x1b[7m{}\x1b[0m", display_content);
                } else {
                    display_content.insert(self.cursor_position, '|');
                }
            }

            let block = Block::default()
                .title(title.as_str())
                .borders(Borders::ALL)
                .border_style(if self.selected_block == i {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                });
            let paragraph = Paragraph::new(display_content)
                .block(block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            paragraph.render(*chunk, buf);
        }

        let items: Vec<ListItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_option {
                    Style::default().bg(Color::Green)
                } else {
                    Style::default()
                };
                ListItem::new(option.to_string()).style(style)
            })
            .collect();
        let list = List::new(items).block(
            Block::default()
                .title("Options")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if self.selected_block != 3 {
                    Color::White
                } else {
                    Color::Green
                })),
        );
        list.render(result_and_features_area[1], buf);

        let block_score = Block::default().title("Score").borders(Borders::ALL);
        Paragraph::new(self.score.to_string())
            .centered()
            .block(block_score)
            .render(time_and_score_area[0], buf);

        let block_time_left = Block::default().title("Time left").borders(Borders::ALL);
        let block_hotkey_guide = Block::default().borders(Borders::ALL);

        Paragraph::new(self.time_left.to_string())
            .centered()
            .block(block_time_left)
            .render(time_and_score_area[1], buf);

        let hotkey_guide = Text::from(vec![Line::from(vec![
            "Ctrl+A: Select All / Ctrl + R: Run Query / Enter: Choose / \u{2192}, \u{2190}: Move / Esc: Menu"
                .green()
                .into(),
        ])]);
        Paragraph::new(hotkey_guide)
            .centered()
            .block(block_hotkey_guide)
            .render(time_and_score_area[2], buf);

        if self.popup_visible {
            let selected_table = &self.tables_info[self.tab_idx];
            let centered_popup_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ])
                .split(area);

            let popup_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Percentage(90)])
                .split(centered_popup_area[1]);

            let tab_area = popup_area[0];
            let tab_width = tab_area.width / self.tables_info.len() as u16;

            let tab_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(tab_width); self.tables_info.len()])
                .split(tab_area);

            for (i, (tab_chunk, table)) in
                tab_chunks.iter().zip(self.tables_info.iter()).enumerate()
            {
                let style = if self.tab_idx == i {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                };
                let tab_name = &table.name;
                let tab = Paragraph::new(tab_name.to_string())
                    .style(style)
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).border_style(style));
                tab.render(*tab_chunk, buf);
            }

            let rows: Vec<Row> = selected_table
                .rows
                .iter()
                .map(|row| {
                    Row::new(vec![
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
                    ])
                })
                .collect();

            let header_row = Row::new(vec![
                Cell::from("ID").style(Style::default().fg(Color::Yellow)),
                Cell::from("Name").style(Style::default().fg(Color::Yellow)),
                Cell::from("Type").style(Style::default().fg(Color::Yellow)),
                Cell::from("Not Null").style(Style::default().fg(Color::Yellow)),
                Cell::from("Default").style(Style::default().fg(Color::Yellow)),
                Cell::from("PK").style(Style::default().fg(Color::Yellow)),
            ]);

            let mut all_rows = vec![header_row];
            all_rows.extend(rows);

            let table = Table::new(
                all_rows,
                &[
                    Constraint::Length(5),
                    Constraint::Length(15),
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Length(30),
                    Constraint::Length(5),
                ],
            )
            .block(
                Block::default()
                    .title(format!("Schema: {}", selected_table.name))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            );

            let table_area = popup_area[1];
            Clear.render(table_area, buf);
            table.render(table_area, buf);
        }
    }
}
