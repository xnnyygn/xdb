use std::sync::atomic::{AtomicPtr, Ordering};

pub struct AtomicI32(AtomicPtr<i32>);

impl AtomicI32 {
    #[allow(unused)]
    pub fn new(x: i32) -> AtomicI32 {
        AtomicI32(AtomicPtr::new(Box::into_raw(Box::new(x))))
    }

    #[allow(unused)]
    pub fn load(&self) -> i32 {
        let x_ptr = self.0.load(Ordering::Relaxed);
        unsafe {
            *x_ptr
        }
    }

    #[allow(unused)]
    pub fn store(&mut self, y: i32) {
        let y_ptr = Box::into_raw(Box::new(y));
        self.0.store(y_ptr, Ordering::Release);
    }

    #[allow(unused)]
    pub fn compare_and_swap(&mut self, x: i32, y: i32) -> bool {
        unsafe {
            let y_ptr = Box::into_raw(Box::new(y));
            loop {
                let x_ptr = self.0.load(Ordering::Relaxed);
                if *x_ptr != x {
                    drop(Box::from_raw(y_ptr));
                    return false;
                }
                if self.0.compare_and_swap(x_ptr, y_ptr, Ordering::AcqRel) == x_ptr {
                    drop(Box::from_raw(x_ptr));
                    return true;
                }
            }
        }
    }

    pub fn add(&mut self, delta: i32) -> i32 {
        unsafe {
            loop {
                let x_ptr = self.0.load(Ordering::Relaxed);
                let y_ptr = Box::into_raw(Box::new((*x_ptr) + delta));
                if self.0.compare_and_swap(x_ptr, y_ptr, Ordering::AcqRel) == x_ptr {
                    drop(Box::from_raw(x_ptr));
                    return *y_ptr;
                }
                drop(Box::from_raw(y_ptr));
            }
        }
    }

    #[allow(unused)]
    pub fn increase(&mut self) -> i32 {
        self.add(1)
    }

    #[allow(unused)]
    pub fn decrease(&mut self) -> i32 {
        self.add(-1)
    }
}

impl Drop for AtomicI32 {
    fn drop(&mut self) {
        let x_ptr = self.0.load(Ordering::Relaxed);
        unsafe {
            drop(Box::from_raw(x_ptr));
        }
    }
}