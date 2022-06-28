use crate::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{BarChart, Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(27),
                Constraint::Length(3),
                Constraint::Percentage(40),
                Constraint::Percentage(27),
            ]
            .as_ref(),
        )
        .split(f.size());

    let menus = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let menu = Paragraph::new(Span::raw(app.sort_component.as_str()))
        .block(
            Block::default()
                .title("[S]ort Method")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(menu, menus[0]);
    let menu = Paragraph::new(Span::raw(
        app.sort_component.get_data_len().to_string(),
    ))
    .block(
        Block::default()
            .title("[D]ata Length")
            .borders(Borders::ALL),
    )
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Center);
    f.render_widget(menu, menus[1]);
    let menu = Paragraph::new(Span::raw(app.tick_rate.to_string()))
        .block(Block::default().title("[T]ick Rate").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(menu, menus[2]);

    let graph = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [Constraint::Percentage(83), Constraint::Percentage(17)].as_ref(),
        )
        .split(chunks[2]);

    let data = app.sort_component.get_data();
    let barchart = BarChart::default()
        .block(
            Block::default()
                .title("Graph")
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(if app.auto {
                    Color::Blue
                } else {
                    if app.sort_component.is_sort() {
                        Color::Green
                    } else {
                        Color::Reset
                    }
                })),
        )
        .data(&data)
        .bar_width(1)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().bg(Color::Yellow));
    f.render_widget(barchart, graph[0]);

    let pointer = app.sort_component.get_pointer();

    let ptr_chart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .border_style(Style::default().fg(if app.auto {
                    Color::Blue
                } else {
                    if app.sort_component.is_sort() {
                        Color::Green
                    } else {
                        Color::Reset
                    }
                })),
        )
        .data(&pointer)
        .bar_width(1)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().bg(Color::Green));
    f.render_widget(ptr_chart, graph[1]);

    f.render_widget(
        Paragraph::new(Span::raw(format!("Debug: {:#?}", app.sort_component.is_sort()))),
        chunks[3],
    );

    if app.sort_popup {
        popup_ui(f);
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn popup_ui<B: Backend>(f: &mut Frame<B>) {
    // let block = Block::default().title("Popup").borders(Borders::ALL);
    let items = [
        ListItem::new(Spans::from(vec![
            Span::styled(
                "[1] ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "BubbleSort",
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])),
        ListItem::new(Spans::from(vec![
            Span::styled(
                "[2] ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "SelectionSort",
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])),
    ];
    let sort_list = List::new(items)
        .block(Block::default().title("Sort Method").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(sort_list, area);
}
