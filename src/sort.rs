use crate::app::App;

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

    pub fn sort(self, app: &mut App<'_>) {
        match self {
            SortMethod::BubbleSort => SortMethod::bubble_sort(app),
            SortMethod::SelectionSort => SortMethod::selection_sort(app)
        }
    }

    pub fn bubble_sort(app: &mut App<'_>) {
        /*
        // this extract this two for loop into step
        for i in 0..self.data.len() {
        for j in 0..self.data.len() - 1 - i {}
        }
        */
        if app.i >= app.data.len() {
            // loop finished
            app.i = 0;
            app.j = 0;
            app.auto = false;
            app.sorted = true;
            return;
        }
        if app.j >= app.data.len() - 1 - app.i {
            // next i loop
            app.j = 0;
            app.i += 1;
            return;
        }
        if app.data[app.j].1 > app.data[app.j + 1].1 {
            app.data.swap(app.j, app.j + 1);
        }
        app.j += 1;
    }

    pub fn selection_sort(app: &mut App<'_>) {
        todo!();
        /* let len = app.data.len();
        if app.i >= len {
            app.i = 0;
            app.j = 0;
            app.auto = false;
            app.sorted = true;
        }

        if app.j >= len {
            // next i loop
            app.j = app.i + 1;
            app.i += 1;
            return;
        }


        for i in 0..len {
            let mut temp = i;
            for j in (i + 1)..len {
                if arr[temp] > arr[j] {
                    temp = j;
                }
            }
            arr.swap(i, temp);
        } */
    }
}
