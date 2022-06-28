use super::SortComponent;
use crate::util::gen_rand_data;

#[derive(Debug)]
pub struct SelectionSort<'a> {
    pub data: Vec<(&'a str, u64)>,
    i: usize,
    j: usize,
    tmp_index: usize,
    is_done: bool,
}

impl<'a> SelectionSort<'a> {
    pub fn new(len: usize) -> Self {
        SelectionSort {
            data: gen_rand_data(len),
            i: 0,
            j: 1,
            tmp_index: 0,
            is_done: false,
        }
    }
}

impl<'a> SortComponent<'a> for SelectionSort<'a> {
    fn as_str(&self) -> &'a str {
        "SelectionSort"
    }
    fn shuffle(&mut self, len: usize) {
        (self.i, self.j, self.tmp_index) = (0, 1, 0);
        self.data = gen_rand_data(len);
        self.is_done = false;
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
        /* let len = arr.len();
        for i in 0..len {
            let mut temp = i;
            for j in (i + 1)..len {
                if arr[temp] > arr[j] {
                    temp = j;
                }
            }
            arr.swap(i, temp);
        } */
        let len = self.data.len();
        if self.i >= len {
            self.i = 0;
            self.j = 1;
            self.is_done = true;
            return;
        }

        if self.i == 0 && self.j == 1 {
            self.tmp_index = self.i;
        }
        if self.j >= len {
            self.data.swap(self.i, self.tmp_index);
            self.j = self.i + 1;
            self.i += 1;
            self.tmp_index = self.i;
            return;
        }

        if self.data[self.tmp_index].1 > self.data[self.j].1 {
            self.tmp_index = self.j;
        }

        self.j += 1;
    }
    fn get_pointer(&self) -> Vec<(&'a str, u64)> {
        let mut pointer = vec![("", 0); self.data.len()];
        let len = self.data.len();
        if self.tmp_index < len {
            pointer[self.tmp_index].0 = "tmp";
            pointer[self.tmp_index].1 = 1;
        }
        if self.i < len {
            pointer[self.i].0 = "i";
            pointer[self.i].1 = 1;
        }
        if self.j < len {
            pointer[self.j].0 = "j";
            pointer[self.j].1 = 1;
        }
        pointer
    }
}
