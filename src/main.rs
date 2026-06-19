use checkers_tui::{state::State, ui};
use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use ratatui::{DefaultTerminal, layout::Rect};
use std::time::Duration;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

struct MouseGuard;

impl Drop for MouseGuard {
    fn drop(&mut self) {
        let _ = crossterm::execute!(std::io::stdout(), event::DisableMouseCapture);
    }
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut state = State::new();

    crossterm::execute!(std::io::stdout(), event::EnableMouseCapture)?;
    let _mouse_guard = MouseGuard;

    loop {
        terminal.draw(|frame| ui::render(frame, &mut state))?;

        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key)
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c') =>
                {
                    return Ok(());
                }
                Event::Mouse(mouse)
                    if mouse.kind == MouseEventKind::Down(MouseButton::Left) =>
                {
                    let (width, height) = crossterm::terminal::size()?;
                    let area = Rect::new(0, 0, width, height);
                    if let Some(index) = ui::cell_index_at(area, mouse.column, mouse.row) {
                        state.handle_click(index);
                    }
                }
                _ => {}
            }
        }
    }
}
