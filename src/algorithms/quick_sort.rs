#![allow(unused_mut)] // TODO: remove this
use tui::style::{Color, Style};

use super::{gen_rand_vec, SortComponent};

#[derive(Default)]
struct Pointer(usize, usize);

pub struct QuickSort {
    iterator: Box<dyn Iterator<Item = (Vec<u64>, Pointer)>>,
    data: Vec<u64>,
    ptr: Pointer,
    is_done: bool,
}

impl<'a> QuickSort {
    pub fn new(len: usize) -> Self {
        let mut data = gen_rand_vec(len);
        QuickSort {
            data: gen_rand_vec(len),
            iterator: iterator(data, 1, len - 1),
            ptr: Pointer(0, 0),
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
        let mut data = gen_rand_vec(len);
        self.data = data.clone();
        self.iterator = iterator(data, 1, len - 1);
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
        data[self.ptr.1].2 = Some(Style::default().fg(Color::LightRed));
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
            println!("{} {}", self.ptr.0, self.ptr.1);
        } else {
            self.is_done = true;
        }
    }
}

#[allow(unused)]
fn iterator(
    mut data: Vec<u64>,
    high: usize,
    low: usize,
) -> Box<dyn Iterator<Item = (Vec<u64>, Pointer)>> {
    // Box::new(vec![].into_iter());
    unimplemented!()
}
