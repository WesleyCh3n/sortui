use crate::components::barchart::BarChart;
use crate::AppState;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let size = f.size();
    app.ui_state.width = (size.width as usize - 4) / 2;
    if app.sort_component.get_data_len() > (size.width as usize - 4) / 2 {
        app.sort_component.shuffle((size.width as usize - 4) / 2);
    }
    // fill up the data to match the ui width
    if app.ui_state.first_time {
        app.ui_state.first_time = false;
        app.sort_component.shuffle((size.width as usize - 4) / 2);
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(15), // top margin
                Constraint::Length(3),      // top widgets
                Constraint::Percentage(55), // barchart
                Constraint::Length(1),      // bottom hint
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
        if app.is_auto_sorting {
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
                .title(Spans::from(Span::styled("Sort Method [S]", title_style)))
                .borders(Borders::ALL)
                .border_style(border_style)
                .border_type(BorderType::Rounded),
        )
        .style(content_style)
        .alignment(Alignment::Center);
    f.render_widget(menu, menus[0]);

    // Data Length
    let menu = Paragraph::new(Span::raw(app.sort_component.get_data_len().to_string()))
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
                .title(Spans::from(Span::styled("Tick Rate [H/L]", title_style)))
                .borders(Borders::ALL)
                .border_style(border_style)
                .border_type(BorderType::Rounded),
        )
        .style(content_style)
        .alignment(Alignment::Center);
    f.render_widget(menu, menus[2]);

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
                        .fg(if app.is_auto_sorting {
                            Color::Blue
                        } else {
                            Color::Cyan
                        })
                        .add_modifier(Modifier::BOLD),
                )))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if app.is_auto_sorting {
                    Color::Blue
                } else if app.sort_component.is_sort() {
                    Color::Green
                } else {
                    Color::Reset
                })),
        )
        .data(&data)
        .bar_width(1)
        .bar_gap(1)
        .bar_style(bar_style)
        .value_style(bar_style);
    f.render_widget(barchart, chunks[2]);

    f.render_widget(
        Paragraph::new(Span::styled(
            "[Enter]: start/stop sorting, [Space]: next step, [R]: shuffle data",
            Style::default().fg(Color::LightBlue),
        ))
        .alignment(Alignment::Left),
        chunks[3],
    );

    if let Some(s) = &app.ui_state.popup {
        match s {
            crate::app::PopUp::SortAlgo => popup_ui(f),
        }
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
    let items: Vec<ListItem> = vec![
        "BubbleSort",
        "SelectionSort",
        "InsertionSort",
        "MergeSort",
        "QuickSort",
    ]
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
                e.1.to_string(),
                Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC),
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
