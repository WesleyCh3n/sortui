use tui::style::Style;

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

use rand::prelude::*;

pub fn gen_rand_vec<'a>(n: usize) -> Vec<u64> {
    let start = 5;
    let mut values: Vec<u64> = (start..n as u64 + start).collect();
    let mut rng = rand::thread_rng();
    values.shuffle(&mut rng);

    values
}
