use crate::state::{COL_COUNT, PlayerTurn, ROW_COUNT, State};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, HorizontalAlignment, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::rc::Rc;
use std::vec;

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
        .style(Style::default().fg(Color::White))
        .alignment(HorizontalAlignment::Center);

    frame.render_widget(paragraph, area);
}

fn board_layout(board_area: Rect) -> (Rect, Rect, u16, u16) {
    let max_inner_width = board_area.width.saturating_sub(2);
    let max_inner_height = board_area.height.saturating_sub(2);

    let max_cell_width = max_inner_width / COL_COUNT as u16;
    let max_cell_height = max_inner_height / ROW_COUNT as u16;

    // Terminal cells are roughly 2x as wide as they are tall.
    let (cell_width, cell_height) = if max_cell_width < max_cell_height {
        (max_cell_width * 2, max_cell_width)
    } else {
        (max_cell_height * 2, max_cell_height)
    };

    let container_width = cell_width * COL_COUNT as u16;
    let container_height = cell_height * ROW_COUNT as u16;

    let container_rect = Rect {
        x: board_area.x + (board_area.width - container_width) / 2,
        y: board_area.y + (board_area.height - container_height) / 2,
        // +2 accounts for the borders of the container block.
        width: container_width + 2,
        height: container_height + 2,
    };

    let container_block = Block::default().borders(Borders::ALL);
    let container_inner = container_block.inner(container_rect);

    (container_rect, container_inner, cell_width, cell_height)
}

fn main_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(65),
            Constraint::Percentage(10),
        ])
        .split(area)
}

fn render_board(frame: &mut Frame, area: Rect, state: &mut State) {
    let (container_rect, container_inner, cell_width, cell_height) = board_layout(area);

    let container_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().black());

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
            render_cell(frame, col_area, state, x, y);
        }
    }
}

fn render_cell(frame: &mut Frame, area: Rect, state: &State, x: usize, y: usize) {
    let is_black = (x + y) % 2 == 1;
    let index = y * COL_COUNT + x;
    let is_selected = state.selected_index == Some(index as u8);

    let cell_block = Block::default()
        .style(if is_black {
            Style::new().bg(Color::DarkGray)
        } else {
            Style::new().bg(Color::Gray)
        })
        .borders(if is_selected {
            Borders::ALL
        } else {
            Borders::NONE
        })
        .border_style(Style::new().fg(Color::Yellow));

    let cell_inner = cell_block.inner(area);

    frame.render_widget(cell_block, area);

    if let Some(piece) = state.fields[index] {
        let piece_char = Paragraph::new(match piece {
            crate::state::PlayerPiece::PlayerOnePiece => "●",
            crate::state::PlayerPiece::PlayerTwoPiece => "○",
        })
        .style(match piece {
            crate::state::PlayerPiece::PlayerOnePiece => Style::new().fg(Color::Red),
            crate::state::PlayerPiece::PlayerTwoPiece => Style::new().fg(Color::Blue),
        })
        .alignment(Alignment::Center);

        frame.render_widget(piece_char, cell_inner);
    }
}

pub fn cell_index_at(area: Rect, x: u16, y: u16) -> Option<u8> {
    let layout = main_layout(area);

    let board_area = layout[1];

    let (_, container_inner, cell_width, cell_height) = board_layout(board_area);

    // Ignore clicks on the container border.
    if x < container_inner.x
        || x >= container_inner.x + container_inner.width
        || y < container_inner.y
        || y >= container_inner.y + container_inner.height
    {
        return None;
    }

    let rel_x = x - container_inner.x;
    let rel_y = y - container_inner.y;

    let col = (rel_x / cell_width) as usize;
    let row = (rel_y / cell_height) as usize;

    if col >= COL_COUNT || row >= ROW_COUNT {
        return None;
    }

    Some((row * COL_COUNT + col) as u8)
}

fn render_footer(frame: &mut Frame, area: Rect, state: &State) {
    let layout = main_layout(area);

    let turn_line = Line::from(vec![
        Span::raw("Current turn: "),
        Span::styled(
            match state.current_turn {
                PlayerTurn::PlayerOne => "Player one",
                PlayerTurn::PlayerTwo => "Player two",
            },
            match state.current_turn {
                PlayerTurn::PlayerOne => Style::new().fg(Color::Red),
                PlayerTurn::PlayerTwo => Style::new().fg(Color::Blue),
            },
        ),
    ]);

    let turn_paragraph = Paragraph::new(turn_line).alignment(HorizontalAlignment::Center);

    let legend_line = Line::from(vec![
        Span::raw("Legend: "),
        Span::styled("●", Style::new().fg(Color::Red)),
        Span::raw(" Player one, "),
        Span::styled("○", Style::new().fg(Color::Blue)),
        Span::raw(" Player two"),
    ]);

    let legend_paragraph = Paragraph::new(legend_line).alignment(HorizontalAlignment::Center);

    frame.render_widget(turn_paragraph, layout[0]);
    frame.render_widget(legend_paragraph, layout[1]);
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
