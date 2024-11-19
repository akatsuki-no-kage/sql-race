use crate::app::{App, AppState};
use crate::controllers::{get_question, get_score, run_query, view_schemas};
use crate::models::schema::QuestionTable;
use crate::models::score::Score;
use anyhow::Result;
use ratatui::crossterm::event;
use ratatui::style::{Color, Style};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::Widget,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};
use std::path::Path;
use std::time::{Duration, Instant};
use tui_textarea::{CursorMove, Input, Key, TextArea};

use super::components::schema_table::SchemaComponent;
use super::components::table::TableComponent;

const TIME: u64 = 10;

pub struct InGamePage<'a> {
    query_textarea: TextArea<'a>,
    score: i64,
    pub time_start: Instant,
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
    result: Option<TableComponent>,
    schema_table: Option<SchemaComponent>,
    current_is_done: bool,
    scroll_question_x: u16,
    scroll_question_y: u16,
}

impl<'a> InGamePage<'a> {
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
        self.query_textarea.move_cursor(CursorMove::Back);
    }

    pub fn move_cursor_right(&mut self) {
        self.query_textarea.move_cursor(CursorMove::Forward);
    }
    pub fn move_cursor_up(&mut self) {
        self.query_textarea.move_cursor(CursorMove::Up);
    }

    pub fn move_cursor_down(&mut self) {
        self.query_textarea.move_cursor(CursorMove::Down);
    }

    pub fn update_states(&mut self, app: &mut App) {
        if self.tab_idx >= self.tables_info.len() {
            self.tab_idx = 0;
        }

        if (Instant::now() - self.time_start).as_secs() > TIME {
            app.state = AppState::Menu;
        }

        if self.selected_block == 0 {
            self.query_textarea.set_block(
                Block::default()
                    .title("Query")
                    .borders(Borders::ALL)
                    .fg(Color::Green),
            )
        } else if self.selected_block == 2 {
            match &mut self.result {
                Some(table) => table.is_focus = true,
                None => {}
            }
        } else {
            match &mut self.result {
                Some(table) => table.is_focus = false,
                None => {}
            }
            self.query_textarea.set_block(
                Block::default()
                    .title("Query")
                    .borders(Borders::ALL)
                    .fg(Color::White),
            )
        }
    }

    pub async fn handle_key_events(&mut self, app: &mut App) -> Result<()> {
        let has_event = event::poll(Duration::from_millis(100))?;
        if !has_event {
            return Ok(());
        }
        match event::read()?.into() {
            Input {
                ctrl: true,
                key: Key::Char('q'),
                ..
            } => {
                app.exit = true;
            }
            Input {
                ctrl: true,
                key: Key::Char('r'),
                ..
            } => {
                self.run_query().await?;
            }
            Input {
                ctrl: true,
                key: Key::Char('h'),
                ..
            } => {
                match get_question(Path::new(&format!(
                    "./questions/question-{}",
                    self.question_idx
                )))
                .await
                {
                    Ok(question) => self.view_schema(&question.schema).await?,
                    Err(_) => {
                        // TODO:
                    }
                }
            }
            Input {
                ctrl: true,
                key: Key::Char('s'),
                ..
            } => {
                match get_question(Path::new(&format!(
                    "./questions/question-{}",
                    self.question_idx
                )))
                .await
                {
                    Ok(question) => {
                        let query_input = self.query_textarea.lines().join("\n");
                        match get_score(&query_input, &question.answer, &question.schema).await {
                            Ok(is_correct) => {
                                if is_correct {
                                    self.score += self.question_idx as i64 * 10;
                                    Score::update_score(
                                        &app.pool,
                                        app.username.clone(),
                                        self.score,
                                    )
                                    .await?;
                                    self.query_textarea.select_all();
                                    self.query_textarea.cut();

                                    self.cursor_position = 0;
                                    self.result = Some(TableComponent::new(vec![]));
                                    self.current_is_done = true;
                                } else {
                                    self.result = Some(TableComponent::with_error(
                                        "Wrong answer, try again!".to_string(),
                                    ));
                                }
                            }
                            Err(err) => {
                                self.result = Some(TableComponent::with_error(format!(
                                    "Error calculating score: {:?}",
                                    err
                                )));
                            }
                        }
                    }
                    Err(err) => self.result = Some(TableComponent::with_error(err.to_string())),
                }
            }
            Input { key: Key::Up, .. } => {
                if self.selected_block == 3 {
                    self.next_option();
                } else if self.selected_block == 0 {
                    self.move_cursor_up();
                } else if self.selected_block == 2 {
                    match &mut self.result {
                        Some(table) => table.previous(),
                        None => {}
                    }
                }
            }
            Input { key: Key::Down, .. } => {
                if self.selected_block == 3 {
                    self.previous_option();
                } else if self.selected_block == 0 {
                    self.move_cursor_down();
                } else if self.selected_block == 2 {
                    match &mut self.result {
                        Some(table) => table.next(),
                        None => {}
                    }
                }
            }
            Input {
                ctrl: false,
                key: Key::Left,
                ..
            } => {
                if self.popup_visible {
                    match &mut self.schema_table {
                        Some(schema) => schema.previous_tab(),
                        None => {}
                    }
                } else if self.selected_block == 0 {
                    self.move_cursor_left();
                }
            }

            Input {
                ctrl: true,
                key: Key::Left,
                ..
            } => {
                if self.popup_visible {
                    match &mut self.schema_table {
                        Some(schema) => schema.previous_tab(),
                        None => {}
                    }
                } else {
                    self.previous_block();
                }
            }
            Input {
                ctrl: true,
                key: Key::Right,
                ..
            } => {
                if self.popup_visible {
                    match &mut self.schema_table {
                        Some(schema) => schema.next_tab(),
                        None => {}
                    }
                } else {
                    self.next_block();
                }
            }
            Input {
                ctrl: false,
                key: Key::Right,
                ..
            } => {
                if self.popup_visible {
                    match &mut self.schema_table {
                        Some(schema) => schema.next_tab(),
                        None => {}
                    }
                } else if self.selected_block == 0 {
                    self.move_cursor_right();
                }
            }
            Input {
                key: Key::Backspace,
                ..
            } => {
                if self.selected_block == 0 {
                    self.query_textarea.delete_char();
                }
            }
            Input {
                key: Key::Enter, ..
            } => {
                if self.selected_block == 3 && self.selected_option == 1 {
                    match get_question(Path::new(&format!(
                        "./questions/question-{}",
                        self.question_idx
                    )))
                    .await
                    {
                        Ok(question) => self.view_schema(&question.schema).await?,
                        Err(_) => {
                            self.result = Some(TableComponent::with_error(format!(
                                "Cannot get question - {}",
                                self.question_idx
                            )));
                        }
                    }
                } else if self.selected_block == 3 && self.selected_option == 0 {
                    self.run_query().await?;
                } else if self.selected_block == 3 && self.selected_option == 2 {
                    match get_question(Path::new(&format!(
                        "./questions/question-{}",
                        self.question_idx
                    )))
                    .await
                    {
                        Ok(question) => {
                            match get_score(&self.input, &question.answer, &question.schema).await {
                                Ok(is_correct) => {
                                    if is_correct {
                                        app.score += self.question_idx as i64 * 10;
                                        Score::update_score(
                                            &app.pool,
                                            app.username.clone(),
                                            app.score,
                                        )
                                        .await?;
                                        self.query_textarea = TextArea::default();
                                        self.score = app.score;
                                        self.current_is_done = true;
                                    }
                                }
                                Err(err) => {
                                    self.result = Some(TableComponent::with_error(format!(
                                        "Error calculating score: {:?}",
                                        err
                                    )));
                                }
                            }
                        }
                        Err(err) => self.result = Some(TableComponent::with_error(err.to_string())),
                    }
                } else if self.selected_block == 3 && self.selected_option == 3 {
                    app.exit = true;
                } else if self.selected_block == 0 {
                    self.query_textarea.input(Input {
                        key: Key::Char('\n'),
                        ..Default::default()
                    });
                    self.cursor_position += 1;
                }
            }
            input => {
                if self.selected_block == 0 {
                    self.query_textarea.input(input);
                }
            }
        }
        Ok(())
    }

    async fn view_schema(&mut self, schema: &str) -> Result<()> {
        if !self.popup_visible {
            self.schema_table = Some(SchemaComponent::new(view_schemas(schema).await?));
            self.popup_visible = true;
        } else {
            self.popup_visible = false;
        }
        Ok(())
    }

    async fn run_query(&mut self) -> Result<()> {
        let query_input = self.query_textarea.lines().join("\n");
        let schema = get_question(Path::new(&format!(
            "./questions/question-{}",
            self.question_idx
        )))
        .await?
        .schema;

        self.result = match run_query(&query_input, &schema).await {
            Ok(rows) => Some(TableComponent::new(rows)),
            Err(err) => Some(TableComponent::with_error(err.to_string())),
        };
        Ok(())
    }

    pub fn new() -> Self {
        let mut query_textarea = TextArea::default();
        query_textarea.set_block(Block::default().title("Query").borders(Borders::ALL));

        Self {
            query_textarea,
            selected_block: 0,
            input: String::new(),
            cursor_position: 0,
            time_start: Instant::now(),
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
            result: Some(TableComponent::new(vec![])),
            schema_table: None,
            current_is_done: true,
            scroll_question_x: 0,
            scroll_question_y: 0,
        }
    }

    pub async fn update_question(&mut self) -> Result<()> {
        if self.current_is_done {
            self.question_idx += 1;
            let current_question_path = format!("./questions/question-{}", self.question_idx);
            match get_question(Path::new(&current_question_path)).await {
                Ok(question) => {
                    self.question = question.question;
                    self.current_is_done = false;
                }
                Err(_) => {
                    self.result = Some(TableComponent::new(vec![]));
                }
            };
        }

        Ok(())
    }
}

impl Widget for &InGamePage<'_> {
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

        // Iterate through blocks and render based on selected_block
        let block_style = if self.selected_block == 1 {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        let block = Block::default()
            .title("Question")
            .borders(Borders::ALL)
            .border_style(block_style);

        let paragraph = Paragraph::new(Text::from(self.question.as_str()))
            .block(block)
            .scroll((0, 0))
            .wrap(ratatui::widgets::Wrap { trim: true });

        paragraph.render(query_and_question_area[1], buf);

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

        let time_left = (Instant::now() - self.time_start).as_secs();
        Paragraph::new(time_left.to_string())
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

        self.query_textarea.render(query_and_question_area[0], buf);
        match &self.result {
            Some(table) => table.render(result_and_features_area[0], buf),
            None => {}
        }

        if self.popup_visible {
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

            // Clear the background
            Clear.render(area, buf);

            // Render `SchemaComponent` as a popup
            match &self.schema_table {
                Some(schema) => schema.render(popup_area_horizontal, buf),
                None => {}
            }
        }
    }
}
