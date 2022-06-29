pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;

pub trait SortComponent<'a> {
    fn as_str(&self) -> &'a str;

    fn get_data(&self) -> Vec<(&'a str, u64)>;
    fn get_data_len(&self) -> usize;
    fn is_sort(&self) -> bool;

    fn sort(&mut self);
    fn get_pointer(&self) -> Vec<(&'a str, u64)>;

    fn shuffle(&mut self, len: usize);

    fn iter(&mut self);
}
