use std::io;

use crossterm::event::KeyCode;

use crate::{
    algorithms::{
        bubble_sort::BubbleSort, insertion_sort::InsertionSort,
        merge_sort::MergeSort, quick_sort::QuickSort,
        selection_sort::SelectionSort,
    },
    app::PopUp,
    AppState,
};

pub fn handle_key_event(
    app: &mut AppState,
    key_code: KeyCode,
) -> io::Result<()> {
    if app.is_auto_sorting {
        match key_code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Enter => app.is_auto_sorting = false,
            _ => {}
        }
        return Ok(());
    }

    match &app.ui_state.popup {
        Some(s) => {
            match s {
                PopUp::SortAlgo => match key_code {
                    KeyCode::Char('1') => {
                        let len = app.sort_component.get_data_len();
                        app.sort_component = Box::new(BubbleSort::new(len));
                    }
                    KeyCode::Char('2') => {
                        let len = app.sort_component.get_data_len();
                        app.sort_component = Box::new(SelectionSort::new(len));
                    }
                    KeyCode::Char('3') => {
                        let len = app.sort_component.get_data_len();
                        app.sort_component = Box::new(InsertionSort::new(len));
                    }
                    KeyCode::Char('4') => {
                        let len = app.sort_component.get_data_len();
                        app.sort_component = Box::new(MergeSort::new(len));
                    }
                    KeyCode::Char('5') => {
                        let len = app.sort_component.get_data_len();
                        app.sort_component = Box::new(QuickSort::new(len));
                    }
                    _ => {}
                },
            }
            app.ui_state.popup = None;
        }
        None => match key_code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Enter => app.is_auto_sorting = true,
            KeyCode::Char('s') => {
                app.ui_state.popup = match app.ui_state.popup {
                    Some(_) => None,
                    None => Some(PopUp::SortAlgo),
                }
            }
            KeyCode::Char(' ') => {
                if app.sort_component.is_sort() {
                    return Ok(());
                }
                app.sort_component.iter();
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                let len = app.sort_component.get_data_len();
                app.sort_component.shuffle(len);
            }
            KeyCode::Char('k') => {
                let len = app.sort_component.get_data_len() + 1;
                if len > app.ui_state.width {
                    return Ok(());
                }
                app.sort_component.shuffle(len);
            }
            KeyCode::Char('j') => {
                let len = app.sort_component.get_data_len() - 1;
                if len < 2 {
                    return Ok(());
                }
                app.sort_component.shuffle(len);
            }
            KeyCode::Char('l') => {
                app.tick_rate += 5;
            }
            KeyCode::Char('h') => {
                if app.tick_rate == 0 {
                    return Ok(());
                }
                app.tick_rate -= 5;
            }
            _ => {}
        },
    }
    Ok(())
}
