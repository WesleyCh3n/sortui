use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{
    components::{bubble_sort::BubbleSort, SortComponent},
    ui::ui,
};

pub enum PopUp {
    SortAlgo,
}

pub struct UiState {
    pub width: usize,
    pub first_time: bool,
    pub popup: Option<PopUp>,
}

pub struct AppState<'a> {
    pub is_auto_sorting: bool,
    pub tick_rate: u64,
    pub is_quit: bool,
    pub sort_component: Box<dyn SortComponent<'a>>,
    pub ui_state: UiState,
}

impl AppState<'static> {
    pub fn new() -> AppState<'static> {
        AppState {
            sort_component: Box::new(BubbleSort::new(45)),

            is_auto_sorting: false,
            tick_rate: 10,

            is_quit: false,
            ui_state: UiState {
                width: 0,
                first_time: true,
                popup: None,
            },
        }
    }
}

impl AppState<'_> {
    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        loop {
            // drawing the ui
            terminal.draw(|f| ui(f, self))?;

            // handle input event
            match self.is_auto_sorting {
                true => self.run_auto_sort()?,
                false => {
                    if let Event::Key(key) = event::read()? {
                        self.input_handler(key.code)?
                    }
                }
            }

            // quit the program
            if self.is_quit {
                break;
            }
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.is_quit = true;
    }

    pub fn run_auto_sort(&mut self) -> io::Result<()> {
        let last_tick = Instant::now();
        let tick_rate = Duration::from_millis(self.tick_rate);
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Enter => self.is_auto_sorting = false,
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            if self.sort_component.is_sort() {
                self.is_auto_sorting = false;
                return Ok(());
            }
            self.sort_component.iter();
        }
        Ok(())
    }
}
