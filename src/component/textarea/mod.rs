pub mod attribute;
pub mod command;

use arboard::Clipboard;
use inkjet::constants::HIGHLIGHT_NAMES;
use inkjet::theme::Theme;
use inkjet::tree_sitter_highlight::HighlightEvent;
use inkjet::{Highlighter, Language};
use ratatui::style::Color;
use tui_textarea::{CursorMove, TextArea as TextAreaWidget};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, PropPayload, PropValue, Props, Style, TextModifiers,
};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Block;
use tuirealm::{Frame, MockComponent, State, StateValue};

pub struct TextArea<'a> {
    props: Props,
    widget: TextAreaWidget<'a>,
    single_line: bool,
    language: Language,
    theme: Theme,
}

impl<'a> TextArea<'a> {
    pub fn new(lines: Vec<String>, language: Language, theme: Theme) -> Self {
        Self {
            props: Props::default(),
            widget: TextAreaWidget::new(lines),
            single_line: false,
            language,
            theme,
        }
    }

    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn scroll_step(mut self, step: usize) -> Self {
        self.attr(Attribute::ScrollStep, AttrValue::Length(step));
        self
    }

    pub fn max_histories(mut self, max: usize) -> Self {
        self.attr(
            attribute::MAX_HISTORY,
            AttrValue::Payload(PropPayload::One(PropValue::Usize(max))),
        );
        self
    }

    pub fn cursor_style(mut self, s: Style) -> Self {
        self.attr(attribute::CURSOR_STYLE, AttrValue::Style(s));
        self
    }

    pub fn cursor_line_style(mut self, s: Style) -> Self {
        self.attr(attribute::CURSOR_LINE_STYLE, AttrValue::Style(s));
        self
    }

    pub fn line_number_style(mut self, s: Style) -> Self {
        self.attr(attribute::LINE_NUMBER_STYLE, AttrValue::Style(s));
        self
    }

    pub fn style(mut self, s: Style) -> Self {
        self.attr(Attribute::Style, AttrValue::Style(s));
        self
    }

    pub fn tab_length(mut self, l: u8) -> Self {
        self.attr(attribute::TAB_LENGTH, AttrValue::Size(l as u16));
        self
    }

    pub fn hard_tab(mut self, enabled: bool) -> Self {
        self.attr(attribute::HARD_TAB, AttrValue::Flag(enabled));
        self
    }

    pub fn single_line(mut self, single_line: bool) -> Self {
        self.attr(attribute::SINGLE_LINE, AttrValue::Flag(single_line));
        self
    }

    fn get_block(&self) -> Option<Block<'a>> {
        let mut block = Block::default();
        if let Some(AttrValue::Title((title, alignment))) = self.query(Attribute::Title) {
            block = block.title(title).title_alignment(alignment);
        }
        if let Some(AttrValue::Borders(borders)) = self.query(Attribute::Borders) {
            let inactive_style = self
                .query(Attribute::FocusStyle)
                .unwrap_or_else(|| AttrValue::Style(Style::default()))
                .unwrap_style();
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();

            return Some(
                block
                    .border_style(match focus {
                        true => borders.style(),
                        false => inactive_style,
                    })
                    .border_type(borders.modifiers)
                    .borders(borders.sides),
            );
        }

        None
    }

    fn paste(&mut self) {
        if let Ok(Ok(yank)) = Clipboard::new().map(|mut ctx| ctx.get_text()) {
            if self.single_line {
                self.widget.insert_str(yank);
            } else {
                for line in yank.lines() {
                    self.widget.insert_str(line);
                    self.widget.insert_newline();
                }
            }
        }
    }

    fn to_2d_position(&self, raw: usize) -> (usize, usize) {
        let lines = self.widget.lines();
        let mut sum = 0;
        for (row, line) in lines.iter().enumerate() {
            sum += line.len() + 1;
            if raw < sum {
                sum -= line.len() + 1;
                return (row, raw - sum);
            }
        }

        unreachable!()
    }

    fn highlight(&mut self) {
        self.widget.clear_custom_highlight();

        let source = self.widget.lines().join("\n");
        let mut highlighter = Highlighter::new();

        let events = highlighter.highlight_raw(self.language, &source).unwrap();
        let mut style = Style::default();

        for event in events {
            let event = event.unwrap();
            match event {
                HighlightEvent::Source { start, end } => {
                    self.widget.custom_highlight(
                        (self.to_2d_position(start), self.to_2d_position(end)),
                        style,
                        0,
                    );
                }
                HighlightEvent::HighlightStart(highlight) => {
                    let style_name = HIGHLIGHT_NAMES[highlight.0];

                    let color = self
                        .theme
                        .get_style(style_name)
                        .and_then(|s| s.fg)
                        .unwrap_or(self.theme.fg);

                    let color = Color::Rgb(color.r, color.g, color.b);
                    style = Style::default().fg(color);
                }
                HighlightEvent::HighlightEnd => style = Style::reset(),
            }
        }
    }
}

impl MockComponent for TextArea<'_> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) != AttrValue::Flag(true) {
            return;
        }

        if let Some(block) = self.get_block() {
            self.widget.set_block(block);
        }

        let focus = self
            .props
            .get_or(Attribute::Focus, AttrValue::Flag(false))
            .unwrap_flag();
        let cursor_style = if !focus {
            Style::reset()
        } else {
            self.props
                .get_or(
                    attribute::CURSOR_STYLE,
                    AttrValue::Style(Style::default().add_modifier(TextModifiers::REVERSED)),
                )
                .unwrap_style()
        };
        self.widget.set_cursor_style(cursor_style);

        frame.render_widget(&self.widget, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value.clone());
        match (attr, value) {
            (attribute::CURSOR_STYLE, AttrValue::Style(s)) => self.widget.set_cursor_style(s),
            (attribute::CURSOR_LINE_STYLE, AttrValue::Style(s)) => {
                self.widget.set_cursor_line_style(s)
            }
            (
                attribute::MAX_HISTORY,
                AttrValue::Payload(PropPayload::One(PropValue::Usize(max))),
            ) => self.widget.set_max_histories(max),
            (attribute::LINE_NUMBER_STYLE, AttrValue::Style(s)) => {
                self.widget.set_line_number_style(s)
            }
            (attribute::TAB_LENGTH, AttrValue::Size(size)) => {
                self.widget.set_tab_length(size as u8)
            }
            (attribute::HARD_TAB, AttrValue::Flag(enabled)) => {
                self.widget.set_hard_tab_indent(enabled)
            }
            (attribute::SINGLE_LINE, AttrValue::Flag(single_line)) => {
                self.single_line = single_line
            }
            (Attribute::Style, AttrValue::Style(s)) => self.widget.set_style(s),
            (_, _) => {
                if let Some(block) = self.get_block() {
                    self.widget.set_block(block);
                }
            }
        }
    }

    fn state(&self) -> State {
        State::One(StateValue::String(self.widget.lines().join("\n")))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Cancel => {
                self.widget.delete_next_char();
                CmdResult::None
            }
            command::DEL_LINE_BY_END => {
                self.widget.delete_line_by_end();
                CmdResult::None
            }
            command::DEL_LINE_BY_HEAD => {
                self.widget.delete_line_by_head();
                CmdResult::None
            }
            command::DEL_NEXT_WORD => {
                self.widget.delete_next_word();
                CmdResult::None
            }
            command::DEL_WORD => {
                self.widget.delete_word();
                CmdResult::None
            }
            command::MOVE_PARAGRAPH_BACK => {
                self.widget.move_cursor(CursorMove::ParagraphBack);
                CmdResult::None
            }
            command::MOVE_PARAGRAPH_FORWARD => {
                self.widget.move_cursor(CursorMove::ParagraphForward);
                CmdResult::None
            }
            command::MOVE_WORD_BACK => {
                self.widget.move_cursor(CursorMove::WordBack);
                CmdResult::None
            }
            command::MOVE_WORD_FORWARD => {
                self.widget.move_cursor(CursorMove::WordForward);
                CmdResult::None
            }
            command::MOVE_BOTTOM => {
                if !self.single_line {
                    self.widget.move_cursor(CursorMove::Bottom);
                }
                CmdResult::None
            }
            command::MOVE_TOP => {
                if !self.single_line {
                    self.widget.move_cursor(CursorMove::Top);
                }
                CmdResult::None
            }
            command::PASTE => {
                self.paste();
                CmdResult::None
            }
            command::REDO => {
                self.widget.redo();
                CmdResult::None
            }
            command::UNDO => {
                self.widget.undo();
                CmdResult::None
            }
            Cmd::Delete => {
                self.widget.delete_char();
                CmdResult::None
            }
            Cmd::GoTo(Position::Begin) => {
                self.widget.move_cursor(CursorMove::Head);
                CmdResult::None
            }
            Cmd::GoTo(Position::End) => {
                self.widget.move_cursor(CursorMove::End);
                CmdResult::None
            }
            Cmd::Move(Direction::Down) => {
                if !self.single_line {
                    self.widget.move_cursor(CursorMove::Down);
                }
                CmdResult::None
            }
            Cmd::Move(Direction::Left) => {
                self.widget.move_cursor(CursorMove::Back);
                CmdResult::None
            }
            Cmd::Move(Direction::Right) => {
                self.widget.move_cursor(CursorMove::Forward);
                CmdResult::None
            }
            Cmd::Move(Direction::Up) => {
                if !self.single_line {
                    self.widget.move_cursor(CursorMove::Up);
                }
                CmdResult::None
            }
            Cmd::Scroll(Direction::Down) => {
                if !self.single_line {
                    let step = self
                        .props
                        .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                        .unwrap_length();
                    (0..step).for_each(|_| self.widget.move_cursor(CursorMove::Down));
                }
                CmdResult::None
            }
            Cmd::Scroll(Direction::Up) => {
                if !self.single_line {
                    let step = self
                        .props
                        .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                        .unwrap_length();
                    (0..step).for_each(|_| self.widget.move_cursor(CursorMove::Up));
                }
                CmdResult::None
            }
            Cmd::Type('\t') => {
                self.widget.insert_tab();
                CmdResult::None
            }
            Cmd::Type('\n') | command::NEWLINE => {
                if !self.single_line {
                    self.widget.insert_newline();
                }
                CmdResult::None
            }
            Cmd::Type(ch) => {
                self.widget.insert_char(ch);
                CmdResult::None
            }
            Cmd::Submit => CmdResult::Submit(self.state()),
            _ => CmdResult::None,
        }
    }
}
