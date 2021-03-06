use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self.value (!snyc)
        // SAFETY: we know we're not invalidating any references, as we never give any out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    // #[test]
    // fn bac() {
    //     use std::sync::Arc;
    //     let x = Arc::new(42);
    //     let x1 = Arc::clone(&x);
    //     std::thread::spawn(|| {
    //         x1.set(43);
    //     });
    //     let x2 = Arc::clone(&x);
    //     std::thread::spawn(|| {
    //         x2.set(44);
    //     });
    // }


    // #[test]
    // fn bac2() {
    //     let x = Cell::new(vec![42]);
    //     let first = &x.get()[0];
    //     x.set(vec![]);
    //     eprintln!("{}", first);

    // }
}
