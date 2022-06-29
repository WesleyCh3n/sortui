use super::SortComponent;
use crate::util::gen_rand_vec;
use genawaiter::{rc::gen, yield_};

#[derive(Default)]
struct Pointer(usize, usize);

pub struct MergeSort {
    iterator: Box<dyn Iterator<Item = (Vec<u64>, Pointer)>>,
    data: Vec<u64>,
    ptr: Pointer,
    is_done: bool,
}

impl<'a> MergeSort {
    pub fn new(len: usize) -> Self {
        let data = gen_rand_vec(len);
        MergeSort {
            data: data.clone(),
            iterator: iterator(data),
            ptr: Pointer(0, 0),
            is_done: false,
        }
    }
}

impl<'a> SortComponent<'a> for MergeSort {
    fn as_str(&self) -> &'a str {
        "MergeSort"
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

    fn get_pointer(&self) -> Vec<(&'a str, u64)> {
        let len = self.data.len();
        let mut ptr = vec![("", 0); len];
        ptr[self.ptr.0].0 = "s";
        ptr[self.ptr.0].1 = 1;
        ptr[self.ptr.1].0 = "m";
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
            // main func
            let mut width = 1;
            let mut ret = data.to_vec();
            let len = data.len();

            while width < len {
                let mut i = 0;
                while i < len {
                    // Check to avoid upper bound and middle index out of bound.
                    let upper = std::cmp::min(i + 2 * width, len);
                    let mid = std::cmp::min(i + width, len);

                    {
                        let mut left = 0; // Head of left pile.
                        let mut right = 0; // Head of right pile.
                        let mut index = 0;

                        // Compare element and insert back to result array.
                        while left < data[i..mid].len()
                            && right < data[mid..upper].len()
                        {
                            if data[i..mid][left] <= data[mid..upper][right] {
                                ret[i..upper][index] = data[i..mid][left];
                                index += 1;
                                left += 1;
                            } else {
                                ret[i..upper][index] = data[mid..upper][right];
                                index += 1;
                                right += 1;
                            }
                            yield_!((data.clone(), Pointer(i, i + index)));
                        }

                        // Copy the reset elements to returned array.
                        // `memcpy` may be more performant than for-loop assignment.
                        if left < data[i..mid].len() {
                            ret[i..upper][index..]
                                .copy_from_slice(&data[i..mid][left..]);
                        }
                        if right < data[mid..upper].len() {
                            ret[i..upper][index..]
                                .copy_from_slice(&data[mid..upper][right..]);
                        }
                        yield_!((data.clone(), Pointer(i, i + index)));
                    }

                    // Copy the merged result back to original array.
                    data[i..upper].copy_from_slice(&ret[i..upper]);

                    yield_!((
                        data.clone(),
                        Pointer(i, if mid < data.len() { mid } else { 0 })
                    ));
                    // Increase start index to merge next two subsequences.
                    i += 2 * width;
                }
                width *= 2;
            }
            yield_!((data.clone(), Pointer::default()));
        })
        .into_iter(),
    )
}
