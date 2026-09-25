use std::{
    cell::RefCell,
    sync::mpsc::{Sender, channel},
    thread::{self, JoinHandle},
};

use crate::{ThreadPool, Worker};

enum WorkerMessageForWorkerWithRoundRobin {
    SomeFun(Box<dyn FnOnce() + Send + 'static>),
    Exit,
}

pub struct RoundRobinThreadPool {
    workers: Vec<WorkerForRoundRobin>,
    thread_id_to_assign: RefCell<usize>,
}

impl ThreadPool for RoundRobinThreadPool {
    fn new(thread_pool_size: usize) -> impl ThreadPool {
        let workers_vector = (0..thread_pool_size)
            .into_iter()
            .map(|_| WorkerForRoundRobin::new())
            .collect();
        Self {
            workers: workers_vector,
            thread_id_to_assign: RefCell::new(0usize),
        }
    }

    fn execute_task(&self, fun: impl FnOnce() + Send + 'static) {
        self.workers
            .get(*self.thread_id_to_assign.borrow())
            .unwrap()
            .execute(fun);
        *self.thread_id_to_assign.borrow_mut() += 1usize;
        *self.thread_id_to_assign.borrow_mut() %= self.workers.len();
    }

    fn stop_threads(self) {
        for worker in self.workers {
            worker.stop();
        }
    }
}

pub struct WorkerForRoundRobin {
    join_handler: JoinHandle<()>,
    sender: Sender<WorkerMessageForWorkerWithRoundRobin>,
}

impl WorkerForRoundRobin {
    fn new() -> Self {
        let (tx, rx) = channel();
        let join_handle = thread::spawn(move || {
            while let Ok(message) = rx.recv() {
                match message {
                    WorkerMessageForWorkerWithRoundRobin::SomeFun(fn_once) => {
                        fn_once();
                    }
                    WorkerMessageForWorkerWithRoundRobin::Exit => {
                        break;
                    }
                };
            }
        });
        Self {
            join_handler: join_handle,
            sender: tx,
        }
    }

    fn execute(&self, fun: impl FnOnce() + Send + 'static) {
        let sender = self.sender.clone();
        sender
            .send(WorkerMessageForWorkerWithRoundRobin::SomeFun(Box::new(fun)))
            .unwrap();
    }
}

impl Worker for WorkerForRoundRobin {
    fn stop(self) {
        self.sender
            .send(WorkerMessageForWorkerWithRoundRobin::Exit)
            .unwrap();
        self.join_handler.join().unwrap();
    }
}

impl Default for WorkerForRoundRobin {
    fn default() -> Self {
        Self::new()
    }
}
