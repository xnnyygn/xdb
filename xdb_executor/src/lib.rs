use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F> FnBox for F
    where F: FnOnce() {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn run(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let h = thread::spawn(move || {
            loop {
                let receiver = receiver.lock().unwrap();
                let message = receiver.recv().unwrap();
                drop(receiver);
                match message {
                    Message::Job(action) => action.call_box(),
                    Message::Terminate => break,
                }
            }
        });
        Worker { handle: Some(h) }
    }
}

enum Message {
    Job(Box<FnBox + Send + 'static>),
    Terminate,
}

impl ThreadPool {
    /// n must > 0
    pub fn new(n: usize) -> ThreadPool {
        assert!(n > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(n);
        for _ in 0..n {
            workers.push(Worker::run(receiver.clone()));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        self.sender.send(Message::Job(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(h) = worker.handle.take() {
                h.join().unwrap_err();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
