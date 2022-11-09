use tui::style::{Color, Style};

use super::{gen_rand_vec, SortComponent};

#[derive(Default)]
struct Pointer(usize, usize, usize);

pub struct QuickSort {
    iterator: Box<dyn Iterator<Item = (Vec<u64>, Pointer)>>,
    data: Vec<u64>,
    ptr: Pointer,
    is_done: bool,
}

impl<'a> QuickSort {
    pub fn new(len: usize) -> Self {
        let data = gen_rand_vec(len);
        QuickSort {
            data: data.clone(),
            iterator: iterator(data),
            ptr: Pointer(0, 0, 0),
            is_done: false,
        }
    }
}

impl<'a> SortComponent<'a> for QuickSort {
    fn as_str(&self) -> &'a str {
        "QuickSort"
    }
    fn shuffle(&mut self, len: usize) {
        self.ptr = Pointer::default();
        let data = gen_rand_vec(len);
        self.data = data.clone();
        self.iterator = iterator(data);
        self.is_done = false;
    }
    fn get_data(&self) -> Vec<(&'a str, u64, Option<Style>)> {
        let mut data =
            self.data
                .to_vec()
                .iter()
                .fold(Vec::new(), |mut data, value| {
                    data.push(("", *value, None));
                    data
                });
        data[self.ptr.0].2 = Some(Style::default().fg(Color::LightRed));
        data[self.ptr.1].2 = Some(Style::default().fg(Color::LightBlue));
        data[self.ptr.2].2 = Some(Style::default().fg(Color::LightBlue));
        data
    }
    fn get_data_len(&self) -> usize {
        self.data.len()
    }
    fn is_sort(&self) -> bool {
        self.is_done
    }

    fn iter(&mut self) {
        if let Some((data, ptr)) = self.iterator.next() {
            self.data = data;
            self.ptr = ptr;
        } else {
            self.is_done = true;
        }
    }
}

fn iterator(
    mut data: Vec<u64>,
) -> Box<dyn Iterator<Item = (Vec<u64>, Pointer)>> {
    let mut result = vec![];
    let len = data.len();
    quicksort(&mut data, 0, len as isize - 1, &mut result);
    Box::new(result.into_iter())
}

fn quicksort(
    arr: &mut Vec<u64>,
    low: isize,
    high: isize,
    state_helper: &mut Vec<(Vec<u64>, Pointer)>,
) {
    if low < high {
        let pivot = partition(arr, low, high, state_helper);
        quicksort(arr, low, pivot - 1, state_helper);
        quicksort(arr, pivot + 1, high, state_helper);
    }
}
fn partition(
    arr: &mut Vec<u64>,
    low: isize,
    high: isize,
    state_helper: &mut Vec<(Vec<u64>, Pointer)>,
) -> isize {
    // -- Determine the pivot --
    // In Lomuto parition scheme,
    // the latest element is always chosen as the pivot.
    let pivot = arr[high as usize];
    let mut i = low;

    // -- Swap elements --
    for j in low..high {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            state_helper.push((
                arr.clone(),
                Pointer(high as usize, i as usize, j as usize),
            ));
            i += 1;
        }
    }
    // Swap pivot to the middle of two piles.
    arr.swap(i as usize, high as usize);
    state_helper.push((
        arr.clone(),
        Pointer(high as usize, i as usize, high as usize),
    ));
    // Return the final index of the pivoti
    i
}
