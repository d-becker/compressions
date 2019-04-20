struct View<'a, T> {
    data: &'a Vec<T>,
    begin: usize,
    len: usize
}

struct SlidingWindow<T> {
    data: Vec<T>,
    begin: usize,
    len: usize
}

impl<T> SlidingWindow<T> {
    pub fn new(capacity: usize) -> SlidingWindow<T> {
        SlidingWindow{
            data: Vec::with_capacity(capacity),
            begin: 0,
            len: 0
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, elem: T) {
        if self.len < self.capacity() {
            self.len += 1;
        } else {
            self.begin = (self.begin + 1) % self.capacity()
        }
    }

    pub fn view_from(&self, index: usize) -> Option<View<T>> {
        if index < self.len {
        Some(View {data: &self.data, begin: self.begin + index, len: self.len})
        } else {
            None
        }
    }
}

impl<'a, T> View<'a, T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(self.begin + index)
    }
}
