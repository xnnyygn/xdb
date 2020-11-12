use std::sync::atomic::{AtomicPtr, Ordering};



pub trait Number {
    fn add(&self, delta: i32) -> Self;
    fn to_i32(&self) -> i32;
}

pub struct AtomicNumber<T: Number + PartialEq>(AtomicPtr<T>);

impl<T: Number + PartialEq> AtomicNumber<T> {
    #[allow(unused)]
    pub fn new(x: T) -> AtomicNumber<T> {
        AtomicNumber(AtomicPtr::new(Box::into_raw(Box::new(x))))
    }

    #[allow(unused)]
    pub fn load(&self) -> i32 {
        let x_ptr = self.0.load(Ordering::Relaxed);
        unsafe {
            (*x_ptr).to_i32()
        }
    }

    #[allow(unused)]
    pub fn store(&mut self, y: T) {
        let y_ptr = Box::into_raw(Box::new(y));
        self.0.store(y_ptr, Ordering::Release);
    }

//    #[allow(unused)]
//    pub fn compare_and_swap(&mut self, x: T, y: T) -> bool {
//        unsafe {
//            let y_ptr = Box::into_raw(Box::new(y));
//            loop {
//                let x_ptr = self.0.load(Ordering::Acquire);
//                if *x_ptr != x {
//                    drop(Box::from_raw(y_ptr));
//                    return false;
//                }
//                if self.0.compare_and_swap(x_ptr, y_ptr, Ordering::AcqRel) == x_ptr {
//                    drop(Box::from_raw(x_ptr));
//                    return true;
//                }
//            }
//        }
//    }

    pub fn add(&self, delta: i32) -> i32 {
        unsafe {
            let mut x_ptr = self.0.load(Ordering::Acquire);
            let mut y_ptr = Box::into_raw(Box::new((*x_ptr).add(delta)));
            loop {
                match self.0.compare_exchange_weak(x_ptr, y_ptr,
                                                   Ordering::Release, Ordering::Relaxed) {
                    Ok(_) => {
                        drop(Box::from_raw(x_ptr));
                        return (*y_ptr).to_i32();
                    },
                    Err(z_ptr) => {
                        x_ptr = z_ptr;
                        drop(Box::from_raw(y_ptr));
                        y_ptr = Box::into_raw(Box::new((*x_ptr).add(delta)));
                    },
                }
            }
        }
    }

    #[allow(unused)]
    pub fn increase(&self) -> i32 {
        self.add(1)
    }

    #[allow(unused)]
    pub fn decrease(&mut self) -> i32 {
        self.add(-1)
    }
}

impl<T: Number + PartialEq> Drop for AtomicNumber<T> {
    fn drop(&mut self) {
        let x_ptr = self.0.load(Ordering::Relaxed);
        unsafe {
            drop(Box::from_raw(x_ptr));
        }
    }
}