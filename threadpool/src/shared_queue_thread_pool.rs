use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{ThreadPool, Worker, WorkerMessage};

type TasksQueueType = VecDeque<WorkerMessage>;

struct PullWorker {
    join_handler: JoinHandle<()>,
}

pub struct SharedQueueThreadPool {
    workers: Vec<PullWorker>,
    tasks_queue: Arc<Mutex<TasksQueueType>>,
}

impl PullWorker {
    fn new(tasks_queue: Arc<Mutex<TasksQueueType>>) -> Self {
        let join_handler = thread::spawn(move || {
            loop {
                let optional_task;
                {
                    optional_task = (*tasks_queue.lock().unwrap()).pop_front();
                }
                if let Some(message) = optional_task {
                    match message {
                        WorkerMessage::SomeFun(fn_once) => {
                            fn_once();
                        }
                        WorkerMessage::Exit => break,
                    }
                }
                thread::sleep(Duration::from_millis(10));
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

impl ThreadPool for SharedQueueThreadPool {
    fn new(thread_pool_size: usize) -> impl ThreadPool {
        let tasks = Arc::new(Mutex::new(VecDeque::new()));
        let workers = (0..thread_pool_size)
            .into_iter()
            .map(|_| PullWorker::new(tasks.clone()))
            .collect();
        Self {
            workers,
            tasks_queue: tasks,
        }
    }

    fn execute_task(&self, fun: impl FnOnce() + Send + 'static) {
        let mut mutex_guard = self.tasks_queue.lock().unwrap();
        (*mutex_guard).push_back(WorkerMessage::SomeFun(Box::new(fun)));
    }

    fn stop_threads(self) {
        self.workers.iter().for_each(|_| {
            let mut mutex_guard = self.tasks_queue.lock().unwrap();
            (*mutex_guard).push_back(WorkerMessage::Exit);
        });
        self.workers.into_iter().for_each(|w| w.stop());
    }
}
