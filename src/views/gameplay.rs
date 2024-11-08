use std::time::{Duration};
use ratatui::{ backend::CrosstermBackend, widgets::{Block, Borders, Paragraph}, layout::{Layout, Constraint, Direction}, Terminal, buffer::Buffer, layout::{Alignment, Rect}, style::Stylize, symbols::border, text::{Line, Text, Span}, widgets::{ block::{Position, Title}, Widget, }, DefaultTerminal, Frame, };
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,KeyEvent, KeyEventKind};
use tokio::time::sleep;
use std::io;use ratatui::style::{Color, Style};
use ratatui::text::ToText;
use ratatui::widgets::{List, ListItem};

#[derive(Debug, Default)]
pub struct gameplayScreen {
    exit: bool,
    score:u8,
    time_left:f32,
    tick_rate:Duration,
    input: String,
    selected_block: usize,
    question: String,
    selected_option: usize,
    options: Vec<String>,
    result: String,
}
impl gameplayScreen {
    async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.new();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            sleep(self.tick_rate).await;
            self.time_left -= 0.05;
        }
        Ok(())
    }
    pub fn next_option(&mut self) {
        if self.selected_option == 0{
            self.selected_option = self.options.len() - 1;
        }
        else {
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
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(self.tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Left => self.previous_block(),
                    KeyCode::Right => self.next_block(),
                    KeyCode::Char(c) => self.input.push(c),
                    KeyCode::Backspace => { self.input.pop(); },
                    KeyCode::Enter => { self.input.push('\n'); },
                    KeyCode::Up => self.next_option(),
                    KeyCode::Down => self.previous_option(),
                    _ => {}
                }

            }
        }
        Ok(())
    }
    fn exit(&mut self) {
        self.exit = true;
    }
    fn new(&mut self) {
        self.selected_block = 0;
        self.input = String::new();
        self.exit = false;
        self.time_left = 20f32;
        self.tick_rate = Duration::from_millis(50);
        self.question = String::new();
        self.options = vec!["Run".to_string(), "View Schema".to_string(), "Exit".to_string(), "Clear Screen".to_string()];
    }
}
impl Widget for &gameplayScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //Create All Block
        let main_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(70),
                Constraint::Percentage(25),
            ].as_ref())
            .split(area);
        let time_and_score_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(20),
            ].as_ref())
            .split(main_area[0]);
        let query_and_question_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30)
            ].as_ref())
            .split(main_area[1]);
        let result_and_features_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30)
            ].as_ref())
            .split(main_area[2]);
        // Generate Query, Question and Result Block
        let chunks = vec![query_and_question_area[0], query_and_question_area[1], result_and_features_area[0]];
        let blocks = vec![
            ("Query".to_string(), self.input.as_str(), Color::Red),
            ("Question".to_string(), self.question.as_str(), Color::Green),
            ("Result".to_string(), self.result.as_str(), Color::Blue),
        ];
        for (i, (chunk, (title, content, color))) in chunks.iter().zip(blocks.iter()).enumerate() {
            let block = Block::default()
                .title(title.as_str())
                .borders(Borders::ALL)
                .border_style(
                    if self.selected_block == i {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::White)
                    },
                );
            let paragraph = Paragraph::new(content.to_text())
                .block(block)
                .wrap(ratatui::widgets::Wrap { trim: true });
            paragraph.render(*chunk, buf);
        }
        // Generate Options Block
        let items: Vec<ListItem> = self.options.iter().enumerate().map(|(i, option)| {
            let style = if i == self.selected_option {
                Style::default().bg(Color::Green)
            } else {
                Style::default()
            };
            ListItem::new(option.to_string()).style(style)
        }).collect();
        let list = List::new(items)
            .block(Block::default().title("Options").borders(Borders::ALL).border_style(if self.selected_block == 3 {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            },));
        list.render(result_and_features_area[1], buf);
        // Block Score
        let block_score = Block::default()
            .title("Score")
            .borders(Borders::ALL);
        let score_text = Text::from(vec![Line::from(vec![
            self.score.to_string().green(),
        ])]);
        Paragraph::new(score_text)
            .centered()
            .block(block_score)
            .render(time_and_score_area[0], buf);
        // Block Time Left
        let block_time_left = Block::default()
            .title("Time left")
            .borders(Borders::ALL);
        let block_hotkey_guide = Block::default()
            .borders(Borders::ALL);
        let time_left_text = Text::from(vec![Line::from(vec![
            format!("{:.1}",self.time_left).green(),
        ])]);
        Paragraph::new(time_left_text)
            .centered()
            .block(block_time_left)
            .render(time_and_score_area[1], buf);
        let hotkey_guide = Text::from(vec![Line::from(vec![
            "Enter: Choose / \u{2192}, \u{2190}: Move / Esc: Open Menu / ".green().into(),
        ])]);
        Paragraph::new(hotkey_guide)
            .centered()
            .block(block_hotkey_guide)
            .render(time_and_score_area[2], buf);
    }
}