use core::fmt;

use crate::state::{PlayerTurn, State};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

fn render_header(frame: &mut Frame, area: Rect) {
    let art = r"
 ██████╗██╗  ██╗███████╗ ██████╗██╗  ██╗███████╗██████╗ ███████╗    ████████╗██╗   ██╗██╗
██╔════╝██║  ██║██╔════╝██╔════╝██║ ██╔╝██╔════╝██╔══██╗██╔════╝    ╚══██╔══╝██║   ██║██║
██║     ███████║█████╗  ██║     █████╔╝ █████╗  ██████╔╝███████╗       ██║   ██║   ██║██║
██║     ██╔══██║██╔══╝  ██║     ██╔═██╗ ██╔══╝  ██╔══██╗╚════██║       ██║   ██║   ██║██║
╚██████╗██║  ██║███████╗╚██████╗██║  ██╗███████╗██║  ██║███████║       ██║   ╚██████╔╝██║
 ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚══════╝       ╚═╝    ╚═════╝ ╚═╝
                                                                                         
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

fn render_board(frame: &mut Frame, area: Rect, state: &mut State) {
    // terminal cells are roughly 2x taller than wide
    let mut width = area.width;
    let mut height = area.height;

    if width > height * 2 {
        width = height * 2;
    } else {
        height = width / 2;
    }

    // render board with black borders
    let container = Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    };
    let container_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().black());

    frame.render_widget(container_block, container);
}

fn render_footer(frame: &mut Frame, area: Rect, state: &State) {
    let text = match state.current_turn {
        PlayerTurn::PlayerOne => String::from("Current turn: Player one"),
        PlayerTurn::PlayerTwo => String::from("Current turn: Player two"),
    };

    let paragraph = Paragraph::new(text)
        .style(Style::new().red())
        .alignment(HorizontalAlignment::Center);

    frame.render_widget(paragraph, area);
}

pub fn render(frame: &mut Frame, state: &mut State) {
    let layout = Layout::default()
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(65),
            Constraint::Percentage(10),
        ])
        .split(frame.area());

    let header_area = layout[0];
    let board_area = layout[1];
    let footer_area = layout[2];

    render_header(frame, header_area);
    render_board(frame, board_area, state);
    render_footer(frame, footer_area, state);
}
