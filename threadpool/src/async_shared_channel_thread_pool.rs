use std::{
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    thread::{self, JoinHandle},
};

use crate::{ThreadPool, Worker, WorkerMessage};

struct PullWorker {
    join_handler: JoinHandle<()>,
}

pub struct SharedChannelThreadPool {
    workers: Vec<PullWorker>,
    sender: Sender<WorkerMessage>,
}

impl PullWorker {
    fn new(tasks_receiver: Arc<Mutex<Receiver<WorkerMessage>>>) -> Self {
        let join_handler = thread::spawn(move || {
            loop {
                let receiver = tasks_receiver.lock().unwrap();
                let task = receiver.recv();
                drop(receiver);
                match task {
                    Ok(message) => match message {
                        WorkerMessage::SomeFun(fn_once) => {
                            fn_once();
                        }
                        WorkerMessage::Exit => {
                            break;
                        }
                    },
                    Err(_) => todo!(),
                }
            }
        });
        Self { join_handler }
    }
}

impl Worker for PullWorker {
    fn stop(self) {
        self.join_handler.join().unwrap();
    }
}

impl ThreadPool for SharedChannelThreadPool {
    fn new(thread_pool_size: usize) -> impl ThreadPool {
        let (tx, rx) = channel();
        let rx_wrapped = Arc::new(Mutex::new(rx));
        let workers = (0..thread_pool_size)
            .into_iter()
            .map(|_| PullWorker::new(rx_wrapped.clone()))
            .collect();
        Self {
            workers,
            sender: tx,
        }
    }

    fn execute_task(&self, fun: impl FnOnce() + Send + 'static) {
        self.sender
            .send(WorkerMessage::SomeFun(Box::new(fun)))
            .unwrap();
    }

    fn stop_threads(self) {
        self.workers.iter().for_each(|_| {
            self.sender.send(WorkerMessage::Exit).unwrap();
        });
        self.workers.into_iter().for_each(|w| {
            w.stop();
        });
    }
}
