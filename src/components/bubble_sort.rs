use super::SortComponent;
use crate::util::gen_rand_data;

#[derive(Debug)]
pub struct BubbleSort<'a> {
    pub data: Vec<(&'a str, u64)>,
    i: usize,
    j: usize,
    is_done: bool,
}

impl<'a> BubbleSort<'a> {
    pub fn new(len: usize) -> Self {
        BubbleSort {
            data: gen_rand_data(len),
            i: 0,
            j: 0,
            is_done: false,
        }
    }
}

impl<'a> SortComponent<'a> for BubbleSort<'a> {
    fn shuffle(&mut self, len: usize) {
        (self.i, self.j) = (0, 1);
        self.data = gen_rand_data(len);
    }
    fn get_data(&self) -> Vec<(&'a str, u64)> {
        self.data.to_vec()
    }
    fn get_data_len(&self) -> usize {
        self.data.len()
    }
    fn is_sort(&self) -> bool {
        self.is_done
    }
    fn sort(&mut self) {
        let len = self.data.len();
        if self.i >= len {
            self.i = 0;
            self.j = 0;
            self.is_done = true;
            return;
        }
        if self.j >= len - 1 - self.i {
            self.j = 0;
            self.i += 1;
        }
        if self.data[self.j].1 > self.data[self.j + 1].1 {
            self.data.swap(self.j, self.j + 1);
        }
        self.j += 1;
    }
    fn get_pointer(&self) -> Vec<(&'a str, u64)> {
        let mut pointer = vec![("", 0); self.data.len()];
        let len = self.data.len();
        pointer[self.j].0 = "j";
        pointer[self.j].1 = 1;
        if self.j > 0 && self.j < len {
            pointer[self.j - 1].0 = "j";
            pointer[self.j - 1].1 = 1;
        }
        pointer
    }
}
