use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
};
use std::error::Error;

use ratatui_minimal_toggle_button::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

struct ToggleList {
    items: Vec<Toggle>,
    focused_index: Option<usize>,
}

impl ToggleList {
    fn focus_last(&mut self) {
        if self.focused_index.is_none() && self.items.len() != 0 {
            self.focused_index = Some(self.items.len() - 1);
        }
    }

    fn focus_first(&mut self) {
        if self.focused_index.is_none() && self.items.len() != 0 {
            self.focused_index = Some(0);
        }
    }

    fn focus_next(&mut self) {
        if self.focused_index.is_none() {
            self.focus_first();
            return;
        }

        self.focused_index = Some((self.focused_index.unwrap() + 1) % self.items.len())
    }

    fn focus_previous(&mut self) {
        if self.focused_index.is_none() {
            self.focus_first();
            return;
        }

        let len = self.items.len();
        self.focused_index = Some((self.focused_index.unwrap() + len - 1) % len);
    }

    fn toggle_current(&mut self) {
        if self.focused_index.is_some() {
            self.items[self.focused_index.unwrap()].state.toggle();
        }
    }
}

fn run(terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let mut toggle_list = ToggleList {
        items: vec![
            Toggle::new(Some(String::from("Do a Toggle Button"))),
            Toggle::new(Some(String::from("Learn Rust"))),
            Toggle::new(Some(String::from("Code the example"))),
        ],
        focused_index: None,
    };

    loop {
        terminal.draw(|f| {
            let constraints = vec![Constraint::Length(1); toggle_list.items.len()];
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(f.area());

            for (i, toggle) in toggle_list.items.iter_mut().enumerate() {
                let mut toggle_widget = toggle.clone();

                if toggle_list.focused_index.is_some() {
                    if i == toggle_list.focused_index.unwrap() {
                        toggle_widget.is_focused = true
                    }
                }

                f.render_stateful_widget(toggle_widget.clone(), chunks[i], &mut toggle_widget.state);
            }
        })?;

        let event = read()?;

        if let Event::Key(key) = &event {
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                break;
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                    toggle_list.focus_previous();
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                    toggle_list.focus_next();
                }
                KeyCode::PageUp => toggle_list.focus_last(),
                KeyCode::PageDown => toggle_list.focus_last(),
                KeyCode::Enter => toggle_list.toggle_current(),
                KeyCode::Esc => toggle_list.focused_index = None,
                _ => {}
            }
        }
    }

    Ok(())
}
