use checkers_tui::{state::State, ui::render};
use ratatui::DefaultTerminal;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let state = State {};

    loop {
        terminal.draw(|frame| render(frame, &state))?;
    }
}
