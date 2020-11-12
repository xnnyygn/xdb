use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::thread::ThreadId;
use std::cell::Cell;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

pub struct RecursiveMutexGuard<'a> {
    mutex: &'a RecursiveMutex,
}

impl<'a> Drop for RecursiveMutexGuard<'a> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

pub struct RecursiveMutex {
    underlying: Mutex<()>,
    thread_id: Cell<ThreadId>,
    depth: Cell<usize>,
}

impl RecursiveMutex {
    pub fn new() -> RecursiveMutex {
        RecursiveMutex {
            underlying: Mutex::new(()),
            thread_id: Cell::new(ThreadId(0)),
            depth: Cell::new(0),
        }
    }

    pub fn lock(&self) -> RecursiveMutexGuard {
        let me = thread::current().id();
        if self.thread_id.get() == me {
            let d = self.depth.get();
            self.depth.set(d + 1);
        } else {
            self.thread_id.set(me);
            self.depth.set(1);
        }
        RecursiveMutexGuard { mutex: &self }
    }

    pub fn unlock(&self) {
        let d = self.depth.get();
        self.depth.set(d - 1);
        if d > 1 {
            return;
        }

    }
}