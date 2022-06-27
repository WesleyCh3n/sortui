use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use tui::{backend::Backend, Terminal};

use crate::{
    components::{bubble_sort::BubbleSort, SortComponent, selection_sort::SelectionSort},
    sort::SortMethod,
    ui::ui,
};

pub struct App<'a> {
    pub sort_method: SortMethod,
    pub sort_popup: bool,
    pub auto: bool,
    pub tick_rate: u64,
    pub is_quit: bool,
    pub sort_component: Box<dyn SortComponent<'a>>,
}

impl App<'static> {
    pub fn new() -> App<'static> {
        App {
            sort_method: SortMethod::BubbleSort,
            sort_popup: false,
            auto: false,
            tick_rate: 50,
            is_quit: false,
            sort_component: Box::new(BubbleSort::new(45)),
        }
    }

    pub fn event(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('q') => self.is_quit = true,
            KeyCode::Enter => self.auto = true,
            KeyCode::Char('s') => self.sort_popup = !self.sort_popup,
            KeyCode::Char('n') => {
                if self.sort_component.is_sort() {
                    return Ok(());
                }
                self.sort_component.sort();
            }
            KeyCode::Char('r') => {
                let len = self.sort_component.get_data_len();
                self.sort_component.shuffle(len);
            }
            KeyCode::Up => {
                let len = self.sort_component.get_data_len() + 1;
                self.sort_component.shuffle(len);
            }
            KeyCode::Down => {
                let len = self.sort_component.get_data_len() - 1;
                self.sort_component.shuffle(len);
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
            if self.sort_component.is_sort() {
                return Ok(());
            }
            self.sort_component.sort();
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
                                let len = self.sort_component.get_data_len();
                                self.sort_component = Box::new(BubbleSort::new(len));
                            }
                            KeyCode::Char('2') => {
                                self.sort_method = SortMethod::SelectionSort;
                                let len = self.sort_component.get_data_len();
                                self.sort_component = Box::new(SelectionSort::new(len));
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
