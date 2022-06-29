use super::SortComponent;
use crate::util::gen_rand_vec;
use genawaiter::{rc::gen, yield_};

struct Pointer(usize, usize);

impl Default for Pointer {
    fn default() -> Self {
        Self(0, 0)
    }
}

pub struct InsertionSort {
    iterator: Box<dyn Iterator<Item = (Vec<u64>, Pointer)>>,
    data: Vec<u64>,
    ptr: Pointer,
    is_done: bool,
}

impl<'a> InsertionSort {
    pub fn new(len: usize) -> Self {
        let data = gen_rand_vec(len);
        InsertionSort {
            data: data.clone(),
            iterator: iterator(data),
            ptr: Pointer(0, 0),
            is_done: false,
        }
    }
}

impl<'a> SortComponent<'a> for InsertionSort {
    fn as_str(&self) -> &'a str {
        "IntsertionSort"
    }
    fn shuffle(&mut self, len: usize) {
        self.ptr = Pointer::default();
        let data = gen_rand_vec(len);
        self.data = data.clone();
        self.iterator = iterator(data);
        self.is_done = false;
    }
    fn get_data(&self) -> Vec<(&'a str, u64)> {
        self.data
            .to_vec()
            .iter()
            .fold(Vec::new(), |mut data, value| {
                data.push(("", *value));
                data
            })
    }
    fn get_data_len(&self) -> usize {
        self.data.len()
    }
    fn is_sort(&self) -> bool {
        self.is_done
    }
    fn sort(&mut self) {}

    fn get_pointer(&self) -> Vec<(&'a str, u64)> {
        let len = self.data.len();
        let mut ptr = vec![("", 0); len];
        ptr[self.ptr.0].0 = "i";
        ptr[self.ptr.0].1 = 1;
        ptr[self.ptr.1].0 = "j";
        ptr[self.ptr.1].1 = 1;
        ptr
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
    Box::new(
        gen!({
            for i in 1..data.len() {
                let mut j = i;
                while j > 0 && data[j - 1] > data[j] {
                    data.swap(j - 1, j);
                    j -= 1;
                    yield_!((data.clone(), Pointer(i, j)));
                }
            }
        })
        .into_iter(),
    )
}
