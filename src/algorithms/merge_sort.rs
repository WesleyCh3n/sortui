use tui::style::{Color, Style};

use super::{gen_rand_vec, SortComponent};

#[derive(Default)]
struct Pointer(usize, usize, usize);

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
            ptr: Pointer(0, 0, 0),
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
    let mut curr = data.clone();
    mergesort(&mut data[..], &mut result, &mut curr, 0);
    Box::new(result.into_iter())
}

fn mergesort(
    arr: &mut [u64],
    state: &mut Vec<(Vec<u64>, Pointer)>,
    curr: &mut Vec<u64>,
    start: usize,
) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    mergesort(&mut arr[..mid], state, curr, start + 0); // 2
    mergesort(&mut arr[mid..], state, curr, start + mid);

    // Create an array to store intermediate result.
    let mut ret = arr.to_vec(); // 3

    // Merge the two piles.
    merge(&arr[..mid], &arr[mid..], &mut ret[..], state, curr, start);

    // Copy back the result back to original array.
    arr.copy_from_slice(&ret); // 5
}

fn merge(
    arr1: &[u64],
    arr2: &[u64],
    ret: &mut [u64],
    state: &mut Vec<(Vec<u64>, Pointer)>,
    curr: &mut Vec<u64>,
    start: usize,
) {
    let mut left = 0; // Head of left pile.
    let mut right = 0; // Head of right pile.
    let mut index = 0;

    // Compare element and insert back to result array.
    while left < arr1.len() && right < arr2.len() {
        if arr1[left] <= arr2[right] {
            curr[start + index] = arr1[left];
            ret[index] = arr1[left];
            index += 1;
            left += 1;
        } else {
            curr[start + index] = arr2[right];
            ret[index] = arr2[right];
            index += 1;
            right += 1;
        }
        // add state and pointer
        state.push((
            curr.clone(),
            Pointer(
                start + index,
                start + left,
                start + ret.len() / 2 - 1 + right,
            ),
        ));
    }

    // Copy the reset elements to returned array.
    // `memcpy` may be more performant than for-loop assignment.
    if left < arr1.len() {
        curr[start + index..start + index + arr1[left..].len()]
            .copy_from_slice(&arr1[left..]);
        ret[index..].copy_from_slice(&arr1[left..]);
        state.push((
            curr.clone(),
            Pointer(
                start + index,
                start + left,
                start + ret.len() / 2 - 1 + right,
            ),
        ));
    }
    if right < arr2.len() {
        curr[start + index..start + index + arr2[right..].len()]
            .copy_from_slice(&arr2[right..]);
        ret[index..].copy_from_slice(&arr2[right..]);
        state.push((
            curr.clone(),
            Pointer(
                start + index,
                start + left,
                start + ret.len() / 2 - 1 + right,
            ),
        ));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::*;
        let mut data = gen_rand_vec(12);
        let mut result = vec![];
        let mut curr = data.clone();
        mergesort(&mut data[..], &mut result, &mut curr, 0);
    }
}
