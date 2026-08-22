use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

#[derive(Debug, Clone)]
pub struct Toggle {
    pub icon_disable: String,
    pub icon_enable: String,
    pub label: String,
    pub active_style: Style,
    pub inactive_style: Style,
    pub focus_style: Style,
    pub unfocus_style: Style,
    pub state: ToggleState,
    pub is_focused: bool,
}

impl Toggle {
    pub fn new(content: Option<String>) -> Self {
        Self {
            icon_disable: String::from("[ ]"),
            icon_enable: String::from("[X]"),
            label: content.unwrap_or_default(),
            active_style: Style::default().fg(Color::White),
            inactive_style: Style::default().fg(Color::DarkGray),
            focus_style: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),
            unfocus_style: Style::default(),
            state: ToggleState::Disable,
            is_focused: false,
        }
    }

    pub fn active_style(mut self, style: Style) -> Self {
        self.active_style = style;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleState {
    Enable,
    #[default]
    Disable,
}

impl ToggleState {
    pub fn toggle(&mut self) {
        *self = match self {
            ToggleState::Enable => ToggleState::Disable,
            ToggleState::Disable => ToggleState::Enable,
        };
    }
}

impl StatefulWidget for Toggle {
    type State = ToggleState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (icon, style) = match state {
            ToggleState::Enable => (self.icon_enable, self.active_style),
            ToggleState::Disable => (self.icon_disable, self.inactive_style),
        };

        let focus_style = match self.is_focused {
            true => self.focus_style,
            false => self.unfocus_style,
        };

        let text = format!("{} {}", icon, self.label);
        let paragraph = Paragraph::new(text).style(style.patch(focus_style));

        paragraph.render(area, buf);
    }
}
