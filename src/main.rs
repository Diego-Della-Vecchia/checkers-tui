use checkers_tui::{state::State, ui::render};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::DefaultTerminal;
use std::time::Duration;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut state = State::new();

    loop {
        terminal.draw(|frame| render(frame, &mut state))?;

        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
            && key.modifiers.contains(KeyModifiers::CONTROL)
            && key.code == KeyCode::Char('c')
        {
            return Ok(());
        }
    }
}
