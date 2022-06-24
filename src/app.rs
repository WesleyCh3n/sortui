use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use tui::{backend::Backend, Terminal};

use crate::{sort::SortMethod, ui::ui, util::gen_rand_data};

pub struct App<'a> {
    pub data: Vec<(&'a str, u64)>,
    pub sort_method: SortMethod,
    pub sort_popup: bool,
    pub auto: bool,
    pub sorted: bool,
    pub tick_rate: u64,
    pub i: usize,
    pub j: usize,
    pub is_quit: bool,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            data: gen_rand_data(45),
            sort_method: SortMethod::BubbleSort,
            sort_popup: false,
            auto: false,
            sorted: false,
            tick_rate: 50,
            i: 0,
            j: 0,
            is_quit: false,
        }
    }

    pub fn reset_graph(&mut self) {
        (self.i, self.j) = (0, 0);
        self.sorted = false;
    }

    pub fn event(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('q') => self.is_quit = true,
            KeyCode::Enter => self.auto = true,
            KeyCode::Char('s') => self.sort_popup = !self.sort_popup,
            KeyCode::Char('n') => {
                if self.sorted {
                    return Ok(());
                }
                SortMethod::sort(self.sort_method.clone(), self);
            }
            KeyCode::Char('r') => {
                self.data = gen_rand_data(self.data.len());
                self.reset_graph()
            }
            KeyCode::Up => {
                self.data = gen_rand_data(self.data.len() + 1);
                self.reset_graph()
            }
            KeyCode::Down => {
                self.data = gen_rand_data(self.data.len() - 1);
                self.reset_graph()
            }
            _ => {}
        }
        Ok(())
    }

    pub fn auto_event(&mut self) -> io::Result<()> {
        let last_tick = Instant::now();
        let tick_rate = Duration::from_millis(self.tick_rate);
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.is_quit = true,
                    KeyCode::Enter => self.auto = false,
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            if self.sorted {
                return Ok(());
            }
            SortMethod::sort(self.sort_method.clone(), self);
        }
        Ok(())
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui(f, &self))?;

            if self.auto {
                self.auto_event()?
            } else {
                if let Event::Key(key) = event::read()? {
                    if self.sort_popup {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Char('s') => {
                                self.sort_popup = false
                            }
                            KeyCode::Char('1') => {
                                self.sort_method = SortMethod::BubbleSort;
                                (self.i, self.j) = (0, 0);
                                // initial state
                                // pointer follow which variable
                            }
                            KeyCode::Char('2') => {
                                self.sort_method = SortMethod::SelectionSort;
                                (self.i, self.j) = (0, 1);
                            }
                            _ => {}
                        }
                        self.sort_popup = false;
                    } else {
                        self.event(key)?
                    }
                }
            }

            if self.is_quit {
                break;
            }
        }
        Ok(())
    }
}
