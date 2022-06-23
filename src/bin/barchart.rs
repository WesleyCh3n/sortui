use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use rand::prelude::*;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{BarChart, Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

enum SortMethod {
    BubbleSort,
}

impl<'a> SortMethod {
    fn as_str(&self) -> &'a str {
        match self {
            SortMethod::BubbleSort => "BubbleSort",
        }
    }
}

struct App<'a> {
    data: Vec<(&'a str, u64)>,
    sort_method: SortMethod,
    sorting: bool,
    sorted: bool,
    tick_rate: u64,
    i: usize,
    j: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            data: gen_rand_data(30),
            sort_method: SortMethod::BubbleSort,
            sorting: false,
            sorted: false,
            tick_rate: 50,
            i: 0,
            j: 0,
        }
    }

    fn bubble_sort(&mut self) {
        /*
        // this extract this two for loop into step
        for i in 0..self.data.len() {
        for j in 0..self.data.len() - 1 - i {}
        }
        */
        if self.i >= self.data.len() {
            // loop finished
            self.i = 0;
            self.j = 0;
            self.sorting = false;
            self.sorted = true;
            return;
        }
        if self.j >= self.data.len() - 1 - self.i {
            // next i loop
            self.j = 0;
            self.i += 1;
            return;
        }
        if self.data[self.j].1 > self.data[self.j + 1].1 {
            self.data.swap(self.j, self.j + 1);
        }
        self.j += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;
        let tick_rate = Duration::from_millis(app.tick_rate);
        if app.sorting {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                    if let KeyCode::Enter = key.code {
                        app.sorting = false
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if !app.sorted {
                    match app.sort_method {
                        SortMethod::BubbleSort => app.bubble_sort(),
                    }
                }
                last_tick = Instant::now();
            }
        } else {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
                if let KeyCode::Enter = key.code {
                    app.sorting = true;
                }
                if let KeyCode::Char('n') = key.code {
                    if !app.sorted {
                        match app.sort_method {
                            SortMethod::BubbleSort => app.bubble_sort(),
                        }
                    }
                }
                if let KeyCode::Char('r') = key.code {
                    app.data = gen_rand_data(6);

                    // reset pointer
                    (app.i, app.j) = (0, 0);
                }
                if let KeyCode::Up = key.code {
                    app.data = gen_rand_data(app.data.len() + 1);

                    // reset pointer
                    (app.i, app.j) = (0, 0);
                }
                if let KeyCode::Down = key.code {
                    app.data = gen_rand_data(app.data.len() - 1);

                    // reset pointer
                    (app.i, app.j) = (0, 0);
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(27),
                Constraint::Length(3),
                Constraint::Percentage(30),
                Constraint::Percentage(10),
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

    let menu = Paragraph::new(Span::raw(app.sort_method.as_str()))
        .block(
            Block::default()
                .title("[S]ort Method")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(menu, menus[0]);
    let menu = Paragraph::new(Span::raw(app.data.len().to_string()))
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

    let barchart = BarChart::default()
        .block(
            Block::default()
                .title("Graph")
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(if app.sorting {
                    Color::Blue
                } else {
                    if app.sorted {
                        Color::Green
                    } else {
                        Color::Reset
                    }
                })),
        )
        .data(&app.data)
        .bar_width(2)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().bg(Color::Yellow));
    f.render_widget(barchart, chunks[2]);

    let mut pointer = vec![("", 0); app.data.len()];
    pointer[app.j].1 = 1;

    let barchart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .border_style(Style::default().fg(if app.sorting {
                    Color::Blue
                } else {
                    if app.sorted {
                        Color::Green
                    } else {
                        Color::Reset
                    }
                })),
        )
        .data(&pointer)
        .bar_width(2)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().bg(Color::Green));
    f.render_widget(barchart, chunks[3]);

    f.render_widget(
        Paragraph::new(Span::raw(format!("Debug: {:#?}", f.size()))),
        chunks[4],
    )
}

fn gen_rand_data<'a>(n: usize) -> Vec<(&'a str, u64)> {
    let start = 5;
    let mut values: Vec<u64> = (start..n as u64 + start).collect();
    let mut rng = rand::thread_rng();
    values.shuffle(&mut rng);

    values.iter().fold(Vec::new(), |mut data, value| {
        data.push(("", *value));
        data
    })
}
