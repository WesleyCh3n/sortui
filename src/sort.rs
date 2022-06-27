#[derive(Clone)]
pub enum SortMethod {
    BubbleSort,
    SelectionSort,
}

impl<'a> SortMethod {
    pub fn as_str(&self) -> &'a str {
        match self {
            SortMethod::BubbleSort => "BubbleSort",
            SortMethod::SelectionSort => "SelectionSort"
        }
    }

    /* pub fn selection_sort(app: &mut App<'_>) {
        let len = app.data.len();
        if app.i >= len {
            app.i = 0;
            app.j = 0;
            app.auto = false;
            app.sorted = true;
            return;
        }
        let mut temp = app.i;

        if app.j >= len {
            // next i loop
            app.j = app.i + 1;
            app.i += 1;
            return;
        }


        /* for i in 0..len {
            let mut temp = i;
            for j in (i + 1)..len {
                if arr[temp] > arr[j] {
                    temp = j;
                }
            }
            arr.swap(i, temp);
        } */
    } */
}
