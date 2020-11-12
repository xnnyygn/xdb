use crate::atomic_number::{AtomicNumber, Number};

struct Id(u32);

impl Number for Id {
    fn add(&self, delta: i32) -> Id {
        Id(self.0 + delta as u32)
    }

    fn to_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.0 == other.0
    }
}

impl Drop for Id {
    fn drop(&mut self) {
        println!("drop id {}", self.0);
    }
}

pub struct ThreadId {
    last_id: AtomicNumber<Id>,
}

impl ThreadId {
    pub fn new() -> ThreadId {
        let last_id = AtomicNumber::new(Id(0));
        ThreadId { last_id }
    }

    pub fn generate(&self) -> u32 {
        self.last_id.increase() as u32
    }
}

unsafe impl Send for ThreadId {}

unsafe impl Sync for ThreadId {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use super::*;

    #[test]
    fn generate() {
        let thread_id = Arc::new(ThreadId::new());
        let mut handles = Vec::with_capacity(4);
        for i in 0..4 {
            let tid = thread_id.clone();
            let handle = thread::spawn(move || {
                for _ in 0..9 {
                    println!("thread {}, {}", i, tid.generate());
                }
            });
            handles.push(handle);
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}