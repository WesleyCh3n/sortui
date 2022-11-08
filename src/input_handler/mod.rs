use std::io;

use crossterm::event::KeyCode;

use crate::{
    app::PopUp,
    components::{
        bubble_sort::BubbleSort, insertion_sort::InsertionSort,
        merge_sort::MergeSort, quick_sort::QuickSort,
        selection_sort::SelectionSort,
    },
    AppState,
};

impl AppState<'_> {
    pub fn input_handler(&mut self, key_code: KeyCode) -> io::Result<()> {
        match &self.ui_state.popup {
            Some(s) => {
                match s {
                    PopUp::SortAlgo => match key_code {
                        KeyCode::Char('1') => {
                            let len = self.sort_component.get_data_len();
                            self.sort_component =
                                Box::new(BubbleSort::new(len));
                        }
                        KeyCode::Char('2') => {
                            let len = self.sort_component.get_data_len();
                            self.sort_component =
                                Box::new(SelectionSort::new(len));
                        }
                        KeyCode::Char('3') => {
                            let len = self.sort_component.get_data_len();
                            self.sort_component =
                                Box::new(InsertionSort::new(len));
                        }
                        KeyCode::Char('4') => {
                            let len = self.sort_component.get_data_len();
                            self.sort_component = Box::new(MergeSort::new(len));
                        }
                        KeyCode::Char('5') => {
                            let len = self.sort_component.get_data_len();
                            self.sort_component = Box::new(QuickSort::new(len));
                        }
                        _ => {}
                    },
                }
                self.ui_state.popup = None;
            }
            None => match key_code {
                KeyCode::Char('q') => self.quit(),
                KeyCode::Enter => self.is_auto_sorting = true,
                KeyCode::Char('s') => {
                    self.ui_state.popup = match self.ui_state.popup {
                        Some(_) => None,
                        None => Some(PopUp::SortAlgo),
                    }
                }
                KeyCode::Char(' ') => {
                    if self.sort_component.is_sort() {
                        return Ok(());
                    }
                    self.sort_component.iter();
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    let len = self.sort_component.get_data_len();
                    self.sort_component.shuffle(len);
                }
                KeyCode::Char('k') => {
                    let len = self.sort_component.get_data_len() + 1;
                    if len > self.ui_state.width {
                        return Ok(());
                    }
                    self.sort_component.shuffle(len);
                }
                KeyCode::Char('j') => {
                    let len = self.sort_component.get_data_len() - 1;
                    if len < 2 {
                        return Ok(());
                    }
                    self.sort_component.shuffle(len);
                }
                KeyCode::Char('l') => {
                    self.tick_rate += 5;
                }
                KeyCode::Char('h') => {
                    if self.tick_rate == 0 {
                        return Ok(());
                    }
                    self.tick_rate -= 5;
                }
                _ => {}
            },
        }
        Ok(())
    }
}
