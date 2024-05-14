use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Margin, Rect}, style::{Color, Style, Stylize}, text::Text, widgets::{Block, Borders, Paragraph}, Frame};

use crate::app::App;

use ratatui::{prelude::*, widgets::*};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(f.size().inner(&Margin::new(0, 1)));

    let title = Paragraph::new(Text::styled(
        "FileX",
        Style::default().fg(Color::White).bold(),
    ));

    f.render_widget(title.alignment(Alignment::Center), chunks[0]);

    let dir_block = Block::default()
        .borders(Borders::TOP);

    let dir_path = Paragraph::new(Text::styled(
        app.filesystem.current_path.to_str().unwrap().to_string(),
        Style::default().italic().white(),
    )).block(dir_block);

    f.render_widget(dir_path, chunks[2]);

    let tableChunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(chunks[1]);

    let widths = [Constraint::Length(100), Constraint::Length(100), Constraint::Length(100)];
    
    let header_rows = [
        Row::new(vec!["Name".bold(), "Type".bold(), "Size".bold()]),
    ];

    let header_table = Table::new(header_rows, widths);

    f.render_widget(header_table, tableChunks[0]);

    let mut table_state = TableState::default();
    table_state.select(Some(app.selected_item_index));
    
    let rows = [
        Row::new(vec!["Hosts", "", "2kb"]),
        Row::new(vec!["fr.txt", "txt", "1MB"]),
    ];
    let table = Table::new(rows, widths)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table, tableChunks[1], &mut table_state);


}

fn centered_rect(precent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - precent_x) / 2),
                Constraint::Percentage(precent_x),
                Constraint::Percentage((100 - precent_x) / 2),
            ])
            .split(popup_layout[1])[1]
}