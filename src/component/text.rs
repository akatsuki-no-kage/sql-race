use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};
use tuirealm::{
    AttrValue, Attribute, MockComponent, Props, State,
    command::{Cmd, CmdResult},
    props::{Alignment, BorderSides, Borders, Color},
};

#[derive(Default)]
pub struct Text {
    props: Props,
}

impl MockComponent for Text {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let text = self
            .props
            .get_or(Attribute::Text, AttrValue::String(String::new()))
            .unwrap_string();

        let (title, title_alignment) = self
            .props
            .get_or(
                Attribute::Title,
                AttrValue::Title((String::new(), Alignment::Left)),
            )
            .unwrap_title();

        let border_color = if self.props.get_or(Attribute::Focus, AttrValue::Flag(true))
            != AttrValue::Flag(true)
        {
            Color::Green
        } else {
            Color::White
        };
        let borders = self
            .props
            .get_or(
                Attribute::Borders,
                AttrValue::Borders(Borders::default().sides(BorderSides::all())),
            )
            .unwrap_borders()
            .color(border_color);

        let widget = Paragraph::new(text).block(
            Block::new()
                .title(title)
                .title_alignment(title_alignment)
                .borders(borders.sides)
                .border_style(borders.style()),
        );

        frame.render_widget(widget, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Text {
    pub fn text(mut self, s: impl AsRef<str>) -> Self {
        self.attr(Attribute::Text, AttrValue::String(s.as_ref().to_string()));
        self
    }

    pub fn title(mut self, title: Option<String>, alignment: Option<Alignment>) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((
                title.unwrap_or_default(),
                alignment.unwrap_or(Alignment::Left),
            )),
        );
        self
    }

    pub fn border_side(mut self, side: BorderSides) -> Self {
        self.attr(
            Attribute::Borders,
            AttrValue::Borders(Borders::default().sides(side)),
        );
        self
    }
}
