use tui::style::Style;

// pub mod barchart;
pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

pub trait SortComponent<'a> {
    fn as_str(&self) -> &'a str;

    fn get_data(&self) -> Vec<(&'a str, u64, Option<Style>)>;
    fn get_data_len(&self) -> usize;
    fn is_sort(&self) -> bool;

    fn shuffle(&mut self, len: usize);

    fn iter(&mut self);
}
