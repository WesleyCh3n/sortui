use crate::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        self, BarChart, Block, BorderType, Borders, Clear, List, ListItem,
        Paragraph,
    },
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    app.fwidth = (size.width as usize - 4) / 2;
    if app.sort_component.get_data_len() > (size.width as usize - 4) / 2 {
        app.sort_component.shuffle((size.width as usize - 4) / 2);
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Length(3),
                Constraint::Percentage(40),
                Constraint::Length(1),
                Constraint::Percentage(27),
            ]
            .as_ref(),
        )
        .split(size);

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

    // SortMethod
    let auto_lambda = |c| {
        if app.auto {
            Color::Gray
        } else {
            c
        }
    };
    let border_style = Style::default().fg(auto_lambda(Color::Reset));
    let title_style = Style::default()
        .fg(auto_lambda(Color::Cyan))
        .add_modifier(Modifier::BOLD);
    let content_style = Style::default().fg(auto_lambda(Color::Reset));
    let menu = Paragraph::new(Span::raw(app.sort_component.as_str()))
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    "Sort Method [S]",
                    title_style,
                )))
                .borders(Borders::ALL)
                .border_style(border_style)
                .border_type(BorderType::Rounded),
        )
        .style(content_style)
        .alignment(Alignment::Center);
    f.render_widget(menu, menus[0]);

    // Data Length
    let menu = Paragraph::new(Span::raw(
        app.sort_component.get_data_len().to_string(),
    ))
    .block(
        Block::default()
            .title(Spans::from(Span::styled("Data Length [J/K]", title_style)))
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded),
    )
    .style(content_style)
    .alignment(Alignment::Center);
    f.render_widget(menu, menus[1]);

    // Tick Rate
    let menu = Paragraph::new(Span::raw(app.tick_rate.to_string()))
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    "Tick Rate [H/L]",
                    title_style,
                )))
                .borders(Borders::ALL)
                .border_style(border_style)
                .border_type(BorderType::Rounded),
        )
        .style(content_style)
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
    let bar_style = Style::default().fg(if app.sort_component.is_sort() {
        Color::Green
    } else {
        Color::Yellow
    });
    let barchart = BarChart::default()
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    "Graph",
                    Style::default()
                        .fg(if app.auto { Color::Blue } else { Color::Cyan })
                        .add_modifier(Modifier::BOLD),
                )))
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .border_type(BorderType::Rounded)
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
        .bar_style(bar_style)
        .value_style(bar_style);
    f.render_widget(barchart, graph[0]);

    let pointer = app.sort_component.get_pointer();

    let ptr_chart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .border_type(BorderType::Rounded)
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
        Paragraph::new(Span::styled(
            "Enter: start/stop sorting, Space: next step, R: shuffle data",
            Style::default().fg(Color::LightBlue),
        ))
        .alignment(Alignment::Left),
        chunks[3],
    );

    f.render_widget(
        Paragraph::new(Span::raw(format!(
            "Debug: fsize: {:?} vec: {:?}",
            f.size(),
            app.sort_component
                .get_data()
                .iter()
                .map(|e| { e.1 })
                .collect::<Vec<u64>>()
        )))
        .wrap(widgets::Wrap { trim: false }),
        chunks[4],
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
    let items: Vec<ListItem> =
        vec!["BubbleSort", "SelectionSort", "InsertionSort"]
            .iter()
            .enumerate()
            .map(|e| {
                ListItem::new(Spans::from(vec![
                    Span::styled(
                        format!("[{}] ", e.0 + 1),
                        Style::default()
                            .fg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{}", e.1),
                        Style::default()
                            .add_modifier(Modifier::BOLD | Modifier::ITALIC),
                    ),
                ]))
            })
            .collect();

    let sort_list = List::new(items)
        .block(
            Block::default()
                .title(Spans::from(Span::styled(
                    "Sort Method",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White));

    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(sort_list, area);
}
