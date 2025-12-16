use std::sync::{
    mpsc,
    Arc,
    Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
    cancel_flag: Arc<AtomicBool>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(rx));

        let cancel_flag = Arc::new(AtomicBool::new(false));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let rcv = Arc::clone(&receiver);
            let cancelled = Arc::clone(&cancel_flag);

            let handle = thread::spawn(move || loop {
                if cancelled.load(Ordering::SeqCst) {
                    break;
                }

                let message = {
                    let rx_lock = rcv.lock().unwrap();
                    rx_lock.recv()
                };

                match message {
                    Ok(Message::NewJob(job)) => {
                        job();
                    }
                    Ok(Message::Terminate) | Err(_) => {
                        break;
                    }
                }
            });

            workers.push(Worker {
                id,
                handle: Some(handle),
            });
        }

        ThreadPool {
            workers,
            sender: Some(tx),
            cancel_flag,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if self.cancel_flag.load(Ordering::SeqCst) {
            return;
        }

        if let Some(sender) = &self.sender {
            let _ = sender.send(Message::NewJob(Box::new(f)));
        }
    }

    /// Allow external code to request cancellation (not strictly required to use)
    pub fn cancel_all(&self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            for _ in &self.workers {
                let _ = sender.send(Message::Terminate);
            }
        }

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn thread_pool_runs_all_jobs() {
        let pool = ThreadPool::new(4);

        let counter = Arc::new(Mutex::new(0));
        let num_jobs = 20;

        for _ in 0..num_jobs {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut n = counter.lock().unwrap();
                *n += 1;
            });
        }

        // Give the workers a little time to finish
        std::thread::sleep(Duration::from_millis(100));

        let n = counter.lock().unwrap();
        assert_eq!(*n, num_jobs);
    }

    #[test]
    fn thread_pool_handles_zero_jobs() {
        let _pool = ThreadPool::new(4);
        // If this test finishes without panic, it's a pass.
        // (Just checks that creating + dropping a pool with no jobs is safe.)
    }
}
