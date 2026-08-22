use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::DefaultTerminal;
use std::error::Error;

use ratatui_minimal_toggle_button::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let mut toggle = Toggle::new(Some(String::from("It's an example!")));
    toggle.icon_disable = "(x )".to_string();
    toggle.icon_enable = "( X)".to_string();

    loop {
        terminal.draw(|f| {
            f.render_stateful_widget(toggle.clone(), f.area(), &mut toggle.state);
        })?;

        let event = event::read()?;

        if let Event::Key(key) = &event {
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                break;
            }
        }

        toggle.state.toggle();
    }

    Ok(())
}
