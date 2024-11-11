use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::style::{Color, Style};
use ratatui::text::ToText;
use ratatui::widgets::{Cell, List, ListItem, Row as TableRow, Table};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::Widget,
    widgets::{Block, Borders, Paragraph},
};
use std::path::Path;
use std::time::{Duration, Instant};
use sqlx::{Row, SqlitePool};
use crate::app::App;
use crate::controllers::{get_question};

#[derive(Debug)]
pub struct InGamePage {
    exit: bool,
    score: u8,
    time_left: f32,
    tick_rate: Duration,
    last_instant: Instant,
    input: String,
    selected_block: usize,
    question: String,
    selected_option: usize,
    options: Vec<String>,
    result: String,
    current_is_done: bool,
}
impl InGamePage {
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
        self.selected_block = (self.selected_block + 1) % 4; // Cycle through 4 blocks
    }

    pub fn previous_block(&mut self) {
        if self.selected_block == 0 {
            self.selected_block = 3; // Go to the last block
        } else {
            self.selected_block -= 1;
        }
    }

    pub fn update_states(&mut self) {
        if self.last_instant.elapsed() >= Duration::from_secs(1) {
            self.time_left -= 1.0;
            self.last_instant = Instant::now();
        }
    }
    pub fn handle_key_events(&mut self, app: &mut App,db: &SqlitePool) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                        app.exit = true;
                    }
                    (_, KeyCode::Left) => self.previous_block(),
                    (_, KeyCode::Right) => self.next_block(),
                    (_, KeyCode::Char(c)) => {
                        if self.selected_block == 0 {
                            self.input.push(c)
                        }
                    }
                    (_, KeyCode::Backspace) => {
                        self.input.pop();
                    }
                    (_, KeyCode::Enter) => {
                        if (self.selected_block == 3 && self.selected_option == 1){
                            InGamePage::view_schema(db);
                        } else if (self.selected_block == 0) {
                            self.input.push('\n');
                        }
                    }
                    (_, KeyCode::Up) => {
                        if self.selected_block == 3 {
                            self.next_option()
                        };
                    }
                    (_, KeyCode::Down) => {
                        if self.selected_block == 3 {
                            self.previous_option()
                        };
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            selected_block: 0,
            input: String::new(),
            exit: false,
            time_left: 30.0,
            tick_rate: Duration::from_millis(50),
            last_instant: Instant::now(),
            question: String::new(),
            options: vec![
                "Run".to_string(),
                "View Schema".to_string(),
                "Exit".to_string(),
                "Clear Screen".to_string(),
            ],
            selected_option: 0,
            score: 0,
            result: "None".to_string(),
            current_is_done: true,
        }
    }
    async fn view_schema( pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        let mut terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;
        let columns = sqlx::query(&format!("PRAGMA table_info({})", "scores"))
            .fetch_all(pool)
            .await?
            .iter()
            .map(|row| row.get::<String, _>("name"))
            .collect::<Vec<String>>();
        let query = format!("SELECT * FROM {}", "scores");
        let rows = sqlx::query(&query).fetch_all(pool).await?;
        let header = TableRow::new(
            columns
                .iter()
                .map(|col| Cell::from(col.clone()).style(Style::default().fg(Color::Yellow)))
                .collect::<Vec<Cell>>(),
        );let widths = [
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(10),
        ];
        let table_rows = vec![
            TableRow::new(vec![Cell::from("Row 1 - Col 1"), Cell::from("Row 1 - Col 2")]),
            TableRow::new(vec![Cell::from("Row 2 - Col 1"), Cell::from("Row 2 - Col 2")]),
        ];
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let table = Table::new(table_rows,widths)
                .header(header)
                .block(Block::default().title("Header").borders(Borders::ALL));
            f.render_widget(table, chunks[0]);
        })?;
        Ok(())
    }
    pub async fn update_question(&mut self) -> Result<()> {
        let question_idx = 1;
        let current_question_path = format!("./questions/question-{}", question_idx);
        let confirmed_answer = Path::new(&current_question_path).join("confirmed_answer.sql");

        if self.current_is_done {
            if !confirmed_answer.exists() {
                let question = get_question(Path::new(&current_question_path)).await?;
                self.question = question.question;
                self.current_is_done = false;
            }
        }

        Ok(())
    }
}
impl Widget for &InGamePage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //Create All Block
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
        // Generate Query, Question and Result Block
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
            let block = Block::default()
                .title(title.as_str())
                .borders(Borders::ALL)
                .border_style(if self.selected_block == i {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                });
            let paragraph = Paragraph::new(content.to_text())
                .block(block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            paragraph.render(*chunk, buf);
        }
        // Generate Options Block
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
        // Block Score
        let block_score = Block::default().title("Score").borders(Borders::ALL);
        Paragraph::new(self.score.to_string())
            .centered()
            .block(block_score)
            .render(time_and_score_area[0], buf);
        // Block Time Left
        let block_time_left = Block::default().title("Time left").borders(Borders::ALL);
        let block_hotkey_guide = Block::default().borders(Borders::ALL);

        Paragraph::new(self.time_left.to_string())
            .centered()
            .block(block_time_left)
            .render(time_and_score_area[1], buf);
        let hotkey_guide = Text::from(vec![Line::from(vec![
            "Enter: Choose / \u{2192}, \u{2190}: Move / Esc: Open Menu / "
                .green()
                .into(),
        ])]);
        Paragraph::new(hotkey_guide)
            .centered()
            .block(block_hotkey_guide)
            .render(time_and_score_area[2], buf);
    }
}
