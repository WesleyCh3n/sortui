use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event};
use tui::{backend::Backend, Terminal};

use crate::{
    algorithms::{bubble_sort::BubbleSort, SortComponent},
    input_handler::handle_key_event,
    ui::ui,
};

#[derive(Clone)]
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
                // auto mode
                true => {
                    let last_tick = Instant::now();
                    if event::poll(Duration::from_millis(self.tick_rate))? {
                        if let Event::Key(key) = event::read()? {
                            handle_key_event(self, key.code)?;
                        }
                    }
                    if last_tick.elapsed()
                        >= Duration::from_millis(self.tick_rate)
                    {
                        if self.sort_component.is_sort() {
                            self.is_auto_sorting = false;
                        } else {
                            self.sort_component.iter();
                        }
                    }
                }
                // normal mode
                false => {
                    if let Event::Key(key) = event::read()? {
                        handle_key_event(self, key.code)?
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
}
