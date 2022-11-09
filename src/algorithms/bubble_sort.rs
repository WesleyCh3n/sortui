use tui::style::{Color, Style};

use super::SortComponent;
use crate::util::gen_rand_vec;

#[derive(Default)]
struct Pointer(usize, usize);

pub struct BubbleSort {
    iterator: Box<dyn Iterator<Item = (Vec<u64>, Pointer)>>,
    data: Vec<u64>,
    ptr: Pointer,
    is_done: bool,
}

impl<'a> BubbleSort {
    pub fn new(len: usize) -> Self {
        let data = gen_rand_vec(len);
        BubbleSort {
            data: data.clone(),
            iterator: iterator(data),
            ptr: Pointer(0, 0),
            is_done: false,
        }
    }
}

impl<'a> SortComponent<'a> for BubbleSort {
    fn as_str(&self) -> &'a str {
        "BubbleSort"
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
        } else {
            self.is_done = true;
        }
    }
}

fn iterator(
    mut data: Vec<u64>,
) -> Box<dyn Iterator<Item = (Vec<u64>, Pointer)>> {
    let mut result = vec![];
    for i in 0..data.len() {
        for j in 0..data.len() - 1 - i {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
            }
            result.push((data.clone(), Pointer(j, j + 1)));
        }
    }
    Box::new(result.into_iter())
}
