use std::cell;

use crate::state::{COL_COUNT, PlayerTurn, ROW_COUNT, State};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Rect},
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
    // reserve 2 spaces for board borders
    let max_inner_width = area.width.saturating_sub(2);
    let max_inner_height = area.height.saturating_sub(2);

    let max_cell_width = max_inner_width / COL_COUNT as u16;
    let max_cell_height = max_inner_height / ROW_COUNT as u16;

    // term cells are roughly 2x as wide as they are tall
    let (cell_width, cell_height) = if max_cell_width < max_cell_height {
        (max_cell_width * 2, max_cell_width)
    } else {
        (max_cell_height * 2, max_cell_height)
    };

    let container_width = cell_width * COL_COUNT as u16;
    let container_height = cell_height * ROW_COUNT as u16;

    let container_rect = Rect {
        x: area.x + (area.width - container_width) / 2,
        y: area.y + (area.height - container_height) / 2,
        // use +2 to account for the borders of the container block
        width: container_width + 2,
        height: container_height + 2,
    };

    let container_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().black());
    let container_inner = container_block.inner(container_rect);

    frame.render_widget(container_block, container_rect);

    let rows_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(cell_height); ROW_COUNT])
        .split(container_inner);

    for y in 0..ROW_COUNT {
        let row_area = rows_layout[y];
        let cols_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(cell_width); COL_COUNT])
            .split(row_area);

        for x in 0..COL_COUNT {
            let col_area = cols_layout[x];
            let is_black = (x + y) % 2 == 1;

            let cell_block = Block::default().style(if is_black {
                Style::new().bg(Color::Black)
            } else {
                Style::new().bg(Color::White)
            });
            frame.render_widget(cell_block, col_area);
        }
    }
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
