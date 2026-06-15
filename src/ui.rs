use crate::state::State;
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

fn render_header(frame: &mut Frame, area: Rect) {
    let art = r"
 ███  █   █ █████  ███  █   █ █████ ████   ████    █████ █   █ ███   
█ ░░░ █░  █░█░░░░░█ ░░░ █░ █ ░█░░░░░█░░░█ █ ░░░░    ░█░░░█░  █░ █░░  
█░ ░░░█████░████░░█░ ░░░███ ░ ████░░████░░ ███░░░    █░░░█░░ █░░█░░░ 
█░░   █░░░█░█░░░░ █░░   █░░█ ░█░░░░ █░░█░ ░ ░░█      █░░ █░░ █░░█░░  
 ███  █░░░█░█████░ ███  █░░░█ █████░█░░░█░████░░     █░░  ███ ░███░  
  ░░░  ░░  ░░░░░░░  ░░░  ░░  ░ ░░░░░ ░░  ░ ░░░░ ░     ░░   ░░░ ░░░░  
   ░░░  ░   ░ ░░░░░  ░░░  ░   ░ ░░░░░ ░   ░ ░░░░       ░    ░░░  ░░░ 
    ";

    let text: Vec<Line> = art
        .lines()
        .map(|line| Line::from(Span::raw(line)))
        .collect();

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Red))
        .alignment(HorizontalAlignment::Center);

    frame.render_widget(paragraph, area);
}

pub fn render(frame: &mut Frame, state: &mut State) {
    let layout = Layout::default()
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(70),
            Constraint::Percentage(5),
        ])
        .split(frame.area());

    let header_area = layout[0];
    let play_area = layout[1];
    let footer_area = layout[2];

    render_header(frame, header_area);
}
