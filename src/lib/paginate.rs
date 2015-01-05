pub trait PaginatableIterator<T> {
    fn paginate(self, page: uint) -> Paginate<Self>;
}

impl<T, U : Iterator<Item = T>> PaginatableIterator<T> for U {
    fn paginate(self, page: uint) -> Paginate<U> {
        Paginate {
            iter: self,
            page: page,
        }
    }
}

struct Paginate<I> {
    iter: I,
    page: uint,
}

impl<E, I : Iterator<Item = E>> Iterator for Paginate<I> {
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Vec<E>> {
        let mut r = Vec::new();
        for _ in range(0, self.page) {
            match self.iter.next() {
                Some(next) => r.push(next),
                None if r.is_empty() => return None,
                None                 => return Some(r),
            }
        }
        Some(r)
    }
}

